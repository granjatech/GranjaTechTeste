use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Evento sanitario (vacinacao, medicacao, doenca, preventivo)
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct EventoSanitario {
    #[sqlx(rename = "Id")]
    pub id: i32,
    #[sqlx(rename = "LoteId")]
    pub lote_id: i32,
    #[sqlx(rename = "Data")]
    pub data: DateTime<Utc>,
    #[sqlx(rename = "TipoEvento")]
    pub tipo_evento: String,
    #[sqlx(rename = "Produto")]
    pub produto: String,
    #[sqlx(rename = "LoteProduto")]
    pub lote_produto: Option<String>,
    #[sqlx(rename = "Dosagem")]
    pub dosagem: Option<String>,
    #[sqlx(rename = "ViaAdministracao")]
    pub via_administracao: Option<String>,
    #[sqlx(rename = "AvesTratadas")]
    pub aves_tratadas: Option<i32>,
    #[sqlx(rename = "DuracaoTratamentoDias")]
    pub duracao_tratamento_dias: Option<i32>,
    #[sqlx(rename = "PeriodoCarenciaDias")]
    pub periodo_carencia_dias: Option<i32>,
    #[sqlx(rename = "ResponsavelAplicacao")]
    pub responsavel_aplicacao: Option<String>,
    #[sqlx(rename = "Sintomas")]
    pub sintomas: Option<String>,
    #[sqlx(rename = "Observacoes")]
    pub observacoes: Option<String>,
    #[sqlx(rename = "Custo")]
    pub custo: Option<Decimal>,
    #[sqlx(rename = "DataCriacao")]
    pub data_criacao: DateTime<Utc>,
}
