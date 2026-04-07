use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Analise de consumo de racao e agua de um lote
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AnaliseConsumoDto {
    pub lote_id: i32,
    pub lote_identificador: String,
    pub idade_atual_dias: i32,
    // Consumo de racao
    pub consumo_total_racao_kg: Decimal,
    pub consumo_medio_racao_por_ave: Decimal,
    pub consumo_medio_racao_por_dia: Decimal,
    pub consumo_acumulado_racao: Decimal,
    // Consumo de agua
    pub consumo_total_agua_litros: Decimal,
    pub consumo_medio_agua_por_ave: Decimal,
    pub consumo_medio_agua_por_dia: Decimal,
    pub consumo_acumulado_agua: Decimal,
    // Relacoes
    pub relacao_agua_racao: Decimal,
    pub relacao_consumo_ideal: Decimal,
    // Fases de racao
    pub consumos_por_fase: Vec<ConsumoFaseDto>,
    // Eficiencia
    pub eficiencia_conversao: Decimal,
    pub status_consumo: String,
    // Previsoes
    pub consumo_previsto_total: Decimal,
    pub custo_estimado_racao: Decimal,
}

/// Consumo por fase da racao
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConsumoFaseDto {
    pub fase: String,
    pub dia_inicio: i32,
    pub dia_fim: i32,
    pub quantidade_kg: Decimal,
    pub percentual_total: Decimal,
    pub consumo_medio_por_ave: Decimal,
}

/// Curvas de crescimento do lote
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CurvasCrescimentoDto {
    pub lote_id: i32,
    pub lote_identificador: String,
    pub idade_atual_dias: i32,
    pub curva_peso: Vec<PontoCurvaDto>,
    pub curva_consumo_racao: Vec<PontoCurvaDto>,
    pub curva_consumo_agua: Vec<PontoCurvaDto>,
    pub curva_mortalidade: Vec<PontoCurvaDto>,
    pub curva_ganho_medio_diario: Vec<PontoCurvaDto>,
}

/// Ponto em uma curva de crescimento
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PontoCurvaDto {
    pub dia: i32,
    pub semana: i32,
    pub valor: Decimal,
    pub valor_padrao: Option<Decimal>,
    pub data: DateTime<Utc>,
}

/// Alerta de parametro fora do aceitavel
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AlertaParametroDto {
    pub tipo_alerta: String,
    pub severidade: String,
    pub descricao: String,
    pub valor_atual: Decimal,
    pub valor_minimo: Option<Decimal>,
    pub valor_maximo: Option<Decimal>,
    pub unidade: String,
    pub data_ocorrencia: DateTime<Utc>,
    pub recomendacao: Option<String>,
    pub ativo: bool,
}

/// Comparacao de metricas com padroes da industria
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ComparacaoIndustriaDto {
    pub lote_id: i32,
    pub lote_identificador: String,
    pub idade_dias: i32,
    pub conversao_alimentar: MetricaComparacaoDto,
    pub ganho_medio_diario: MetricaComparacaoDto,
    pub viabilidade: MetricaComparacaoDto,
    pub iep: MetricaComparacaoDto,
    pub peso_medio: MetricaComparacaoDto,
    pub classificacao_geral: String,
    pub pontuacao_geral: Decimal,
}

/// Metrica comparada com padrao da industria
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MetricaComparacaoDto {
    pub nome: String,
    pub valor_atual: Decimal,
    pub valor_padrao_industria: Decimal,
    pub valor_padrao_excelencia: Decimal,
    pub percentual_diferenca: Decimal,
    pub status: String,
    pub unidade: String,
}

