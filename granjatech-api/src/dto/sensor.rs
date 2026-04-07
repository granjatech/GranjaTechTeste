use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

/// DTO de criacao de sensor
#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateSensorDto {
    #[validate(length(min = 1))]
    pub tipo: String,
    #[validate(length(min = 1))]
    pub identificador_unico: String,
    pub granja_id: i32,
}

/// DTO de criacao de leitura de sensor
#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateLeituraDto {
    #[validate(length(min = 1))]
    pub identificador_unico: String,
    pub valor: Decimal,
}

/// DTO de resposta de sensor
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SensorResponseDto {
    pub id: i32,
    pub tipo: String,
    pub identificador_unico: String,
    pub granja_id: i32,
}

/// DTO de resposta de leitura de sensor
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LeituraSensorResponseDto {
    pub id: i32,
    pub sensor_id: i32,
    pub valor: Decimal,
    pub timestamp: DateTime<Utc>,
}
