use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use validator::Validate;

use crate::dto::pesagem::*;
use crate::errors::AppError;
use crate::middleware::jwt::Claims;
use crate::services::pesagem_service::PesagemService;

/// POST /api/pesagem -- Administrador + Produtor only
#[utoipa::path(
    post,
    path = "/api/pesagem",
    request_body = CreatePesagemSemanalDto,
    responses(
        (status = 201, description = "Pesagem registrada", body = PesagemSemanalResponseDto),
        (status = 400, description = "Dados invalidos"),
        (status = 401, description = "Nao autenticado"),
        (status = 403, description = "Acesso negado")
    ),
    tag = "pesagem",
    security(("bearer_auth" = []))
)]
pub async fn create_pesagem(
    pool: web::Data<PgPool>,
    claims: Claims,
    body: web::Json<CreatePesagemSemanalDto>,
) -> Result<HttpResponse, AppError> {
    if claims.role != "Administrador" && claims.role != "Produtor" {
        return Err(AppError::Forbidden("Acesso restrito a Administrador e Produtor.".into()));
    }
    body.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;
    let user_id = claims.user_id()?;
    let result = PesagemService::create(&pool, &body, user_id, &claims.email).await?;
    Ok(HttpResponse::Created().json(result))
}

/// GET /api/pesagem/{loteId} -- Administrador + Produtor only
#[utoipa::path(
    get,
    path = "/api/pesagem/{loteId}",
    params(("loteId" = i32, Path, description = "ID do lote")),
    responses(
        (status = 200, description = "Lista de pesagens", body = Vec<PesagemSemanalResponseDto>),
        (status = 401, description = "Nao autenticado"),
        (status = 403, description = "Acesso negado")
    ),
    tag = "pesagem",
    security(("bearer_auth" = []))
)]
pub async fn list_pesagens(
    pool: web::Data<PgPool>,
    claims: Claims,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    if claims.role != "Administrador" && claims.role != "Produtor" {
        return Err(AppError::Forbidden("Acesso restrito a Administrador e Produtor.".into()));
    }
    let lote_id = path.into_inner();
    let result = PesagemService::list(&pool, lote_id).await?;
    Ok(HttpResponse::Ok().json(result))
}

/// GET /api/pesagem/resumo/{loteId} -- Administrador + Produtor only
#[utoipa::path(
    get,
    path = "/api/pesagem/resumo/{loteId}",
    params(("loteId" = i32, Path, description = "ID do lote")),
    responses(
        (status = 200, description = "Resumo de pesagens do lote"),
        (status = 401, description = "Nao autenticado"),
        (status = 403, description = "Acesso negado")
    ),
    tag = "pesagem",
    security(("bearer_auth" = []))
)]
pub async fn resumo_pesagens(
    pool: web::Data<PgPool>,
    claims: Claims,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    if claims.role != "Administrador" && claims.role != "Produtor" {
        return Err(AppError::Forbidden("Acesso restrito a Administrador e Produtor.".into()));
    }
    let lote_id = path.into_inner();
    let result = PesagemService::resumo(&pool, lote_id).await?;
    Ok(HttpResponse::Ok().json(result))
}
