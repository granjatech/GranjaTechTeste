use rust_decimal::Decimal;
use serde::Serialize;
use utoipa::ToSchema;

/// KPIs do dashboard principal
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DashboardKpiDto {
    pub total_entradas: Decimal,
    pub total_saidas: Decimal,
    pub lucro_total: Decimal,
    pub lotes_ativos: i32,
}

/// Resumo mensal financeiro
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MonthlySummaryDto {
    pub mes: String,
    pub entradas: Decimal,
    pub saidas: Decimal,
}
