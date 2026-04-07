use sqlx::PgPool;

use crate::dto::granja::*;
use crate::errors::AppError;
use crate::models::granja::Granja;
use crate::services::auditoria_service::AuditoriaService;

pub struct GranjaService;

impl GranjaService {
    /// GET /api/granjas -- lista granjas com filtro por role
    /// Administrador: todas, Produtor: proprias, Financeiro: via FinanceiroProdutor
    pub async fn get_all(
        pool: &PgPool,
        user_id: i32,
        user_role: &str,
    ) -> Result<Vec<Granja>, AppError> {
        let granjas = match user_role {
            "Administrador" => {
                sqlx::query_as::<_, Granja>(
                    r#"SELECT "Id" as id, "Codigo" as codigo, "Nome" as nome,
                              "Localizacao" as localizacao, "UsuarioId" as usuario_id
                       FROM "Granjas"
                       ORDER BY "Id""#,
                )
                .fetch_all(pool)
                .await?
            }
            "Produtor" => {
                sqlx::query_as::<_, Granja>(
                    r#"SELECT "Id" as id, "Codigo" as codigo, "Nome" as nome,
                              "Localizacao" as localizacao, "UsuarioId" as usuario_id
                       FROM "Granjas"
                       WHERE "UsuarioId" = $1
                       ORDER BY "Id""#,
                )
                .bind(user_id)
                .fetch_all(pool)
                .await?
            }
            "Financeiro" => {
                sqlx::query_as::<_, Granja>(
                    r#"SELECT g."Id" as id, g."Codigo" as codigo, g."Nome" as nome,
                              g."Localizacao" as localizacao, g."UsuarioId" as usuario_id
                       FROM "Granjas" g
                       INNER JOIN "FinanceiroProdutor" fp ON fp."ProdutorId" = g."UsuarioId"
                       WHERE fp."FinanceiroId" = $1
                       ORDER BY g."Id""#,
                )
                .bind(user_id)
                .fetch_all(pool)
                .await?
            }
            _ => Vec::new(),
        };

        Ok(granjas)
    }

