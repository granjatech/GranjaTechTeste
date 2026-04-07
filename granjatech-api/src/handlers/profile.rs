use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use validator::Validate;

use crate::dto::profile::*;
use crate::errors::AppError;
use crate::middleware::jwt::Claims;
use crate::services::profile_service::ProfileService;

/// GET /api/profile -- Retorna detalhes do perfil do usuario logado
#[utoipa::path(
    get,
    path = "/api/profile",
    responses(
        (status = 200, description = "Detalhes do perfil", body = ProfileDetailDto),
        (status = 401, description = "Nao autenticado"),
        (status = 404, description = "Usuario nao encontrado")
    ),
    tag = "profile",
    security(("bearer_auth" = []))
)]
pub async fn get_profile(
    pool: web::Data<PgPool>,
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    let user_id = claims.user_id()?;
    let profile = ProfileService::get_profile(&pool, user_id).await?;
    Ok(HttpResponse::Ok().json(profile))
}

/// PUT /api/profile -- Atualiza nome e email do perfil
#[utoipa::path(
    put,
    path = "/api/profile",
    request_body = UpdateProfileDto,
    responses(
        (status = 200, description = "Perfil atualizado"),
        (status = 400, description = "Dados invalidos ou email duplicado"),
        (status = 401, description = "Nao autenticado")
    ),
    tag = "profile",
    security(("bearer_auth" = []))
)]
pub async fn update_profile(
    pool: web::Data<PgPool>,
    claims: Claims,
    body: web::Json<UpdateProfileDto>,
) -> Result<HttpResponse, AppError> {
    body.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;
    let user_id = claims.user_id()?;
    ProfileService::update_profile(&pool, user_id, &body, &claims.email).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({"message": "Perfil atualizado com sucesso"})))
}

/// POST /api/profile/change-password -- Altera senha do usuario
#[utoipa::path(
    post,
    path = "/api/profile/change-password",
    request_body = ChangePasswordDto,
    responses(
        (status = 200, description = "Senha alterada com sucesso"),
        (status = 400, description = "Senha atual incorreta"),
        (status = 401, description = "Nao autenticado")
    ),
    tag = "profile",
    security(("bearer_auth" = []))
)]
pub async fn change_password(
    pool: web::Data<PgPool>,
    claims: Claims,
    body: web::Json<ChangePasswordDto>,
) -> Result<HttpResponse, AppError> {
    body.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;
    let user_id = claims.user_id()?;
    ProfileService::change_password(
        &pool,
        user_id,
        &body.senha_atual,
        &body.nova_senha,
        &claims.email,
    )
    .await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({"message": "Senha alterada com sucesso"})))
}
