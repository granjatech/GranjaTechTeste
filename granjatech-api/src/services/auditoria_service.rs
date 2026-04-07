use chrono::Utc;
use sqlx::PgPool;

use crate::errors::AppError;

pub struct AuditoriaService;

impl AuditoriaService {
    /// Registra log de auditoria na tabela LogsAuditoria
    pub async fn registrar_log(
        pool: &PgPool,
        usuario_id: i32,
        usuario_email: &str,
        acao: &str,
        detalhes: &str,
    ) -> Result<(), AppError> {
        sqlx::query(
            r#"INSERT INTO "LogsAuditoria" ("Timestamp", "UsuarioId", "UsuarioEmail", "Acao", "Detalhes")
               VALUES ($1, $2, $3, $4, $5)"#,
        )
        .bind(Utc::now())
        .bind(usuario_id)
        .bind(usuario_email)
        .bind(acao)
        .bind(detalhes)
        .execute(pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

        tracing::info!(
            usuario_id = usuario_id,
            acao = acao,
            "Auditoria registrada"
        );

        Ok(())
    }
}
