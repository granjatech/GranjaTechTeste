use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

/// DTO de criacao de pesagem semanal
#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreatePesagemSemanalDto {
    pub lote_id: i32,
    pub data_pesagem: DateTime<Utc>,
    #[validate(range(min = 1, max = 365))]
    pub idade_dias: i32,
    #[validate(range(min = 1, max = 52))]
    pub semana_vida: i32,
    pub peso_medio_gramas: Decimal,
    #[validate(range(min = 10, max = 1000))]
    pub quantidade_amostrada: i32,
    pub peso_minimo: Option<Decimal>,
    pub peso_maximo: Option<Decimal>,
    pub desvio_padrao: Option<Decimal>,
    pub coeficiente_variacao: Option<Decimal>,
    pub ganho_semanal: Option<Decimal>,
    #[validate(length(max = 500))]
    pub observacoes: Option<String>,
}

/// DTO de resposta de pesagem semanal
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PesagemSemanalResponseDto {
    pub id: i32,
    pub lote_id: i32,
    pub data_pesagem: DateTime<Utc>,
    pub idade_dias: i32,
    pub semana_vida: i32,
    pub peso_medio_gramas: Decimal,
    pub quantidade_amostrada: i32,
    pub peso_minimo: Option<Decimal>,
    pub peso_maximo: Option<Decimal>,
    pub desvio_padrao: Option<Decimal>,
    pub coeficiente_variacao: Option<Decimal>,
    pub ganho_semanal: Option<Decimal>,
    pub ganho_medio_diario: Decimal,
    pub observacoes: Option<String>,
    pub data_criacao: DateTime<Utc>,
}
