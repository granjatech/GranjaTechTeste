use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Consumo de racao de um lote
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ConsumoRacao {
    #[sqlx(rename = "Id")]
    pub id: i32,
    #[sqlx(rename = "LoteId")]
    pub lote_id: i32,
    #[sqlx(rename = "Data")]
    pub data: DateTime<Utc>,
    #[sqlx(rename = "QuantidadeKg")]
    pub quantidade_kg: Decimal,
    #[sqlx(rename = "TipoRacao")]
    pub tipo_racao: String,
    #[sqlx(rename = "AvesVivas")]
    pub aves_vivas: i32,
    #[sqlx(rename = "Observacoes")]
    pub observacoes: Option<String>,
    #[sqlx(rename = "DataCriacao")]
    pub data_criacao: DateTime<Utc>,
}

/// Consumo de agua de um lote
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ConsumoAgua {
    #[sqlx(rename = "Id")]
    pub id: i32,
    #[sqlx(rename = "LoteId")]
    pub lote_id: i32,
    #[sqlx(rename = "Data")]
    pub data: DateTime<Utc>,
    #[sqlx(rename = "QuantidadeLitros")]
    pub quantidade_litros: Decimal,
    #[sqlx(rename = "AvesVivas")]
    pub aves_vivas: i32,
    #[sqlx(rename = "TemperaturaAmbiente")]
    pub temperatura_ambiente: Option<Decimal>,
    #[sqlx(rename = "Observacoes")]
    pub observacoes: Option<String>,
    #[sqlx(rename = "DataCriacao")]
    pub data_criacao: DateTime<Utc>,
}
