use chrono::{Duration, Utc};
use rust_decimal::Decimal;
use sqlx::PgPool;

use crate::dto::financeiro::*;
use crate::errors::AppError;
use crate::services::auditoria_service::AuditoriaService;

/// Helper row struct for JOIN query results
#[derive(Debug, sqlx::FromRow)]
struct TransacaoJoinRow {
    #[sqlx(rename = "Id")]
    id: i32,
    #[sqlx(rename = "Descricao")]
    descricao: String,
    #[sqlx(rename = "Valor")]
    valor: Decimal,
    #[sqlx(rename = "Tipo")]
    tipo: String,
    #[sqlx(rename = "Data")]
    data: chrono::DateTime<Utc>,
    #[sqlx(rename = "LoteIdentificador")]
    lote_identificador: Option<String>,
    #[sqlx(rename = "UsuarioNome")]
    usuario_nome: Option<String>,
    #[sqlx(rename = "GranjaNome")]
    granja_nome: Option<String>,
}

impl From<TransacaoJoinRow> for TransacaoSimplificadaDto {
    fn from(row: TransacaoJoinRow) -> Self {
        TransacaoSimplificadaDto {
            id: row.id,
            descricao: row.descricao,
            valor: row.valor,
            tipo: row.tipo,
            data: row.data,
            lote_identificador: row.lote_identificador,
            usuario_nome: row.usuario_nome,
            granja_nome: row.granja_nome,
        }
    }
}

/// Helper row for fetching transaction with creator role
#[derive(Debug, sqlx::FromRow)]
#[allow(dead_code)]
struct TransacaoComRole {
    #[sqlx(rename = "Id")]
    id: i32,
    #[sqlx(rename = "TimestampCriacao")]
    timestamp_criacao: chrono::DateTime<Utc>,
    #[sqlx(rename = "CriadorRole")]
    criador_role: Option<String>,
}

pub struct FinancasService;

impl FinancasService {
    /// GET /api/financas -- lista transacoes com filtro por role
    /// Administrador: todas, Financeiro: via FinanceiroProdutor chain
    pub async fn get_all(
        pool: &PgPool,
        user_id: i32,
        user_role: &str,
    ) -> Result<Vec<TransacaoSimplificadaDto>, AppError> {
        let rows = match user_role {
            "Administrador" => {
                sqlx::query_as::<_, TransacaoJoinRow>(
                    r#"SELECT t."Id", t."Descricao", t."Valor", t."Tipo", t."Data",
                              l."Identificador" AS "LoteIdentificador",
                              u."Nome" AS "UsuarioNome",
                              g."Nome" AS "GranjaNome"
                       FROM "TransacoesFinanceiras" t
                       LEFT JOIN "Lotes" l ON t."LoteId" = l."Id"
                       LEFT JOIN "Granjas" g ON l."GranjaId" = g."Id"
                       LEFT JOIN "Usuarios" u ON t."UsuarioId" = u."Id"
                       ORDER BY t."Data" DESC"#,
                )
                .fetch_all(pool)
                .await?
            }
            "Financeiro" => {
                sqlx::query_as::<_, TransacaoJoinRow>(
                    r#"SELECT t."Id", t."Descricao", t."Valor", t."Tipo", t."Data",
                              l."Identificador" AS "LoteIdentificador",
                              u."Nome" AS "UsuarioNome",
                              g."Nome" AS "GranjaNome"
                       FROM "TransacoesFinanceiras" t
                       LEFT JOIN "Lotes" l ON t."LoteId" = l."Id"
                       LEFT JOIN "Granjas" g ON l."GranjaId" = g."Id"
                       LEFT JOIN "Usuarios" u ON t."UsuarioId" = u."Id"
                       WHERE t."LoteId" IS NOT NULL
                         AND g."UsuarioId" IN (
                           SELECT fp."ProdutorId" FROM "FinanceiroProdutor" fp
                           WHERE fp."FinanceiroId" = $1
                         )
                       ORDER BY t."Data" DESC"#,
                )
                .bind(user_id)
                .fetch_all(pool)
                .await?
            }
            _ => Vec::new(),
        };

        Ok(rows.into_iter().map(TransacaoSimplificadaDto::from).collect())
    }