    /// GET /api/granjas/{id} -- busca granja por ID com verificacao de acesso
    pub async fn get_by_id(
        pool: &PgPool,
        id: i32,
        user_id: i32,
        user_role: &str,
    ) -> Result<Granja, AppError> {
        let granja = sqlx::query_as::<_, Granja>(
            r#"SELECT "Id" as id, "Codigo" as codigo, "Nome" as nome,
                      "Localizacao" as localizacao, "UsuarioId" as usuario_id
               FROM "Granjas"
               WHERE "Id" = $1"#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Granja com ID {} nao encontrada", id)))?;

        // Verificacao de acesso por role
        match user_role {
            "Administrador" => Ok(granja),
            "Produtor" => {
                if granja.usuario_id == user_id {
                    Ok(granja)
                } else {
                    Err(AppError::Forbidden("Acesso negado a esta granja".into()))
                }
            }
            "Financeiro" => {
                let tem_acesso: bool = sqlx::query_scalar(
                    r#"SELECT EXISTS(
                        SELECT 1 FROM "FinanceiroProdutor"
                        WHERE "FinanceiroId" = $1 AND "ProdutorId" = $2
                    )"#,
                )
                .bind(user_id)
                .bind(granja.usuario_id)
                .fetch_one(pool)
                .await
                .map_err(|e| AppError::Internal(e.to_string()))?;

                if tem_acesso {
                    Ok(granja)
                } else {
                    Err(AppError::Forbidden("Acesso negado a esta granja".into()))
                }
            }
            _ => Err(AppError::Forbidden("Perfil nao reconhecido".into())),
        }
    }

    /// POST /api/granjas -- cria nova granja (Financeiro bloqueado)
    pub async fn create(
        pool: &PgPool,
        dto: &CreateGranjaDto,
        user_id: i32,
        user_role: &str,
        user_email: &str,
    ) -> Result<Granja, AppError> {
        // Bloqueia Financeiro
        if user_role == "Financeiro" {
            return Err(AppError::Forbidden(
                "Usuarios do perfil Financeiro nao podem criar granjas.".into(),
            ));
        }

        // Gera codigo sequencial (GRJ-001, GRJ-002...)
        let ultimo_id: Option<i32> =
            sqlx::query_scalar(r#"SELECT MAX("Id") FROM "Granjas""#)
                .fetch_one(pool)
                .await
                .map_err(|e| AppError::Internal(e.to_string()))?;

        let novo_codigo = format!("GRJ-{:03}", ultimo_id.unwrap_or(0) + 1);

        // Define UsuarioId: Admin pode especificar, Produtor usa proprio ID
        let owner_id = if user_role == "Administrador" {
            dto.usuario_id.unwrap_or(user_id)
        } else {
            user_id
        };

        // INSERT granja
        let granja = sqlx::query_as::<_, Granja>(
            r#"INSERT INTO "Granjas" ("Codigo", "Nome", "Localizacao", "UsuarioId")
               VALUES ($1, $2, $3, $4)
               RETURNING "Id" as id, "Codigo" as codigo, "Nome" as nome,
                         "Localizacao" as localizacao, "UsuarioId" as usuario_id"#,
        )
        .bind(&novo_codigo)
        .bind(&dto.nome)
        .bind(&dto.localizacao)
        .bind(owner_id)
        .fetch_one(pool)
        .await?;

        // Auditoria
        AuditoriaService::registrar_log(
            pool,
            user_id,
            user_email,
            "CRIACAO_GRANJA",
            &format!(
                "Granja '{}' (Codigo: {}) criada.",
                granja.nome, granja.codigo
            ),
        )
        .await?;

        Ok(granja)
    }

    /// PUT /api/granjas/{id} -- atualiza granja (Financeiro bloqueado)
    pub async fn update(
        pool: &PgPool,
        id: i32,
        dto: &UpdateGranjaDto,
        user_id: i32,
        user_role: &str,
        user_email: &str,
    ) -> Result<Granja, AppError> {
        // Bloqueia Financeiro
        if user_role == "Financeiro" {
            return Err(AppError::Forbidden(
                "Usuarios do perfil Financeiro nao podem editar granjas.".into(),
            ));
        }

        // Verifica se granja existe e acesso
        let granja_existente = Self::get_by_id(pool, id, user_id, user_role).await?;

        // Admin pode reatribuir owner, Produtor mantem
        let owner_id = if user_role == "Administrador" {
            dto.usuario_id
        } else {
            granja_existente.usuario_id
        };

        // UPDATE
        let granja = sqlx::query_as::<_, Granja>(
            r#"UPDATE "Granjas"
               SET "Nome" = $1, "Localizacao" = $2, "UsuarioId" = $3
               WHERE "Id" = $4
               RETURNING "Id" as id, "Codigo" as codigo, "Nome" as nome,
                         "Localizacao" as localizacao, "UsuarioId" as usuario_id"#,
        )
        .bind(&dto.nome)
        .bind(&dto.localizacao)
        .bind(owner_id)
        .bind(id)
        .fetch_one(pool)
        .await?;

        // Auditoria
        AuditoriaService::registrar_log(
            pool,
            user_id,
            user_email,
            "ATUALIZACAO_GRANJA",
            &format!("Granja '{}' (ID: {}) atualizada.", granja.nome, id),
        )
        .await?;

        Ok(granja)
    }

    /// DELETE /api/granjas/{id} -- deleta granja (Financeiro bloqueado)
    pub async fn delete(
        pool: &PgPool,
        id: i32,
        user_id: i32,
        user_role: &str,
        user_email: &str,
    ) -> Result<(), AppError> {
        // Bloqueia Financeiro
        if user_role == "Financeiro" {
            return Err(AppError::Forbidden(
                "Usuarios do perfil Financeiro nao podem deletar granjas.".into(),
            ));
        }

        // Verifica se granja existe e acesso
        let granja = Self::get_by_id(pool, id, user_id, user_role).await?;

        // DELETE
        sqlx::query(r#"DELETE FROM "Granjas" WHERE "Id" = $1"#)
            .bind(id)
            .execute(pool)
            .await?;

        // Auditoria
        AuditoriaService::registrar_log(
            pool,
            user_id,
            user_email,
            "DELECAO_GRANJA",
            &format!("Granja '{}' (ID: {}) deletada.", granja.nome, id),
        )
        .await?;

        Ok(())
    }
}
