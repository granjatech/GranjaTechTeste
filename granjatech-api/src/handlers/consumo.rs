use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use validator::Validate;

use crate::dto::consumo::*;
use crate::errors::AppError;
use crate::middleware::jwt::Claims;
use crate::services::consumo_service::ConsumoService;

/// POST /api/consumo/racao -- Administrador + Produtor only
#[utoipa::path(
    post,
    path = "/api/consumo/racao",
    request_body = CreateConsumoRacaoDto,
    responses(
        (status = 201, description = "Consumo de racao registrado", body = ConsumoRacaoResponseDto),
        (status = 400, description = "Dados invalidos"),
        (status = 401, description = "Nao autenticado"),
        (status = 403, description = "Acesso negado")
    ),
    tag = "consumo",
    security(("bearer_auth" = []))
)]
pub async fn create_consumo_racao(
    pool: web::Data<PgPool>,
    claims: Claims,
    body: web::Json<CreateConsumoRacaoDto>,
) -> Result<HttpResponse, AppError> {
    if claims.role != "Administrador" && claims.role != "Produtor" {
        return Err(AppError::Forbidden("Acesso restrito a Administrador e Produtor.".into()));
    }
    body.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;
    let user_id = claims.user_id()?;
    let result = ConsumoService::create_racao(&pool, &body, user_id, &claims.email).await?;
    Ok(HttpResponse::Created().json(result))
}

/// POST /api/consumo/agua -- Administrador + Produtor only
#[utoipa::path(
    post,
    path = "/api/consumo/agua",
    request_body = CreateConsumoAguaDto,
    responses(
        (status = 201, description = "Consumo de agua registrado", body = ConsumoAguaResponseDto),
        (status = 400, description = "Dados invalidos"),
        (status = 401, description = "Nao autenticado"),
        (status = 403, description = "Acesso negado")
    ),
    tag = "consumo",
    security(("bearer_auth" = []))
)]
pub async fn create_consumo_agua(
    pool: web::Data<PgPool>,
    claims: Claims,
    body: web::Json<CreateConsumoAguaDto>,
) -> Result<HttpResponse, AppError> {
    if claims.role != "Administrador" && claims.role != "Produtor" {
        return Err(AppError::Forbidden("Acesso restrito a Administrador e Produtor.".into()));
    }
    body.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;
    let user_id = claims.user_id()?;
    let result = ConsumoService::create_agua(&pool, &body, user_id, &claims.email).await?;
    Ok(HttpResponse::Created().json(result))
}

/// GET /api/consumo/racao/{loteId} -- Administrador + Produtor only
#[utoipa::path(
    get,
    path = "/api/consumo/racao/{loteId}",
    params(("loteId" = i32, Path, description = "ID do lote")),
    responses(
        (status = 200, description = "Lista de consumos de racao", body = Vec<ConsumoRacaoResponseDto>),
        (status = 401, description = "Nao autenticado"),
        (status = 403, description = "Acesso negado")
    ),
    tag = "consumo",
    security(("bearer_auth" = []))
)]
pub async fn list_consumo_racao(
    pool: web::Data<PgPool>,
    claims: Claims,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    if claims.role != "Administrador" && claims.role != "Produtor" {
        return Err(AppError::Forbidden("Acesso restrito a Administrador e Produtor.".into()));
    }
    let lote_id = path.into_inner();
    let result = ConsumoService::list_racao(&pool, lote_id).await?;
    Ok(HttpResponse::Ok().json(result))
}

/// GET /api/consumo/agua/{loteId} -- Administrador + Produtor only
#[utoipa::path(
    get,
    path = "/api/consumo/agua/{loteId}",
    params(("loteId" = i32, Path, description = "ID do lote")),
    responses(
        (status = 200, description = "Lista de consumos de agua", body = Vec<ConsumoAguaResponseDto>),
        (status = 401, description = "Nao autenticado"),
        (status = 403, description = "Acesso negado")
    ),
    tag = "consumo",
    security(("bearer_auth" = []))
)]
pub async fn list_consumo_agua(
    pool: web::Data<PgPool>,
    claims: Claims,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    if claims.role != "Administrador" && claims.role != "Produtor" {
        return Err(AppError::Forbidden("Acesso restrito a Administrador e Produtor.".into()));
    }
    let lote_id = path.into_inner();
    let result = ConsumoService::list_agua(&pool, lote_id).await?;
    Ok(HttpResponse::Ok().json(result))
}

/// GET /api/consumo/resumo/{loteId} -- Administrador + Produtor only
#[utoipa::path(
    get,
    path = "/api/consumo/resumo/{loteId}",
    params(("loteId" = i32, Path, description = "ID do lote")),
    responses(
        (status = 200, description = "Resumo de consumo do lote"),
        (status = 401, description = "Nao autenticado"),
        (status = 403, description = "Acesso negado")
    ),
    tag = "consumo",
    security(("bearer_auth" = []))
)]
pub async fn resumo_consumo(
    pool: web::Data<PgPool>,
    claims: Claims,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    if claims.role != "Administrador" && claims.role != "Produtor" {
        return Err(AppError::Forbidden("Acesso restrito a Administrador e Produtor.".into()));
    }
    let lote_id = path.into_inner();
    let result = ConsumoService::resumo(&pool, lote_id).await?;
    Ok(HttpResponse::Ok().json(result))
}
