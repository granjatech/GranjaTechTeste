use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use sqlx::PgPool;

use crate::config::Config;
use crate::dto::auth::*;
use crate::errors::AppError;
use crate::middleware::jwt::Claims;
use crate::models::usuario::UsuarioComPerfil;
use crate::services::auditoria_service::AuditoriaService;

pub struct AuthService;

impl AuthService {
    /// POST /api/auth/login -- verifica credenciais e retorna JWT
    pub async fn login(
        pool: &PgPool,
        dto: &LoginDto,
        config: &Config,
    ) -> Result<LoginResponseDto, AppError> {
        // Busca usuario com perfil por email
        let usuario: UsuarioComPerfil = sqlx::query_as::<_, UsuarioComPerfil>(
            r#"SELECT u."Id" as id, u."Codigo" as codigo, u."Nome" as nome,
                      u."Email" as email, u."SenhaHash" as senha_hash,
                      u."PerfilId" as perfil_id, p."Nome" as perfil_nome
               FROM "Usuarios" u
               INNER JOIN "Perfis" p ON p."Id" = u."PerfilId"
               WHERE u."Email" = $1"#,
        )
        .bind(&dto.email)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::Unauthorized("Email ou senha invalidos".into()))?;

        // Verifica BCrypt hash (compativel com $2a$ do .NET)
        let senha_valida = bcrypt::verify(&dto.senha, &usuario.senha_hash)
            .map_err(|e| AppError::Internal(format!("Erro ao verificar senha: {}", e)))?;

        if !senha_valida {
            return Err(AppError::Unauthorized("Email ou senha invalidos".into()));
        }

        // Gera JWT
        let token = Self::gerar_jwt(usuario.id, &usuario.email, &usuario.perfil_nome, config)?;

