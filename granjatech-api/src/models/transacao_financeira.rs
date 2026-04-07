use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Transacao financeira (entrada ou saida)
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TransacaoFinanceira {
    #[sqlx(rename = "Id")]
    pub id: i32,
    #[sqlx(rename = "Descricao")]
    pub descricao: String,
    #[sqlx(rename = "Valor")]
    pub valor: Decimal,
    #[sqlx(rename = "Tipo")]
    pub tipo: String,
    #[sqlx(rename = "Data")]
    pub data: DateTime<Utc>,
    #[sqlx(rename = "TimestampCriacao")]
    pub timestamp_criacao: DateTime<Utc>,
    #[sqlx(rename = "UsuarioId")]
    pub usuario_id: i32,
    #[sqlx(rename = "LoteId")]
    pub lote_id: Option<i32>,
}