    /// POST /api/financas -- cria transacao financeira
    pub async fn create(
        pool: &PgPool,
        dto: &CreateTransacaoDto,
        user_id: i32,
        _user_role: &str,
        user_email: &str,
    ) -> Result<TransacaoSimplificadaDto, AppError> {
        // Handle LoteId: if 0 or None, set to NULL
        let lote_id = match dto.lote_id {
            Some(0) | None => None,
            Some(id) => Some(id),
        };

        let now = Utc::now();

        // INSERT RETURNING id
        let inserted_id: i32 = sqlx::query_scalar(
            r#"INSERT INTO "TransacoesFinanceiras" ("Descricao", "Valor", "Tipo", "Data", "LoteId", "UsuarioId", "TimestampCriacao")
               VALUES ($1, $2, $3, $4, $5, $6, $7)
               RETURNING "Id""#,
        )
        .bind(&dto.descricao)
        .bind(dto.valor)
        .bind(&dto.tipo)
        .bind(dto.data)
        .bind(lote_id)
        .bind(user_id)
        .bind(now)
        .fetch_one(pool)
        .await?;

        // Fetch full joined row for response
        let row = sqlx::query_as::<_, TransacaoJoinRow>(
            r#"SELECT t."Id", t."Descricao", t."Valor", t."Tipo", t."Data",
                      l."Identificador" AS "LoteIdentificador",
                      u."Nome" AS "UsuarioNome",
                      g."Nome" AS "GranjaNome"
               FROM "TransacoesFinanceiras" t
               LEFT JOIN "Lotes" l ON t."LoteId" = l."Id"
               LEFT JOIN "Granjas" g ON l."GranjaId" = g."Id"
               LEFT JOIN "Usuarios" u ON t."UsuarioId" = u."Id"
               WHERE t."Id" = $1"#,
        )
        .bind(inserted_id)
        .fetch_one(pool)
        .await?;

        // Audit log
        AuditoriaService::registrar_log(
            pool,
            user_id,
            user_email,
            "CRIACAO_TRANSACAO",
            &format!(
                "Transacao '{}' (ID: {}) criada.",
                dto.descricao, inserted_id
            ),
        )
        .await?;

        Ok(TransacaoSimplificadaDto::from(row))
    }

    /// PUT /api/financas/{id} -- atualiza transacao com regras de negocio
    pub async fn update(
        pool: &PgPool,
        id: i32,
        dto: &UpdateTransacaoDto,
        user_id: i32,
        user_role: &str,
        user_email: &str,
    ) -> Result<(), AppError> {
        // Fetch existing transaction with creator's role
        let transacao = sqlx::query_as::<_, TransacaoComRole>(
            r#"SELECT t."Id", t."TimestampCriacao",
                      p."Nome" AS "CriadorRole"
               FROM "TransacoesFinanceiras" t
               INNER JOIN "Usuarios" u ON t."UsuarioId" = u."Id"
               INNER JOIN "Perfis" p ON u."PerfilId" = p."Id"
               WHERE t."Id" = $1"#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Transacao com ID {} nao encontrada", id)))?;

        // REGRA 1: 5-minute edit window (Admin exempt)
        if user_role != "Administrador"
            && Utc::now() - transacao.timestamp_criacao > Duration::minutes(5)
        {
            return Err(AppError::BadRequest(
                "O tempo para edicao expirou. A transacao so pode ser editada nos primeiros 5 minutos.".into(),
            ));
        }

        // REGRA 2: Hierarchy check -- Financeiro cannot edit Admin's transactions
        if user_role == "Financeiro"
            && transacao.criador_role.as_deref() == Some("Administrador")
        {
            return Err(AppError::BadRequest(
                "Permissao negada. Um utilizador Financeiro nao pode editar uma transacao criada por um Administrador.".into(),
            ));
        }

        // Handle LoteId: if 0 or None, set to NULL
        let lote_id = match dto.lote_id {
            Some(0) | None => None,
            Some(id) => Some(id),
        };

        // UPDATE
        sqlx::query(
            r#"UPDATE "TransacoesFinanceiras"
               SET "Descricao" = $1, "Valor" = $2, "Tipo" = $3, "Data" = $4, "LoteId" = $5
               WHERE "Id" = $6"#,
        )
        .bind(&dto.descricao)
        .bind(dto.valor)
        .bind(&dto.tipo)
        .bind(dto.data)
        .bind(lote_id)
        .bind(id)
        .execute(pool)
        .await?;

        // Audit log
        AuditoriaService::registrar_log(
            pool,
            user_id,
            user_email,
            "ATUALIZACAO_TRANSACAO",
            &format!("Transacao (ID: {}) atualizada.", id),
        )
        .await?;

        Ok(())
    }

    /// DELETE /api/financas/{id} -- Admin-only delete
    pub async fn delete(
        pool: &PgPool,
        id: i32,
        user_id: i32,
        _user_role: &str,
        user_email: &str,
    ) -> Result<(), AppError> {
        // Verify exists
        let descricao: Option<String> = sqlx::query_scalar(
            r#"SELECT "Descricao" FROM "TransacoesFinanceiras" WHERE "Id" = $1"#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        let descricao = descricao
            .ok_or_else(|| AppError::NotFound(format!("Transacao com ID {} nao encontrada", id)))?;

        // DELETE
        sqlx::query(r#"DELETE FROM "TransacoesFinanceiras" WHERE "Id" = $1"#)
            .bind(id)
            .execute(pool)
            .await?;

        // Audit log
        AuditoriaService::registrar_log(
            pool,
            user_id,
            user_email,
            "DELECAO_TRANSACAO",
            &format!("Transacao '{}' (ID: {}) deletada.", descricao, id),
        )
        .await?;

        Ok(())
    }
}
