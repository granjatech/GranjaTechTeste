use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

/// DTO de criacao de produto em estoque
#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateProdutoDto {
    #[validate(length(min = 1))]
    pub nome: String,
    #[validate(length(min = 1))]
    pub tipo: String,
    pub quantidade: Decimal,
    #[validate(length(min = 1))]
    pub unidade_de_medida: String,
    pub granja_id: i32,
}

/// DTO de atualizacao de produto em estoque
#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProdutoDto {
    #[validate(length(min = 1))]
    pub nome: String,
    #[validate(length(min = 1))]
    pub tipo: String,
    pub quantidade: Decimal,
    #[validate(length(min = 1))]
    pub unidade_de_medida: String,
    pub granja_id: i32,
}

/// DTO de resposta de produto em estoque
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ProdutoResponseDto {
    pub id: i32,
    pub nome: String,
    pub tipo: String,
    pub quantidade: Decimal,
    pub unidade_de_medida: String,
    pub granja_id: i32,
}
