use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use validator::Validate;

use crate::dto::lote::*;
use crate::dto::sanitario::CreateRegistroMortalidadeDto;
use crate::errors::AppError;
use crate::middleware::jwt::Claims;
use crate::services::lote_service::LoteService;

/// GET /api/lotes -- Any authenticated user (role filtering in service)
#[utoipa::path(
    get,
    path = "/api/lotes",
    responses(
        (status = 200, description = "Lista de lotes", body = Vec<LoteResponseDto>),
        (status = 401, description = "Nao autenticado")
    ),
    tag = "lotes",
    security(("bearer_auth" = []))
)]
pub async fn get_lotes(
    pool: web::Data<PgPool>,
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    let user_id = claims.user_id()?;
    let lotes = LoteService::get_all(&pool, user_id, &claims.role).await?;
    Ok(HttpResponse::Ok().json(lotes))
}

/// GET /api/lotes/{id} -- Any authenticated user (access check in service)
#[utoipa::path(
    get,
    path = "/api/lotes/{id}",
    params(("id" = i32, Path, description = "ID do lote")),
    responses(
        (status = 200, description = "Detalhes do lote", body = LoteResponseDto),
        (status = 401, description = "Nao autenticado"),
        (status = 403, description = "Acesso negado"),
        (status = 404, description = "Lote nao encontrado")
    ),
    tag = "lotes",
    security(("bearer_auth" = []))
)]
pub async fn get_lote(
    pool: web::Data<PgPool>,
    claims: Claims,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let user_id = claims.user_id()?;
    let id = path.into_inner();
    let lote = LoteService::get_by_id(&pool, id, user_id, &claims.role).await?;
    Ok(HttpResponse::Ok().json(lote))
}

/// POST /api/lotes -- Financeiro blocked in service layer
#[utoipa::path(
    post,
    path = "/api/lotes",
    request_body = CreateLoteDto,
    responses(
        (status = 201, description = "Lote criado", body = LoteResponseDto),
        (status = 400, description = "Dados invalidos"),
        (status = 401, description = "Nao autenticado"),
        (status = 403, description = "Financeiro nao pode criar lotes")
    ),
    tag = "lotes",
    security(("bearer_auth" = []))
)]
pub async fn create_lote(
    pool: web::Data<PgPool>,
    claims: Claims,
    body: web::Json<CreateLoteDto>,
) -> Result<HttpResponse, AppError> {
    body.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;
    let user_id = claims.user_id()?;
    let lote = LoteService::create(&pool, &body, user_id, &claims.role, &claims.email).await?;
    Ok(HttpResponse::Created().json(lote))
}

/// PUT /api/lotes/{id} -- Financeiro blocked in service layer
#[utoipa::path(
    put,
    path = "/api/lotes/{id}",
    params(("id" = i32, Path, description = "ID do lote")),
    request_body = UpdateLoteDto,
    responses(
        (status = 200, description = "Lote atualizado", body = LoteResponseDto),
        (status = 400, description = "Dados invalidos"),
        (status = 401, description = "Nao autenticado"),
        (status = 403, description = "Acesso negado"),
        (status = 404, description = "Lote nao encontrado")
    ),
    tag = "lotes",
    security(("bearer_auth" = []))
)]
pub async fn update_lote(
    pool: web::Data<PgPool>,
    claims: Claims,
    path: web::Path<i32>,
    body: web::Json<UpdateLoteDto>,
) -> Result<HttpResponse, AppError> {
    body.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;
    let user_id = claims.user_id()?;
    let id = path.into_inner();
    let lote =
        LoteService::update(&pool, id, &body, user_id, &claims.role, &claims.email).await?;
    Ok(HttpResponse::Ok().json(lote))
}

/// DELETE /api/lotes/{id} -- Financeiro blocked in service layer
#[utoipa::path(
    delete,
    path = "/api/lotes/{id}",
    params(("id" = i32, Path, description = "ID do lote")),
    responses(
        (status = 204, description = "Lote deletado"),
        (status = 401, description = "Nao autenticado"),
        (status = 403, description = "Acesso negado"),
        (status = 404, description = "Lote nao encontrado")
    ),
    tag = "lotes",
    security(("bearer_auth" = []))
)]
pub async fn delete_lote(
    pool: web::Data<PgPool>,
    claims: Claims,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let user_id = claims.user_id()?;
    let id = path.into_inner();
    LoteService::delete(&pool, id, user_id, &claims.role, &claims.email).await?;
    Ok(HttpResponse::NoContent().finish())
}

/// POST /api/lotes/{id}/mortalidades -- Registra mortalidade no lote
#[utoipa::path(
    post,
    path = "/api/lotes/{id}/mortalidades",
    params(("id" = i32, Path, description = "ID do lote")),
    request_body = CreateRegistroMortalidadeDto,
    responses(
        (status = 201, description = "Mortalidade registrada", body = crate::dto::sanitario::RegistroMortalidadeDto),
        (status = 400, description = "Dados invalidos"),
        (status = 401, description = "Nao autenticado"),
        (status = 403, description = "Acesso negado"),
        (status = 404, description = "Lote nao encontrado")
    ),
    tag = "lotes",
    security(("bearer_auth" = []))
)]
pub async fn registrar_mortalidade(
    pool: web::Data<PgPool>,
    claims: Claims,
    path: web::Path<i32>,
    body: web::Json<CreateRegistroMortalidadeDto>,
) -> Result<HttpResponse, AppError> {
    body.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;
    let user_id = claims.user_id()?;
    let lote_id = path.into_inner();
    let mortalidade = LoteService::registrar_mortalidade(
        &pool,
        lote_id,
        &body,
        user_id,
        &claims.role,
        &claims.email,
    )
    .await?;
    Ok(HttpResponse::Created().json(mortalidade))
}

/// GET /api/lotes/{id}/mortalidades -- Lista mortalidades do lote
#[utoipa::path(
    get,
    path = "/api/lotes/{id}/mortalidades",
    params(("id" = i32, Path, description = "ID do lote")),
    responses(
        (status = 200, description = "Lista de mortalidades", body = Vec<crate::dto::sanitario::RegistroMortalidadeDto>),
        (status = 401, description = "Nao autenticado"),
        (status = 403, description = "Acesso negado"),
        (status = 404, description = "Lote nao encontrado")
    ),
    tag = "lotes",
    security(("bearer_auth" = []))
)]
pub async fn listar_mortalidades(
    pool: web::Data<PgPool>,
    claims: Claims,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let user_id = claims.user_id()?;
    let lote_id = path.into_inner();
    let mortalidades =
        LoteService::listar_mortalidades(&pool, lote_id, user_id, &claims.role).await?;
    Ok(HttpResponse::Ok().json(mortalidades))
}