/// Projecao de abate
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ProjecaoAbateDto {
    pub lote_id: i32,
    pub lote_identificador: String,
    pub data_abate_prevista: DateTime<Utc>,
    pub idade_abate_dias: i32,
    pub peso_medio_atual_gramas: Decimal,
    pub peso_medio_projetado_gramas: Decimal,
    pub peso_total_projetado_kg: Decimal,
    pub quantidade_aves_projetada: i32,
    pub quantidade_aves_atual: i32,
    pub mortalidade_projetada_percentual: Decimal,
    pub rendimento_carcaca_estimado: Decimal,
    pub peso_carcaca_projetado_kg: Decimal,
    pub conversao_alimentar_projetada: Decimal,
    pub iep_projetado: Decimal,
    pub viabilidade_projetada: Decimal,
    pub valor_estimado_por_kg: Decimal,
    pub receita_bruta_estimada: Decimal,
    pub custo_producao_estimado: Decimal,
    pub lucro_estimado: Decimal,
    pub status_projecao: String,
    pub observacoes: Vec<String>,
}

/// Mortalidade por fase do lote
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegistroMortalidadePorFaseDto {
    pub fase: String,
    pub dia_inicio: i32,
    pub dia_fim: i32,
    pub total_mortes: i32,
    pub percentual_fase: Decimal,
    pub percentual_acumulado: Decimal,
    pub principais_causas: Vec<String>,
}

/// Resumo sanitario de um lote
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ResumoSanitarioDto {
    pub lote_id: i32,
    pub lote_identificador: String,
    pub total_eventos: i32,
    pub total_vacinacoes: i32,
    pub total_medicacoes: i32,
    pub total_doencas: i32,
    pub custo_total_sanitario: Decimal,
    pub custo_por_ave: Decimal,
    pub eventos_por_tipo: Vec<EventoSanitarioResumoDto>,
    pub cronograma_vacinacao: Vec<VacinacaoScheduleDto>,
    pub alertas_sanitarios: Vec<String>,
    pub proximas_acoes: Vec<ProximaAcaoSanitariaDto>,
}

/// Resumo de eventos sanitarios por tipo
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EventoSanitarioResumoDto {
    pub tipo_evento: String,
    pub quantidade: i32,
    pub custo_total: Decimal,
    pub ultima_ocorrencia: DateTime<Utc>,
}

/// Cronograma de vacinacao
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct VacinacaoScheduleDto {
    pub vacina: String,
    pub data_prevista: DateTime<Utc>,
    pub data_realizada: Option<DateTime<Utc>>,
    pub realizada: bool,
    pub status: String,
}

/// Proxima acao sanitaria programada
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ProximaAcaoSanitariaDto {
    pub acao: String,
    pub data_prevista: DateTime<Utc>,
    pub prioridade: String,
    pub descricao: String,
}

/// Metricas principais do lote (composite response for GET /{loteId}/metricas)
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MetricasLoteDto {
    pub iep: Decimal,
    pub conversao_alimentar: Decimal,
    pub ganho_medio_diario: Decimal,
    pub viabilidade: Decimal,
    pub uniformidade: Decimal,
    pub densidade_atual: Decimal,
}

/// Dashboard completo de avicultura (composite response for GET /{loteId}/dashboard)
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DashboardAviculturaDto {
    pub metricas: MetricasLoteDto,
    pub alertas: Vec<AlertaParametroDto>,
    pub comparacao_industria: ComparacaoIndustriaDto,
    pub resumo_sanitario: ResumoSanitarioDto,
    pub projecao_abate: ProjecaoAbateDto,
}

/// Resposta de estimativa de peso (composite response for GET /{loteId}/estimar-peso)
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EstimarPesoResponseDto {
    pub peso_estimado_gramas: Decimal,
    pub data_abate: DateTime<Utc>,
}

/// Query params for estimar-peso endpoint
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EstimarPesoQuery {
    pub data_abate: DateTime<Utc>,
}

// --- Default implementations for stub DTOs ---
// DateTime<Utc> does not implement Default, so we implement manually.

