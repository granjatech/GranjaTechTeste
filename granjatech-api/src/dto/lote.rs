use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

/// DTO de criacao de lote
#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateLoteDto {
    #[validate(length(min = 1, max = 100))]
    pub identificador: String,
    #[validate(range(min = 1, max = 100000))]
    pub quantidade_aves_inicial: i32,
    pub data_entrada: DateTime<Utc>,
    pub data_saida: Option<DateTime<Utc>>,
    pub granja_id: i32,
}

/// DTO de atualizacao de lote
#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateLoteDto {
    #[validate(length(min = 1, max = 100))]
    pub identificador: String,
    #[validate(range(min = 1, max = 100000))]
    pub quantidade_aves_inicial: i32,
    pub data_entrada: DateTime<Utc>,
    pub data_saida: Option<DateTime<Utc>>,
    pub granja_id: i32,
}

/// DTO de resposta de lote com metricas computadas
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LoteResponseDto {
    pub id: i32,
    pub codigo: String,
    pub identificador: String,
    pub data_entrada: DateTime<Utc>,
    pub data_abate_prevista: Option<DateTime<Utc>>,
    pub data_saida: Option<DateTime<Utc>>,
    pub quantidade_aves_inicial: i32,
    pub quantidade_aves_atual: i32,
    pub area_galpao: Option<Decimal>,
    pub linhagem: Option<String>,
    pub origem_pintinhos: Option<String>,
    pub status: String,
    pub observacoes: Option<String>,
    pub granja_id: i32,
    pub data_criacao: DateTime<Utc>,
    pub data_atualizacao: Option<DateTime<Utc>>,
    // Metricas computadas
    pub idade_atual_dias: i32,
    pub viabilidade: Decimal,
    pub densidade_atual: Decimal,
}
