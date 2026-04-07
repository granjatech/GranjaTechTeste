use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Pesagem semanal das aves de um lote
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PesagemSemanal {
    #[sqlx(rename = "Id")]
    pub id: i32,
    #[sqlx(rename = "LoteId")]
    pub lote_id: i32,
    #[sqlx(rename = "DataPesagem")]
    pub data_pesagem: DateTime<Utc>,
    #[sqlx(rename = "IdadeDias")]
    pub idade_dias: i32,
    #[sqlx(rename = "SemanaVida")]
    pub semana_vida: i32,
    #[sqlx(rename = "PesoMedioGramas")]
    pub peso_medio_gramas: Decimal,
    #[sqlx(rename = "QuantidadeAmostrada")]
    pub quantidade_amostrada: i32,
    #[sqlx(rename = "PesoMinimo")]
    pub peso_minimo: Option<Decimal>,
    #[sqlx(rename = "PesoMaximo")]
    pub peso_maximo: Option<Decimal>,
    #[sqlx(rename = "DesvioPadrao")]
    pub desvio_padrao: Option<Decimal>,
    #[sqlx(rename = "CoeficienteVariacao")]
    pub coeficiente_variacao: Option<Decimal>,
    #[sqlx(rename = "GanhoSemanal")]
    pub ganho_semanal: Option<Decimal>,
    #[sqlx(rename = "Observacoes")]
    pub observacoes: Option<String>,
    #[sqlx(rename = "DataCriacao")]
    pub data_criacao: DateTime<Utc>,
}
