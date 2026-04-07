use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Log de auditoria do sistema
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct LogAuditoria {
    #[sqlx(rename = "Id")]
    pub id: i32,
    #[sqlx(rename = "Timestamp")]
    pub timestamp: DateTime<Utc>,
    #[sqlx(rename = "UsuarioId")]
    pub usuario_id: i32,
    #[sqlx(rename = "UsuarioEmail")]
    pub usuario_email: String,
    #[sqlx(rename = "Acao")]
    pub acao: String,
    #[sqlx(rename = "Detalhes")]
    pub detalhes: String,
}
