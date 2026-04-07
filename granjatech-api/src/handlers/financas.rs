use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use validator::Validate;

use crate::dto::financeiro::*;
use crate::errors::AppError;
use crate::middleware::jwt::Claims;
use crate::services::financas_service::FinancasService;

/// GET /api/financas -- Administrador + Financeiro only
#[utoipa::path(
    get,
    path = "/api/financas",
    responses(
        (status = 200, description = "Lista de transacoes", body = Vec<TransacaoSimplificadaDto>),
        (status = 401, description = "Nao autenticado"),
        (status = 403, description = "Acesso negado")
    ),
    tag = "financas",
    security(("bearer_auth" = []))
)]
pub async fn get_transacoes(
    pool: web::Data<PgPool>,
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    if claims.role != "Administrador" && claims.role != "Financeiro" {
        return Err(AppError::Forbidden("Acesso restrito a Administrador e Financeiro.".into()));
    }
    let user_id = claims.user_id()?;
    let transacoes = FinancasService::get_all(&pool, user_id, &claims.role).await?;
    Ok(HttpResponse::Ok().json(transacoes))
}

/// POST /api/financas -- Administrador + Financeiro only
#[utoipa::path(
    post,
    path = "/api/financas",
    request_body = CreateTransacaoDto,
    responses(
        (status = 201, description = "Transacao criada", body = TransacaoSimplificadaDto),
        (status = 400, description = "Dados invalidos"),
        (status = 401, description = "Nao autenticado"),
        (status = 403, description = "Acesso negado")
    ),
    tag = "financas",
    security(("bearer_auth" = []))
)]
pub async fn create_transacao(
    pool: web::Data<PgPool>,
    claims: Claims,
    body: web::Json<CreateTransacaoDto>,
) -> Result<HttpResponse, AppError> {
    if claims.role != "Administrador" && claims.role != "Financeiro" {
        return Err(AppError::Forbidden("Acesso restrito a Administrador e Financeiro.".into()));
    }
    body.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;
    let user_id = claims.user_id()?;
    let transacao =
        FinancasService::create(&pool, &body, user_id, &claims.role, &claims.email).await?;
    Ok(HttpResponse::Created().json(transacao))
}

/// PUT /api/financas/{id} -- Administrador + Financeiro only
#[utoipa::path(
    put,
    path = "/api/financas/{id}",
    params(("id" = i32, Path, description = "ID da transacao")),
    request_body = UpdateTransacaoDto,
    responses(
        (status = 204, description = "Transacao atualizada"),
        (status = 400, description = "Dados invalidos ou regra de negocio"),
        (status = 401, description = "Nao autenticado"),
        (status = 403, description = "Acesso negado"),
        (status = 404, description = "Transacao nao encontrada")
    ),
    tag = "financas",
    security(("bearer_auth" = []))
)]
pub async fn update_transacao(
    pool: web::Data<PgPool>,
    claims: Claims,
    path: web::Path<i32>,
    body: web::Json<UpdateTransacaoDto>,
) -> Result<HttpResponse, AppError> {
    if claims.role != "Administrador" && claims.role != "Financeiro" {
        return Err(AppError::Forbidden("Acesso restrito a Administrador e Financeiro.".into()));
    }
    body.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;
    let user_id = claims.user_id()?;
    let id = path.into_inner();
    FinancasService::update(&pool, id, &body, user_id, &claims.role, &claims.email).await?;
    Ok(HttpResponse::NoContent().finish())
}

/// DELETE /api/financas/{id} -- Administrador only
#[utoipa::path(
    delete,
    path = "/api/financas/{id}",
    params(("id" = i32, Path, description = "ID da transacao")),
    responses(
        (status = 204, description = "Transacao deletada"),
        (status = 401, description = "Nao autenticado"),
        (status = 403, description = "Acesso negado"),
        (status = 404, description = "Transacao nao encontrada")
    ),
    tag = "financas",
    security(("bearer_auth" = []))
)]
pub async fn delete_transacao(
    pool: web::Data<PgPool>,
    claims: Claims,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    if claims.role != "Administrador" {
        return Err(AppError::Forbidden("Apenas Administrador pode deletar transacoes.".into()));
    }
    let user_id = claims.user_id()?;
    let id = path.into_inner();
    FinancasService::delete(&pool, id, user_id, &claims.role, &claims.email).await?;
    Ok(HttpResponse::NoContent().finish())
}
