use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use validator::Validate;

use crate::dto::sanitario::CreateEventoSanitarioDto;
use crate::errors::AppError;
use crate::middleware::jwt::Claims;
use crate::services::sanitario_service::SanitarioService;

/// Query params for sanitario list
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SanitarioQuery {
    pub tipo_evento: Option<String>,
}

/// POST /api/sanitario -- Administrador + Produtor only
#[utoipa::path(
    post,
    path = "/api/sanitario",
    request_body = CreateEventoSanitarioDto,
    responses(
        (status = 201, description = "Evento sanitario registrado"),
        (status = 400, description = "Dados invalidos"),
        (status = 401, description = "Nao autenticado"),
        (status = 403, description = "Acesso negado")
    ),
    tag = "sanitario",
    security(("bearer_auth" = []))
)]
pub async fn create_evento(
    pool: web::Data<PgPool>,
    claims: Claims,
    body: web::Json<CreateEventoSanitarioDto>,
) -> Result<HttpResponse, AppError> {
    if claims.role != "Administrador" && claims.role != "Produtor" {
        return Err(AppError::Forbidden("Acesso restrito a Administrador e Produtor.".into()));
    }
    body.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;
    let user_id = claims.user_id()?;
    let result = SanitarioService::create(&pool, &body, user_id, &claims.email).await?;
    Ok(HttpResponse::Created().json(result))
}

/// GET /api/sanitario/{loteId} -- Administrador + Produtor only
#[utoipa::path(
    get,
    path = "/api/sanitario/{loteId}",
    params(
        ("loteId" = i32, Path, description = "ID do lote"),
    ),
    responses(
        (status = 200, description = "Lista de eventos sanitarios"),
        (status = 401, description = "Nao autenticado"),
        (status = 403, description = "Acesso negado")
    ),
    tag = "sanitario",
    security(("bearer_auth" = []))
)]
pub async fn list_eventos(
    pool: web::Data<PgPool>,
    claims: Claims,
    path: web::Path<i32>,
    query: web::Query<SanitarioQuery>,
) -> Result<HttpResponse, AppError> {
    if claims.role != "Administrador" && claims.role != "Produtor" {
        return Err(AppError::Forbidden("Acesso restrito a Administrador e Produtor.".into()));
    }
    let lote_id = path.into_inner();
    let result =
        SanitarioService::list(&pool, lote_id, query.tipo_evento.as_deref()).await?;
    Ok(HttpResponse::Ok().json(result))
}

/// GET /api/sanitario/resumo/{loteId} -- Administrador + Produtor only
#[utoipa::path(
    get,
    path = "/api/sanitario/resumo/{loteId}",
    params(("loteId" = i32, Path, description = "ID do lote")),
    responses(
        (status = 200, description = "Resumo sanitario do lote"),
        (status = 401, description = "Nao autenticado"),
        (status = 403, description = "Acesso negado")
    ),
    tag = "sanitario",
    security(("bearer_auth" = []))
)]
pub async fn resumo_sanitario(
    pool: web::Data<PgPool>,
    claims: Claims,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    if claims.role != "Administrador" && claims.role != "Produtor" {
        return Err(AppError::Forbidden("Acesso restrito a Administrador e Produtor.".into()));
    }
    let lote_id = path.into_inner();
    let result = SanitarioService::resumo(&pool, lote_id).await?;
    Ok(HttpResponse::Ok().json(result))
}

/// GET /api/sanitario/cronograma-vacinacao -- Any authenticated user
#[utoipa::path(
    get,
    path = "/api/sanitario/cronograma-vacinacao",
    responses(
        (status = 200, description = "Cronograma padrao de vacinacao"),
        (status = 401, description = "Nao autenticado")
    ),
    tag = "sanitario",
    security(("bearer_auth" = []))
)]
pub async fn cronograma_vacinacao(
    _claims: Claims,
) -> Result<HttpResponse, AppError> {
    let cronograma = SanitarioService::cronograma_vacinacao();
    Ok(HttpResponse::Ok().json(cronograma))
}