impl Default for AnaliseConsumoDto {
    fn default() -> Self {
        Self {
            lote_id: 0,
            lote_identificador: String::new(),
            idade_atual_dias: 0,
            consumo_total_racao_kg: Decimal::ZERO,
            consumo_medio_racao_por_ave: Decimal::ZERO,
            consumo_medio_racao_por_dia: Decimal::ZERO,
            consumo_acumulado_racao: Decimal::ZERO,
            consumo_total_agua_litros: Decimal::ZERO,
            consumo_medio_agua_por_ave: Decimal::ZERO,
            consumo_medio_agua_por_dia: Decimal::ZERO,
            consumo_acumulado_agua: Decimal::ZERO,
            relacao_agua_racao: Decimal::ZERO,
            relacao_consumo_ideal: Decimal::ZERO,
            consumos_por_fase: Vec::new(),
            eficiencia_conversao: Decimal::ZERO,
            status_consumo: String::new(),
            consumo_previsto_total: Decimal::ZERO,
            custo_estimado_racao: Decimal::ZERO,
        }
    }
}

impl Default for CurvasCrescimentoDto {
    fn default() -> Self {
        Self {
            lote_id: 0,
            lote_identificador: String::new(),
            idade_atual_dias: 0,
            curva_peso: Vec::new(),
            curva_consumo_racao: Vec::new(),
            curva_consumo_agua: Vec::new(),
            curva_mortalidade: Vec::new(),
            curva_ganho_medio_diario: Vec::new(),
        }
    }
}

impl Default for ResumoSanitarioDto {
    fn default() -> Self {
        Self {
            lote_id: 0,
            lote_identificador: String::new(),
            total_eventos: 0,
            total_vacinacoes: 0,
            total_medicacoes: 0,
            total_doencas: 0,
            custo_total_sanitario: Decimal::ZERO,
            custo_por_ave: Decimal::ZERO,
            eventos_por_tipo: Vec::new(),
            cronograma_vacinacao: Vec::new(),
            alertas_sanitarios: Vec::new(),
            proximas_acoes: Vec::new(),
        }
    }
}

impl Default for ProjecaoAbateDto {
    fn default() -> Self {
        Self {
            lote_id: 0,
            lote_identificador: String::new(),
            data_abate_prevista: Utc::now(),
            idade_abate_dias: 0,
            peso_medio_atual_gramas: Decimal::ZERO,
            peso_medio_projetado_gramas: Decimal::ZERO,
            peso_total_projetado_kg: Decimal::ZERO,
            quantidade_aves_projetada: 0,
            quantidade_aves_atual: 0,
            mortalidade_projetada_percentual: Decimal::ZERO,
            rendimento_carcaca_estimado: Decimal::ZERO,
            peso_carcaca_projetado_kg: Decimal::ZERO,
            conversao_alimentar_projetada: Decimal::ZERO,
            iep_projetado: Decimal::ZERO,
            viabilidade_projetada: Decimal::ZERO,
            valor_estimado_por_kg: Decimal::ZERO,
            receita_bruta_estimada: Decimal::ZERO,
            custo_producao_estimado: Decimal::ZERO,
            lucro_estimado: Decimal::ZERO,
            status_projecao: String::new(),
            observacoes: Vec::new(),
        }
    }
}

impl Default for ComparacaoIndustriaDto {
    fn default() -> Self {
        Self {
            lote_id: 0,
            lote_identificador: String::new(),
            idade_dias: 0,
            conversao_alimentar: MetricaComparacaoDto::default(),
            ganho_medio_diario: MetricaComparacaoDto::default(),
            viabilidade: MetricaComparacaoDto::default(),
            iep: MetricaComparacaoDto::default(),
            peso_medio: MetricaComparacaoDto::default(),
            classificacao_geral: String::new(),
            pontuacao_geral: Decimal::ZERO,
        }
    }
}

impl Default for MetricaComparacaoDto {
    fn default() -> Self {
        Self {
            nome: String::new(),
            valor_atual: Decimal::ZERO,
            valor_padrao_industria: Decimal::ZERO,
            valor_padrao_excelencia: Decimal::ZERO,
            percentual_diferenca: Decimal::ZERO,
            status: String::new(),
            unidade: String::new(),
        }
    }
}
