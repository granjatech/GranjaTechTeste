use serde::{Deserialize, Serialize};

/// Associacao entre usuario Financeiro e usuario Produtor (chave composta)
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct FinanceiroProdutor {
    #[sqlx(rename = "FinanceiroId")]
    pub financeiro_id: i32,
    #[sqlx(rename = "ProdutorId")]
    pub produtor_id: i32,
}
