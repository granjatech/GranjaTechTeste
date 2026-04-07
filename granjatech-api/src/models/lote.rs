use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Lote de aves -- apenas colunas do banco de dados.
/// Propriedades computadas (Viabilidade, IEP, CA) ficam na camada de servico.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Lote {
    #[sqlx(rename = "Id")]
    pub id: i32,
    #[sqlx(rename = "Codigo")]
    pub codigo: String,
    #[sqlx(rename = "Identificador")]
    pub identificador: String,
    #[sqlx(rename = "DataEntrada")]
    pub data_entrada: DateTime<Utc>,
    #[sqlx(rename = "DataAbatePrevista")]
    pub data_abate_prevista: Option<DateTime<Utc>>,
    #[sqlx(rename = "DataSaida")]
    pub data_saida: Option<DateTime<Utc>>,
    #[sqlx(rename = "QuantidadeAvesInicial")]
    pub quantidade_aves_inicial: i32,
    #[sqlx(rename = "QuantidadeAvesAtual")]
    pub quantidade_aves_atual: i32,
    #[sqlx(rename = "AreaGalpao")]
    pub area_galpao: Option<Decimal>,
    #[sqlx(rename = "Linhagem")]
    pub linhagem: Option<String>,
    #[sqlx(rename = "OrigemPintinhos")]
    pub origem_pintinhos: Option<String>,
    #[sqlx(rename = "Status")]
    pub status: String,
    #[sqlx(rename = "Observacoes")]
    pub observacoes: Option<String>,
    #[sqlx(rename = "GranjaId")]
    pub granja_id: i32,
    #[sqlx(rename = "DataCriacao")]
    pub data_criacao: DateTime<Utc>,
    #[sqlx(rename = "DataAtualizacao")]
    pub data_atualizacao: Option<DateTime<Utc>>,
}
