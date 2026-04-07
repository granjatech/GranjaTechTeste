use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use validator::Validate;

use crate::dto::granja::*;
use crate::errors::AppError;
use crate::middleware::jwt::Claims;
use crate::services::granja_service::GranjaService;

/// GET /api/granjas -- Any authenticated user (role filtering in service)
#[utoipa::path(
    get,
    path = "/api/granjas",
    responses(
        (status = 200, description = "Lista de granjas", body = Vec<GranjaResponseDto>),
        (status = 401, description = "Nao autenticado")
    ),
    tag = "granjas",
    security(("bearer_auth" = []))
)]
pub async fn get_granjas(
    pool: web::Data<PgPool>,
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    let user_id = claims.user_id()?;
    let granjas = GranjaService::get_all(&pool, user_id, &claims.role).await?;
    Ok(HttpResponse::Ok().json(granjas))
}

/// GET /api/granjas/{id} -- Any authenticated user (access check in service)
#[utoipa::path(
    get,
    path = "/api/granjas/{id}",
    params(("id" = i32, Path, description = "ID da granja")),
    responses(
        (status = 200, description = "Detalhes da granja", body = GranjaResponseDto),
        (status = 401, description = "Nao autenticado"),
        (status = 403, description = "Acesso negado"),
        (status = 404, description = "Granja nao encontrada")
    ),
    tag = "granjas",
    security(("bearer_auth" = []))
)]
pub async fn get_granja(
    pool: web::Data<PgPool>,
    claims: Claims,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let user_id = claims.user_id()?;
    let id = path.into_inner();
    let granja = GranjaService::get_by_id(&pool, id, user_id, &claims.role).await?;
    Ok(HttpResponse::Ok().json(granja))
}

/// POST /api/granjas -- Financeiro blocked, returns 201
#[utoipa::path(
    post,
    path = "/api/granjas",
    request_body = CreateGranjaDto,
    responses(
        (status = 201, description = "Granja criada", body = GranjaResponseDto),
        (status = 400, description = "Dados invalidos"),
        (status = 401, description = "Nao autenticado"),
        (status = 403, description = "Financeiro nao pode criar granjas")
    ),
    tag = "granjas",
    security(("bearer_auth" = []))
)]
pub async fn create_granja(
    pool: web::Data<PgPool>,
    claims: Claims,
    body: web::Json<CreateGranjaDto>,
) -> Result<HttpResponse, AppError> {
    body.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;
    let user_id = claims.user_id()?;
    let granja =
        GranjaService::create(&pool, &body, user_id, &claims.role, &claims.email).await?;
    Ok(HttpResponse::Created().json(granja))
}

/// PUT /api/granjas/{id} -- Financeiro blocked
#[utoipa::path(
    put,
    path = "/api/granjas/{id}",
    params(("id" = i32, Path, description = "ID da granja")),
    request_body = UpdateGranjaDto,
    responses(
        (status = 200, description = "Granja atualizada", body = GranjaResponseDto),
        (status = 400, description = "Dados invalidos"),
        (status = 401, description = "Nao autenticado"),
        (status = 403, description = "Acesso negado"),
        (status = 404, description = "Granja nao encontrada")
    ),
    tag = "granjas",
    security(("bearer_auth" = []))
)]
pub async fn update_granja(
    pool: web::Data<PgPool>,
    claims: Claims,
    path: web::Path<i32>,
    body: web::Json<UpdateGranjaDto>,
) -> Result<HttpResponse, AppError> {
    body.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;
    let user_id = claims.user_id()?;
    let id = path.into_inner();
    let granja =
        GranjaService::update(&pool, id, &body, user_id, &claims.role, &claims.email).await?;
    Ok(HttpResponse::Ok().json(granja))
}

/// DELETE /api/granjas/{id} -- Financeiro blocked, returns 204
#[utoipa::path(
    delete,
    path = "/api/granjas/{id}",
    params(("id" = i32, Path, description = "ID da granja")),
    responses(
        (status = 204, description = "Granja deletada"),
        (status = 401, description = "Nao autenticado"),
        (status = 403, description = "Acesso negado"),
        (status = 404, description = "Granja nao encontrada")
    ),
    tag = "granjas",
    security(("bearer_auth" = []))
)]
pub async fn delete_granja(
    pool: web::Data<PgPool>,
    claims: Claims,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let user_id = claims.user_id()?;
    let id = path.into_inner();
    GranjaService::delete(&pool, id, user_id, &claims.role, &claims.email).await?;
    Ok(HttpResponse::NoContent().finish())
}
