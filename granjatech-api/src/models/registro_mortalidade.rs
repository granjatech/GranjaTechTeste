use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
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
    #[sqlx(rename = "QuantidadeMortas")]
    pub quantidade_mortas: i32,
    #[sqlx(rename = "AvesVivas")]
    pub aves_vivas: i32,
    #[sqlx(rename = "CausaPrincipal")]
    pub causa_principal: Option<String>,
    #[sqlx(rename = "IdadeDias")]
    pub idade_dias: i32,
    #[sqlx(rename = "PesoMedioMortas")]
    pub peso_medio_mortas: Option<Decimal>,
    #[sqlx(rename = "Observacoes")]
    pub observacoes: Option<String>,
    #[sqlx(rename = "AcaoTomada")]
    pub acao_tomada: Option<String>,
    #[sqlx(rename = "ResponsavelRegistro")]
    pub responsavel_registro: Option<String>,
    #[sqlx(rename = "DataCriacao")]
    pub data_criacao: DateTime<Utc>,
}
