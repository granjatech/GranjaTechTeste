use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Registro de mortalidade de aves em um lote
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct RegistroMortalidade {
    #[sqlx(rename = "Id")]
    pub id: i32,
    #[sqlx(rename = "LoteId")]
    pub lote_id: i32,
    #[sqlx(rename = "Data")]
    pub data: DateTime<Utc>,
    #[sqlx(rename = "Quantidade")]
    pub quantidade: i32,
    #[sqlx(rename = "Motivo")]
    pub motivo: Option<String>,
    #[sqlx(rename = "Setor")]
    pub setor: Option<String>,
    #[sqlx(rename = "Observacoes")]
    pub observacoes: Option<String>,
    #[sqlx(rename = "ResponsavelRegistro")]
    pub responsavel_registro: Option<String>,
    #[sqlx(rename = "DataCriacao")]
    pub data_criacao: DateTime<Utc>,
}
