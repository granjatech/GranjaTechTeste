use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

/// DTO de criacao de transacao financeira
#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateTransacaoDto {
    #[validate(length(min = 1))]
    pub descricao: String,
    pub valor: Decimal,
    #[validate(length(min = 1))]
    pub tipo: String,
    pub data: DateTime<Utc>,
    pub lote_id: Option<i32>,
}

/// DTO de atualizacao de transacao financeira
#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTransacaoDto {
    #[validate(length(min = 1))]
    pub descricao: String,
    pub valor: Decimal,
    #[validate(length(min = 1))]
    pub tipo: String,
    pub data: DateTime<Utc>,
    pub lote_id: Option<i32>,
}

/// DTO simplificado de transacao para listagem
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TransacaoSimplificadaDto {
    pub id: i32,
    pub descricao: String,
    pub valor: Decimal,
    pub tipo: String,
    pub data: DateTime<Utc>,
    pub lote_identificador: Option<String>,
    pub usuario_nome: Option<String>,
    pub granja_nome: Option<String>,
}

/// Relatorio financeiro completo
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RelatorioFinanceiroDto {
    pub total_entradas: Decimal,
    pub total_saidas: Decimal,
    pub saldo: Decimal,
    pub transacoes: Vec<TransacaoSimplificadaDto>,
}

/// Relatorio financeiro simplificado
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RelatorioFinanceiroSimplificadoDto {
    pub total_entradas: Decimal,
    pub total_saidas: Decimal,
    pub saldo: Decimal,
    pub transacoes: Vec<TransacaoSimplificadaDto>,
}

/// Item de relatorio financeiro por periodo
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct FinanceReportItemDto {
    pub data: DateTime<Utc>,
    pub categoria: String,
    pub descricao: String,
    pub valor: Decimal,
}

/// Relatorio financeiro por granja
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct FinanceReportDto {
    pub granja_id: i32,
    pub inicio: DateTime<Utc>,
    pub fim: DateTime<Utc>,
    pub total_entradas: Decimal,
    pub total_saidas: Decimal,
    pub saldo: Decimal,
    pub itens: Vec<FinanceReportItemDto>,
}
