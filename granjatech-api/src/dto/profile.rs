use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

/// DTO de detalhes do perfil do usuario logado
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ProfileDetailDto {
    pub nome: String,
    pub email: String,
    pub perfil_nome: String,
    pub associados: Vec<String>,
}

/// DTO de atualizacao de perfil
#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProfileDto {
    #[validate(length(min = 2, max = 100))]
    pub nome: String,
    #[validate(email)]
    pub email: String,
}

/// DTO de troca de senha
#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ChangePasswordDto {
    #[validate(length(min = 1))]
    pub senha_atual: String,
    #[validate(length(min = 6))]
    pub nova_senha: String,
}