        Ok(LoginResponseDto { token })
    }

    /// POST /api/auth/registrar -- cria novo usuario
    pub async fn registrar(
        pool: &PgPool,
        dto: &RegisterDto,
        config: &Config,
    ) -> Result<UserDto, AppError> {
        // Verifica email duplicado
        let existe: bool = sqlx::query_scalar(
            r#"SELECT EXISTS(SELECT 1 FROM "Usuarios" WHERE "Email" = $1)"#,
        )
        .bind(&dto.email)
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

        if existe {
            return Err(AppError::BadRequest("Email ja esta em uso".into()));
        }

        // Gera codigo sequencial (USR-001, USR-002...)
        let ultimo_id: Option<i32> = sqlx::query_scalar(
            r#"SELECT MAX("Id") FROM "Usuarios""#,
        )
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

        let novo_codigo = format!("USR-{:03}", ultimo_id.unwrap_or(0) + 1);

        // Hash da senha com custo 10 (compativel com .NET BCrypt)
        let senha_hash = bcrypt::hash(&dto.senha, 10)
            .map_err(|e| AppError::Internal(format!("Erro ao gerar hash: {}", e)))?;

        // Busca nome do perfil
        let perfil_nome: String = sqlx::query_scalar(
            r#"SELECT "Nome" FROM "Perfis" WHERE "Id" = $1"#,
        )
        .bind(dto.perfil_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::BadRequest("Perfil nao encontrado".into()))?;

        // INSERT usuario
        let user_id: i32 = sqlx::query_scalar(
            r#"INSERT INTO "Usuarios" ("Codigo", "Nome", "Email", "SenhaHash", "PerfilId")
               VALUES ($1, $2, $3, $4, $5)
               RETURNING "Id""#,
        )
        .bind(&novo_codigo)
        .bind(&dto.nome)
        .bind(&dto.email)
        .bind(&senha_hash)
        .bind(dto.perfil_id)
        .fetch_one(pool)
        .await?;

        // Registra auditoria
        AuditoriaService::registrar_log(
            pool,
            user_id,
            &dto.email,
            "CRIACAO_USUARIO",
            &format!(
                "Utilizador '{}' (Codigo: {}) foi criado.",
                dto.email, novo_codigo
            ),
        )
        .await?;

        // Associacao Financeiro -> Produtores (perfil_id == 3)
        if dto.perfil_id == 3 {
            if let Some(ref produtores_ids) = dto.produtores_ids {
                for produtor_id in produtores_ids {
                    sqlx::query(
                        r#"INSERT INTO "FinanceiroProdutor" ("FinanceiroId", "ProdutorId")
                           VALUES ($1, $2)"#,
                    )
                    .bind(user_id)
                    .bind(produtor_id)
                    .execute(pool)
                    .await?;
                }

                AuditoriaService::registrar_log(
                    pool,
                    user_id,
                    &dto.email,
                    "ASSOCIACAO_USUARIO",
                    &format!(
                        "Utilizador Financeiro '{}' associado aos Produtores IDs: {:?}.",
                        dto.email, produtores_ids
                    ),
                )
                .await?;
            }
        }

        Ok(UserDto {
            id: user_id,
            codigo: novo_codigo,
            nome: dto.nome.clone(),
            email: dto.email.clone(),
            perfil_id: dto.perfil_id,
            perfil_nome,
        })
    }

    /// GET /api/auth/usuarios -- lista todos os usuarios (admin only)
    pub async fn get_all(pool: &PgPool) -> Result<Vec<UserDto>, AppError> {
        let rows = sqlx::query_as::<_, UsuarioComPerfil>(
            r#"SELECT u."Id" as id, u."Codigo" as codigo, u."Nome" as nome,
                      u."Email" as email, u."SenhaHash" as senha_hash,
                      u."PerfilId" as perfil_id, p."Nome" as perfil_nome
               FROM "Usuarios" u
               INNER JOIN "Perfis" p ON p."Id" = u."PerfilId"
               ORDER BY u."Id""#,
        )
        .fetch_all(pool)
        .await?;

        let usuarios = rows
            .into_iter()
            .map(|u| UserDto {
                id: u.id,
                codigo: u.codigo,
                nome: u.nome,
                email: u.email,
                perfil_id: u.perfil_id,
                perfil_nome: u.perfil_nome,
            })
            .collect();

        Ok(usuarios)
    }

    /// GET /api/auth/usuarios/{id} -- detalhes do usuario (admin only)
    pub async fn get_by_id(pool: &PgPool, id: i32) -> Result<UserDetailDto, AppError> {
        let usuario = sqlx::query_as::<_, UsuarioComPerfil>(
            r#"SELECT u."Id" as id, u."Codigo" as codigo, u."Nome" as nome,
                      u."Email" as email, u."SenhaHash" as senha_hash,
                      u."PerfilId" as perfil_id, p."Nome" as perfil_nome
               FROM "Usuarios" u
               INNER JOIN "Perfis" p ON p."Id" = u."PerfilId"
               WHERE u."Id" = $1"#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Usuario com ID {} nao encontrado", id)))?;

        // Busca produtores associados (para Financeiro)
        let produtores_ids: Vec<i32> = sqlx::query_scalar(
            r#"SELECT "ProdutorId" FROM "FinanceiroProdutor" WHERE "FinanceiroId" = $1"#,
        )
        .bind(id)
        .fetch_all(pool)
        .await?;

        Ok(UserDetailDto {
            id: usuario.id,
            codigo: usuario.codigo,
            nome: usuario.nome,
            email: usuario.email,
            perfil_id: usuario.perfil_id,
            perfil_nome: usuario.perfil_nome,
            produtores_ids,
        })
    }

    /// PUT /api/auth/usuarios/{id} -- atualiza usuario (admin only)
    pub async fn update(
        pool: &PgPool,
        id: i32,
        dto: &UpdateUserDto,
        admin_id: i32,
        admin_email: &str,
    ) -> Result<(), AppError> {
        // Verifica se usuario existe
        let existe: bool = sqlx::query_scalar(
            r#"SELECT EXISTS(SELECT 1 FROM "Usuarios" WHERE "Id" = $1)"#,
        )
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

        if !existe {
            return Err(AppError::NotFound(format!(
                "Usuario com ID {} nao encontrado",
                id
            )));
        }

        // Verifica email duplicado (excluindo o proprio usuario)
        let email_duplicado: bool = sqlx::query_scalar(
            r#"SELECT EXISTS(SELECT 1 FROM "Usuarios" WHERE "Email" = $1 AND "Id" != $2)"#,
        )
        .bind(&dto.email)
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

        if email_duplicado {
            return Err(AppError::BadRequest("Email ja esta em uso".into()));
        }

        // Atualiza campos basicos
        if let Some(ref senha) = dto.senha {
            let senha_hash = bcrypt::hash(senha, 10)
                .map_err(|e| AppError::Internal(format!("Erro ao gerar hash: {}", e)))?;

            sqlx::query(
                r#"UPDATE "Usuarios"
                   SET "Nome" = $1, "Email" = $2, "PerfilId" = $3, "SenhaHash" = $4
                   WHERE "Id" = $5"#,
            )
            .bind(&dto.nome)
            .bind(&dto.email)
            .bind(dto.perfil_id)
            .bind(&senha_hash)
            .bind(id)
            .execute(pool)
            .await?;
        } else {
            sqlx::query(
                r#"UPDATE "Usuarios"
                   SET "Nome" = $1, "Email" = $2, "PerfilId" = $3
                   WHERE "Id" = $4"#,
            )
            .bind(&dto.nome)
            .bind(&dto.email)
            .bind(dto.perfil_id)
            .bind(id)
            .execute(pool)
            .await?;
        }

        // Gerencia associacoes FinanceiroProdutor
        // Remove associacoes existentes
        sqlx::query(r#"DELETE FROM "FinanceiroProdutor" WHERE "FinanceiroId" = $1"#)
            .bind(id)
            .execute(pool)
            .await?;

        // Insere novas associacoes se perfil Financeiro
        if dto.perfil_id == 3 {
            if let Some(ref produtores_ids) = dto.produtores_ids {
                for produtor_id in produtores_ids {
                    sqlx::query(
                        r#"INSERT INTO "FinanceiroProdutor" ("FinanceiroId", "ProdutorId")
                           VALUES ($1, $2)"#,
                    )
                    .bind(id)
                    .bind(produtor_id)
                    .execute(pool)
                    .await?;
                }
            }
        }

        // Auditoria
        AuditoriaService::registrar_log(
            pool,
            admin_id,
            admin_email,
            "ATUALIZACAO_USUARIO",
            &format!("Utilizador '{}' (ID: {}) foi atualizado.", dto.email, id),
        )
        .await?;

        Ok(())
    }

    /// DELETE /api/auth/usuarios/{id} -- deleta usuario (admin only)
    pub async fn delete(
        pool: &PgPool,
        id: i32,
        admin_id: i32,
        admin_email: &str,
    ) -> Result<(), AppError> {
        // Verifica se usuario existe e busca dados
        let usuario = sqlx::query_as::<_, UsuarioComPerfil>(
            r#"SELECT u."Id" as id, u."Codigo" as codigo, u."Nome" as nome,
                      u."Email" as email, u."SenhaHash" as senha_hash,
                      u."PerfilId" as perfil_id, p."Nome" as perfil_nome
               FROM "Usuarios" u
               INNER JOIN "Perfis" p ON p."Id" = u."PerfilId"
               WHERE u."Id" = $1"#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Usuario com ID {} nao encontrado", id)))?;

        // Se produtor, verifica dependencias
        if usuario.perfil_nome == "Produtor" {
            let tem_granjas: bool = sqlx::query_scalar(
                r#"SELECT EXISTS(SELECT 1 FROM "Granjas" WHERE "UsuarioId" = $1)"#,
            )
            .bind(id)
            .fetch_one(pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

            if tem_granjas {
                return Err(AppError::BadRequest(
                    "Este produtor possui granjas associadas e nao pode ser excluido.".into(),
                ));
            }

            let tem_associacoes: bool = sqlx::query_scalar(
                r#"SELECT EXISTS(SELECT 1 FROM "FinanceiroProdutor" WHERE "ProdutorId" = $1)"#,
            )
            .bind(id)
            .fetch_one(pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

            if tem_associacoes {
                return Err(AppError::BadRequest(
                    "Este produtor esta associado a um ou mais utilizadores financeiros e nao pode ser excluido.".into(),
                ));
            }
        }

        // Remove associacoes FinanceiroProdutor
        sqlx::query(r#"DELETE FROM "FinanceiroProdutor" WHERE "FinanceiroId" = $1"#)
            .bind(id)
            .execute(pool)
            .await?;

        // Remove usuario
        sqlx::query(r#"DELETE FROM "Usuarios" WHERE "Id" = $1"#)
            .bind(id)
            .execute(pool)
            .await?;

        // Auditoria
        AuditoriaService::registrar_log(
            pool,
            admin_id,
            admin_email,
            "DELECAO_USUARIO",
            &format!(
                "Utilizador '{}' (ID: {}) foi deletado.",
                usuario.nome, id
            ),
        )
        .await?;

        Ok(())
    }

    /// Gera token JWT com claims compativeis com o formato .NET
    fn gerar_jwt(
        usuario_id: i32,
        email: &str,
        perfil_nome: &str,
        config: &Config,
    ) -> Result<String, AppError> {
        let agora = chrono::Utc::now();
        let expiracao = agora + chrono::Duration::hours(8);

        let claims = Claims {
            nameid: usuario_id.to_string(),
            email: email.to_string(),
            role: perfil_nome.to_string(),
            exp: expiracao.timestamp() as usize,
            iss: config.jwt_issuer.clone(),
            aud: config.jwt_audience.clone(),
        };

        let header = Header::new(Algorithm::HS256);
        let key = EncodingKey::from_secret(config.jwt_key.as_bytes());

        encode(&header, &claims, &key)
            .map_err(|e| AppError::Internal(format!("Erro ao gerar token JWT: {}", e)))
    }
}
