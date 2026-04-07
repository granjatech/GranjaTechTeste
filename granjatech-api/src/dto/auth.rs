use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

/// DTO de login
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct LoginDto {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1))]
    pub senha: String,
}

/// DTO de registro de novo usuario
#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterDto {
    #[validate(length(min = 2, max = 100))]
    pub nome: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6))]
    pub senha: String,
    pub perfil_id: i32,
    pub produtores_ids: Option<Vec<i32>>,
}

/// Resposta de login com token JWT
#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponseDto {
    pub token: String,
}

/// DTO de usuario para listagem
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserDto {
    pub id: i32,
    pub codigo: String,
    pub nome: String,
    pub email: String,
    pub perfil_id: i32,
    pub perfil_nome: String,
}

/// DTO de usuario com detalhes (inclui produtores associados)
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserDetailDto {
    pub id: i32,
    pub codigo: String,
    pub nome: String,
    pub email: String,
    pub perfil_id: i32,
    pub perfil_nome: String,
    pub produtores_ids: Vec<i32>,
}

/// DTO de atualizacao de usuario (admin)
#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserDto {
    #[validate(length(min = 2, max = 100))]
    pub nome: String,
    #[validate(email)]
    pub email: String,
    pub perfil_id: i32,
    pub senha: Option<String>,
    pub produtores_ids: Option<Vec<i32>>,
}

/// DTO de alteracao de senha
#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ChangePasswordDto {
    #[validate(length(min = 1))]
    pub senha_atual: String,
    #[validate(length(min = 6))]
    pub nova_senha: String,
}
