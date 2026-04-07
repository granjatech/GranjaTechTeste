use sqlx::PgPool;

use crate::dto::profile::*;
use crate::errors::AppError;
use crate::services::auditoria_service::AuditoriaService;

pub struct ProfileService;

/// Helper row para query de perfil com JOIN
#[derive(Debug, sqlx::FromRow)]
struct ProfileRow {
    nome: String,
    email: String,
    perfil_nome: String,
}

/// Helper row para buscar senha_hash
#[derive(Debug, sqlx::FromRow)]
struct SenhaRow {
    senha_hash: String,
}

impl ProfileService {
    /// Retorna detalhes do perfil do usuario logado, incluindo associados
    pub async fn get_profile(
        pool: &PgPool,
        user_id: i32,
    ) -> Result<ProfileDetailDto, AppError> {
        let row = sqlx::query_as::<_, ProfileRow>(
            r#"SELECT u."Nome" as nome, u."Email" as email, p."Nome" as perfil_nome
               FROM "Usuarios" u
               INNER JOIN "Perfis" p ON u."PerfilId" = p."Id"
               WHERE u."Id" = $1"#,
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Usuario nao encontrado.".into()))?;

        // Busca associados baseado no perfil
        let associados = match row.perfil_nome.as_str() {
            "Financeiro" => {
                // Financeiro ve seus produtores associados
                sqlx::query_scalar::<_, String>(
                    r#"SELECT u."Nome"
                       FROM "Usuarios" u
                       INNER JOIN "FinanceiroProdutor" fp ON fp."ProdutorId" = u."Id"
                       WHERE fp."FinanceiroId" = $1"#,
                )
                .bind(user_id)
                .fetch_all(pool)
                .await
                .unwrap_or_default()
            }
            "Produtor" => {
                // Produtor ve seus financeiros associados
                sqlx::query_scalar::<_, String>(
                    r#"SELECT u."Nome"
                       FROM "Usuarios" u
                       INNER JOIN "FinanceiroProdutor" fp ON fp."FinanceiroId" = u."Id"
                       WHERE fp."ProdutorId" = $1"#,
                )
                .bind(user_id)
                .fetch_all(pool)
                .await
                .unwrap_or_default()
            }
            _ => Vec::new(), // Admin nao tem associados
        };

        Ok(ProfileDetailDto {
            nome: row.nome,
            email: row.email,
            perfil_nome: row.perfil_nome,
            associados,
        })
    }

    /// Atualiza nome e email do perfil do usuario logado
    pub async fn update_profile(
        pool: &PgPool,
        user_id: i32,
        dto: &UpdateProfileDto,
        user_email: &str,
    ) -> Result<(), AppError> {
        // Verifica unicidade do email
        let existe: bool = sqlx::query_scalar(
            r#"SELECT EXISTS(SELECT 1 FROM "Usuarios" WHERE "Email" = $1 AND "Id" != $2)"#,
        )
        .bind(&dto.email)
        .bind(user_id)
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

        if existe {
            return Err(AppError::BadRequest(
                "Email ja esta em uso por outro usuario.".into(),
            ));
        }

        sqlx::query(r#"UPDATE "Usuarios" SET "Nome" = $1, "Email" = $2 WHERE "Id" = $3"#)
            .bind(&dto.nome)
            .bind(&dto.email)
            .bind(user_id)
            .execute(pool)
            .await?;

        AuditoriaService::registrar_log(
            pool,
            user_id,
            user_email,
            "ATUALIZACAO_PERFIL",
            &format!("Perfil do usuario (ID: {}) atualizado.", user_id),
        )
        .await?;

        Ok(())
    }

    /// Altera senha do usuario logado, verificando senha atual via bcrypt
    pub async fn change_password(
        pool: &PgPool,
        user_id: i32,
        old_password: &str,
        new_password: &str,
        user_email: &str,
    ) -> Result<(), AppError> {
        // Busca hash atual
        let row = sqlx::query_as::<_, SenhaRow>(
            r#"SELECT "SenhaHash" as senha_hash FROM "Usuarios" WHERE "Id" = $1"#,
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Usuario nao encontrado.".into()))?;

        // Verifica senha atual com bcrypt
        let senha_valida = bcrypt::verify(old_password, &row.senha_hash)
            .map_err(|e| AppError::Internal(format!("Erro ao verificar senha: {}", e)))?;

        if !senha_valida {
            return Err(AppError::BadRequest("Senha atual incorreta.".into()));
        }

        // Hash nova senha com custo 12 (compatibilidade .NET BCrypt)
        let nova_hash = bcrypt::hash(new_password, 12)
            .map_err(|e| AppError::Internal(format!("Erro ao gerar hash: {}", e)))?;

        sqlx::query(r#"UPDATE "Usuarios" SET "SenhaHash" = $1 WHERE "Id" = $2"#)
            .bind(&nova_hash)
            .bind(user_id)
            .execute(pool)
            .await?;

        AuditoriaService::registrar_log(
            pool,
            user_id,
            user_email,
            "TROCA_SENHA",
            &format!("Senha do usuario (ID: {}) alterada.", user_id),
        )
        .await?;

        Ok(())
    }
}
