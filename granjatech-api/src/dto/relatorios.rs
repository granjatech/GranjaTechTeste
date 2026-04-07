use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::Serialize;
use utoipa::ToSchema;

/// Relatorio de producao
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RelatorioProducaoDto {
    pub total_lotes: i32,
    pub total_aves_inicial: i32,
    pub lotes: Vec<super::lote::LoteResponseDto>,
}

/// Resumo de consumo para relatorio geral
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConsumoResumoDto {
    pub data: DateTime<Utc>,
    pub racao_kg: f64,
    pub agua_litros: f64,
    pub aves_vivas: i32,
}

/// Resumo de pesagem para relatorio geral
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PesagemResumoDto {
    pub data: DateTime<Utc>,
    pub peso_medio_kg: f64,
    pub amostra: i32,
}

/// Resumo sanitario para relatorio geral
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SanitarioResumoDto {
    pub data: DateTime<Utc>,
    pub tipo_evento: String,
    pub produto: String,
    pub via: Option<String>,
}

/// Resumo de sensor para relatorio geral
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SensorResumoDto {
    pub data: DateTime<Utc>,
    pub tipo: String,
    pub valor: f64,
}

/// Relatorio geral de uma granja
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GeralReportDto {
    pub granja_id: i32,
    pub inicio: DateTime<Utc>,
    pub fim: DateTime<Utc>,
    pub consumo: Vec<ConsumoResumoDto>,
    pub pesagens: Vec<PesagemResumoDto>,
    pub sanitario: Vec<SanitarioResumoDto>,
    pub sensores: Vec<SensorResumoDto>,
}

/// Relatorio setorial generico
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SetorReportDto<TItem: Serialize> {
    pub granja_id: i32,
    pub setor: String,
    pub inicio: DateTime<Utc>,
    pub fim: DateTime<Utc>,
    pub itens: Vec<TItem>,
}

/// Registro de abate para relatorio
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegistroAbateResponseDto {
    pub id: i32,
    pub lote_id: i32,
    pub data_abate: DateTime<Utc>,
    pub data_abate_prevista: Option<DateTime<Utc>>,
    pub idade_abate_dias: i32,
    pub quantidade_enviada: i32,
    pub peso_vivo_total_kg: Decimal,
    pub peso_medio_por_ave: Decimal,
    pub peso_carcaca_total_kg: Option<Decimal>,
    pub rendimento_carcaca: Decimal,
    pub aves_condenadas: Option<i32>,
    pub motivo_condenacoes: Option<String>,
    pub peso_condenado_kg: Option<Decimal>,
    pub frigorifico_destino: Option<String>,
    pub transportadora: Option<String>,
    pub valor_por_kg: Option<Decimal>,
    pub valor_total_recebido: Option<Decimal>,
    pub observacoes: Option<String>,
    pub data_criacao: DateTime<Utc>,
}
