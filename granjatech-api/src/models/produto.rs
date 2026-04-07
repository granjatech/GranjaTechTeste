use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Produto em estoque (racao, vacina, medicamento)
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Produto {
    #[sqlx(rename = "Id")]
    pub id: i32,
    #[sqlx(rename = "Nome")]
    pub nome: String,
    #[sqlx(rename = "Tipo")]
    pub tipo: String,
    #[sqlx(rename = "Quantidade")]
    pub quantidade: Decimal,
    #[sqlx(rename = "UnidadeDeMedida")]
    pub unidade_de_medida: String,
    #[sqlx(rename = "GranjaId")]
    pub granja_id: i32,
}
