use serde::{Deserialize, Serialize};

/// Perfil de usuario (Administrador, Produtor, Financeiro)
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Perfil {
    #[sqlx(rename = "Id")]
    pub id: i32,
    #[sqlx(rename = "Nome")]
    pub nome: String,
}

/// Usuario do sistema
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Usuario {
    #[sqlx(rename = "Id")]
    pub id: i32,
    #[sqlx(rename = "Codigo")]
    pub codigo: String,
    #[sqlx(rename = "Nome")]
    pub nome: String,
    #[sqlx(rename = "Email")]
    pub email: String,
    #[sqlx(rename = "SenhaHash")]
    pub senha_hash: String,
    #[sqlx(rename = "PerfilId")]
    pub perfil_id: i32,
}

/// Resultado de JOIN entre Usuario e Perfil
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UsuarioComPerfil {
    pub id: i32,
    pub codigo: String,
    pub nome: String,
    pub email: String,
    pub senha_hash: String,
    pub perfil_id: i32,
    pub perfil_nome: String,
}
