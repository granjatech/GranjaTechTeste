use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Registro de abate de um lote
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct RegistroAbate {
    #[sqlx(rename = "Id")]
    pub id: i32,
    #[sqlx(rename = "LoteId")]
    pub lote_id: i32,
    #[sqlx(rename = "DataAbate")]
    pub data_abate: DateTime<Utc>,
    #[sqlx(rename = "DataAbatePrevista")]
    pub data_abate_prevista: Option<DateTime<Utc>>,
    #[sqlx(rename = "IdadeAbateDias")]
    pub idade_abate_dias: i32,
    #[sqlx(rename = "QuantidadeEnviada")]
    pub quantidade_enviada: i32,
    #[sqlx(rename = "PesoVivoTotalKg")]
    pub peso_vivo_total_kg: Decimal,
    #[sqlx(rename = "PesoCarcacaTotalKg")]
    pub peso_carcaca_total_kg: Option<Decimal>,
    #[sqlx(rename = "AvesCondenadas")]
    pub aves_condenadas: Option<i32>,
    #[sqlx(rename = "MotivoCondenacoes")]
    pub motivo_condenacoes: Option<String>,
    #[sqlx(rename = "PesoCondenadoKg")]
    pub peso_condenado_kg: Option<Decimal>,
    #[sqlx(rename = "FrigorificoDestino")]
    pub frigorifico_destino: Option<String>,
    #[sqlx(rename = "Transportadora")]
    pub transportadora: Option<String>,
    #[sqlx(rename = "ValorPorKg")]
    pub valor_por_kg: Option<Decimal>,
    #[sqlx(rename = "ValorTotalRecebido")]
    pub valor_total_recebido: Option<Decimal>,
    #[sqlx(rename = "Observacoes")]
    pub observacoes: Option<String>,
    #[sqlx(rename = "DataCriacao")]
    pub data_criacao: DateTime<Utc>,
}
