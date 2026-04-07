use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

/// DTO de criacao de evento sanitario
#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateEventoSanitarioDto {
    pub lote_id: i32,
    pub data: DateTime<Utc>,
    #[validate(length(min = 1, max = 50))]
    pub tipo_evento: String,
    #[validate(length(min = 1, max = 200))]
    pub produto: String,
    #[validate(length(max = 100))]
    pub lote_produto: Option<String>,
    #[validate(length(max = 100))]
    pub dosagem: Option<String>,
    #[validate(length(max = 50))]
    pub via_administracao: Option<String>,
    #[validate(range(min = 1, max = 100000))]
    pub aves_tratadas: Option<i32>,
    #[validate(range(min = 1, max = 365))]
    pub duracao_tratamento_dias: Option<i32>,
    #[validate(range(min = 0, max = 365))]
    pub periodo_carencia_dias: Option<i32>,
    #[validate(length(max = 200))]
    pub responsavel_aplicacao: Option<String>,
    #[validate(length(max = 1000))]
    pub sintomas: Option<String>,
    #[validate(length(max = 1000))]
    pub observacoes: Option<String>,
    pub custo: Option<Decimal>,
}

/// DTO de registro de mortalidade
#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateRegistroMortalidadeDto {
    pub lote_id: Option<i32>,
    pub data: DateTime<Utc>,
    #[validate(range(min = 1))]
    pub quantidade_mortas: i32,
    pub causa_principal: Option<String>,
    pub peso_medio_mortas: Option<Decimal>,
    pub observacoes: Option<String>,
    pub acao_tomada: Option<String>,
}

/// DTO de resposta de registro de mortalidade
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegistroMortalidadeDto {
    pub id: i32,
    pub lote_id: i32,
    pub data: DateTime<Utc>,
    pub quantidade_mortas: i32,
    pub aves_vivas: i32,
    pub causa_principal: Option<String>,
    pub idade_dias: i32,
    pub peso_medio_mortas: Option<Decimal>,
    pub observacoes: Option<String>,
    pub acao_tomada: Option<String>,
    pub responsavel_registro: Option<String>,
    pub percentual_mortalidade_dia: Decimal,
}
