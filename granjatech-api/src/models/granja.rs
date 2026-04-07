use serde::{Deserialize, Serialize};

/// Granja (unidade produtiva)
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Granja {
    #[sqlx(rename = "Id")]
    pub id: i32,
    #[sqlx(rename = "Codigo")]
    pub codigo: String,
    #[sqlx(rename = "Nome")]
    pub nome: String,
    #[sqlx(rename = "Localizacao")]
    pub localizacao: Option<String>,
    #[sqlx(rename = "UsuarioId")]
    pub usuario_id: i32,
}
