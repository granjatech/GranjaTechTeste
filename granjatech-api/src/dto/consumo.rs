use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

/// DTO de criacao de consumo de racao
#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateConsumoRacaoDto {
    pub lote_id: i32,
    pub data: DateTime<Utc>,
    pub quantidade_kg: Decimal,
    #[validate(length(min = 1, max = 50))]
    pub tipo_racao: String,
    #[validate(range(min = 1, max = 100000))]
    pub aves_vivas: i32,
    #[validate(length(max = 500))]
    pub observacoes: Option<String>,
}

/// DTO de criacao de consumo de agua
#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateConsumoAguaDto {
    pub lote_id: i32,
    pub data: DateTime<Utc>,
    pub quantidade_litros: Decimal,
    #[validate(range(min = 1, max = 100000))]
    pub aves_vivas: i32,
    pub temperatura_ambiente: Option<Decimal>,
    #[validate(length(max = 500))]
    pub observacoes: Option<String>,
}

/// DTO de resposta de consumo de racao
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConsumoRacaoResponseDto {
    pub id: i32,
    pub lote_id: i32,
    pub data: DateTime<Utc>,
    pub quantidade_kg: Decimal,
    pub tipo_racao: String,
    pub aves_vivas: i32,
    pub consumo_por_ave_gramas: Decimal,
    pub observacoes: Option<String>,
    pub data_criacao: DateTime<Utc>,
}

/// DTO de resposta de consumo de agua
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConsumoAguaResponseDto {
    pub id: i32,
    pub lote_id: i32,
    pub data: DateTime<Utc>,
    pub quantidade_litros: Decimal,
    pub aves_vivas: i32,
    pub consumo_por_ave_ml: Decimal,
    pub temperatura_ambiente: Option<Decimal>,
    pub observacoes: Option<String>,
    pub data_criacao: DateTime<Utc>,
}
