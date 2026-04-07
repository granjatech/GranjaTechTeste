use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Medicao de qualidade do ar no galpao
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct QualidadeAr {
    #[sqlx(rename = "Id")]
    pub id: i32,
    #[sqlx(rename = "LoteId")]
    pub lote_id: i32,
    #[sqlx(rename = "DataHora")]
    pub data_hora: DateTime<Utc>,
    #[sqlx(rename = "NH3_ppm")]
    pub nh3_ppm: Option<Decimal>,
    #[sqlx(rename = "CO2_ppm")]
    pub co2_ppm: Option<Decimal>,
    #[sqlx(rename = "O2_percentual")]
    pub o2_percentual: Option<Decimal>,
    #[sqlx(rename = "VelocidadeAr_ms")]
    pub velocidade_ar_ms: Option<Decimal>,
    #[sqlx(rename = "Luminosidade_lux")]
    pub luminosidade_lux: Option<Decimal>,
    #[sqlx(rename = "TemperaturaAr")]
    pub temperatura_ar: Option<Decimal>,
    #[sqlx(rename = "UmidadeRelativa")]
    pub umidade_relativa: Option<Decimal>,
    #[sqlx(rename = "LocalMedicao")]
    pub local_medicao: Option<String>,
    #[sqlx(rename = "EquipamentoMedicao")]
    pub equipamento_medicao: Option<String>,
    #[sqlx(rename = "Observacoes")]
    pub observacoes: Option<String>,
    #[sqlx(rename = "DataCriacao")]
    pub data_criacao: DateTime<Utc>,
}
