use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

/// DTO de criacao de granja
#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateGranjaDto {
    #[validate(length(min = 2, max = 100))]
    pub nome: String,
    pub localizacao: Option<String>,
    pub usuario_id: Option<i32>,
}

/// DTO de atualizacao de granja
#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateGranjaDto {
    #[validate(length(min = 2, max = 100))]
    pub nome: String,
    pub localizacao: Option<String>,
    pub usuario_id: i32,
}

/// DTO de resposta de granja
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GranjaResponseDto {
    pub id: i32,
    pub codigo: String,
    pub nome: String,
    pub localizacao: Option<String>,
    pub usuario_id: i32,
}
