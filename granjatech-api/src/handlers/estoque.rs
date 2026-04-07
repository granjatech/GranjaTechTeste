use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use validator::Validate;

use crate::dto::estoque::*;
use crate::errors::AppError;
use crate::middleware::jwt::Claims;
use crate::services::estoque_service::EstoqueService;

/// GET /api/estoque -- Lista produtos em estoque (Admin+Produtor)
#[utoipa::path(
    get,
    path = "/api/estoque",
    responses(
        (status = 200, description = "Lista de produtos", body = Vec<ProdutoResponseDto>),
        (status = 401, description = "Nao autenticado"),
        (status = 403, description = "Acesso negado")
    ),
    tag = "estoque",
    security(("bearer_auth" = []))
)]
pub async fn get_produtos(
    pool: web::Data<PgPool>,
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    if !["Administrador", "Produtor"].contains(&claims.role.as_str()) {
        return Err(AppError::Forbidden(
            "Acesso restrito a Administrador e Produtor.".into(),
        ));
    }
    let user_id = claims.user_id()?;
    let produtos = EstoqueService::list(&pool, user_id, &claims.role).await?;
    Ok(HttpResponse::Ok().json(produtos))
}

/// POST /api/estoque -- Cria produto em estoque (Admin+Produtor)
#[utoipa::path(
    post,
    path = "/api/estoque",
    request_body = CreateProdutoDto,
    responses(
        (status = 201, description = "Produto criado", body = ProdutoResponseDto),
        (status = 400, description = "Dados invalidos"),
        (status = 401, description = "Nao autenticado"),
        (status = 403, description = "Acesso negado")
    ),
    tag = "estoque",
    security(("bearer_auth" = []))
)]
pub async fn create_produto(
    pool: web::Data<PgPool>,
    claims: Claims,
    body: web::Json<CreateProdutoDto>,
) -> Result<HttpResponse, AppError> {
    if !["Administrador", "Produtor"].contains(&claims.role.as_str()) {
        return Err(AppError::Forbidden(
            "Acesso restrito a Administrador e Produtor.".into(),
        ));
    }
    body.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;
    let user_id = claims.user_id()?;
    let produto =
        EstoqueService::create(&pool, &body, user_id, &claims.role, &claims.email).await?;
    Ok(HttpResponse::Created().json(produto))
}

/// PUT /api/estoque/{id} -- Atualiza produto em estoque (Admin+Produtor)
#[utoipa::path(
    put,
    path = "/api/estoque/{id}",
    params(("id" = i32, Path, description = "ID do produto")),
    request_body = UpdateProdutoDto,
    responses(
        (status = 200, description = "Produto atualizado", body = ProdutoResponseDto),
        (status = 400, description = "Dados invalidos"),
        (status = 401, description = "Nao autenticado"),
        (status = 403, description = "Acesso negado"),
        (status = 404, description = "Produto nao encontrado")
    ),
    tag = "estoque",
    security(("bearer_auth" = []))
)]
pub async fn update_produto(
    pool: web::Data<PgPool>,
    claims: Claims,
    path: web::Path<i32>,
    body: web::Json<UpdateProdutoDto>,
) -> Result<HttpResponse, AppError> {
    if !["Administrador", "Produtor"].contains(&claims.role.as_str()) {
        return Err(AppError::Forbidden(
            "Acesso restrito a Administrador e Produtor.".into(),
        ));
    }
    body.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;
    let user_id = claims.user_id()?;
    let id = path.into_inner();
    let produto =
        EstoqueService::update(&pool, id, &body, user_id, &claims.role, &claims.email).await?;
    Ok(HttpResponse::Ok().json(produto))
}

/// DELETE /api/estoque/{id} -- Deleta produto em estoque (Admin+Produtor)
#[utoipa::path(
    delete,
    path = "/api/estoque/{id}",
    params(("id" = i32, Path, description = "ID do produto")),
    responses(
        (status = 204, description = "Produto deletado"),
        (status = 401, description = "Nao autenticado"),
        (status = 403, description = "Acesso negado"),
        (status = 404, description = "Produto nao encontrado")
    ),
    tag = "estoque",
    security(("bearer_auth" = []))
)]
pub async fn delete_produto(
    pool: web::Data<PgPool>,
    claims: Claims,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    if !["Administrador", "Produtor"].contains(&claims.role.as_str()) {
        return Err(AppError::Forbidden(
            "Acesso restrito a Administrador e Produtor.".into(),
        ));
    }
    let user_id = claims.user_id()?;
    let id = path.into_inner();
    EstoqueService::delete(&pool, id, user_id, &claims.role, &claims.email).await?;
    Ok(HttpResponse::NoContent().finish())
}
