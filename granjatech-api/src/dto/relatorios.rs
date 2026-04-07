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

// === Avicultura Report DTOs (RELA-05) ===

/// Top-level avicultura report response
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RelatorioAviculturaDto {
    pub periodo_inicio: DateTime<Utc>,
    pub periodo_fim: DateTime<Utc>,
    pub total_lotes: i32,
    pub data_geracao: DateTime<Utc>,
    pub resumo_geral: ResumoGeralAviculturaDto,
    pub detalhes_por_lote: Vec<DetalheLoteAviculturaDto>,
    pub benchmarks: BenchmarksAviculturaDto,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ResumoGeralAviculturaDto {
    pub total_aves_alojadas: i32,
    pub total_aves_atuais: i32,
    pub mortalidade_media: Decimal,
    pub viabilidade_media: Decimal,
    pub consumo_total_racao: Decimal,
    pub consumo_total_agua: Decimal,
    pub total_eventos_sanitarios: i32,
    pub custo_total_sanitario: Decimal,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DetalheLoteAviculturaDto {
    pub lote_id: i32,
    pub identificador: String,
    pub granja: String,
    pub data_entrada: DateTime<Utc>,
    pub idade_atual_dias: i64,
    pub status: String,
    pub quantidade_inicial: i32,
    pub quantidade_atual: i32,
    pub mortalidade_percentual: Decimal,
    pub viabilidade: Decimal,
    pub densidade_atual: Decimal,
    pub ganho_medio_diario: Decimal,
    pub consumo_racao_total_kg: Decimal,
    pub consumo_racao_media_por_ave: Decimal,
    pub consumo_agua_total_litros: Decimal,
    pub consumo_agua_media_por_ave: Decimal,
    pub relacao_agua_racao: Decimal,
    pub conversao_alimentar: Decimal,
    pub iep: Decimal,
    pub eventos_sanitarios_total: i32,
    pub eventos_vacinacoes: i32,
    pub eventos_medicacoes: i32,
    pub eventos_custo_total: Decimal,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BenchmarksAviculturaDto {
    pub melhor_conversao_alimentar: Decimal,
    pub melhor_iep: Decimal,
    pub melhor_viabilidade: Decimal,
    pub menor_mortalidade: Decimal,
}

// === Desempenho Lote DTOs (RELA-06) ===

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RelatorioDesempenhoLoteDto {
    pub lote_id: i32,
    pub identificador: String,
    pub granja: String,
    pub data_entrada: DateTime<Utc>,
    pub idade_atual_dias: i64,
    pub status: String,
    pub linhagem: Option<String>,
    pub origem_pintinhos: Option<String>,
    pub performance: PerformanceLoteDto,
    pub curva_crescimento: Vec<CurvaCrescimentoItemDto>,
    pub consumo_racao: Vec<ConsumoRacaoGroupDto>,
    pub consumo_agua: Vec<ConsumoAguaItemDto>,
    pub historico_sanitario: Vec<HistoricoSanitarioItemDto>,
    pub analise_mortalidade: Vec<AnaliseMortalidadeItemDto>,
    pub qualidade_ambiental: Vec<QualidadeAmbientalItemDto>,
    pub data_geracao: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PerformanceLoteDto {
    pub quantidade_inicial: i32,
    pub quantidade_atual: i32,
    pub mortalidade_total: i32,
    pub mortalidade_percentual: Decimal,
    pub viabilidade: Decimal,
    pub densidade_atual: Decimal,
    pub conversao_alimentar: Decimal,
    pub iep: Decimal,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CurvaCrescimentoItemDto {
    pub semana: i32,
    pub idade_dias: i32,
    pub peso_medio: Decimal,
    pub ganho_semanal: Decimal,
    pub ganho_medio_diario: Decimal,
    pub uniformidade: Decimal,
    pub quantidade_amostrada: i32,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConsumoRacaoGroupDto {
    pub tipo_racao: String,
    pub total_kg: Decimal,
    pub media_por_ave: Decimal,
    pub registros_por_dia: Vec<ConsumoRacaoDiaDto>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConsumoRacaoDiaDto {
    pub data: DateTime<Utc>,
    pub quantidade_kg: Decimal,
    pub aves_vivas: i32,
    pub consumo_por_ave: Decimal,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConsumoAguaItemDto {
    pub data: DateTime<Utc>,
    pub quantidade_litros: Decimal,
    pub aves_vivas: i32,
    pub consumo_por_ave: Decimal,
    pub temperatura_ambiente: Option<Decimal>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct HistoricoSanitarioItemDto {
    pub data: DateTime<Utc>,
    pub tipo_evento: String,
    pub produto: String,
    pub via_administracao: Option<String>,
    pub aves_tratadas: Option<i32>,
    pub custo: Option<Decimal>,
    pub periodo_carencia: Option<i32>,
    pub responsavel: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AnaliseMortalidadeItemDto {
    pub data: DateTime<Utc>,
    pub idade_dias: i32,
    pub quantidade_mortas: i32,
    pub percentual_dia: Decimal,
    pub causa_principal: Option<String>,
    pub aves_vivas: i32,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct QualidadeAmbientalItemDto {
    pub data_hora: DateTime<Utc>,
    pub temperatura_ar: Option<Decimal>,
    pub umidade_relativa: Option<Decimal>,
    pub nh3_ppm: Option<Decimal>,
    pub co2_ppm: Option<Decimal>,
    pub o2_percentual: Option<Decimal>,
    pub velocidade_ar: Option<Decimal>,
    pub luminosidade: Option<Decimal>,
    pub parametros_ok: bool,
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
