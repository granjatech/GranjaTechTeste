use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use validator::Validate;

use crate::config::Config;
use crate::dto::auth::*;
use crate::errors::AppError;
use crate::middleware::jwt::Claims;
use crate::services::auth_service::AuthService;
use crate::services::auditoria_service::AuditoriaService;

/// POST /api/auth/login -- AllowAnonymous
#[utoipa::path(
    post,
    path = "/api/auth/login",
    request_body = LoginDto,
    responses(
        (status = 200, description = "Login bem-sucedido", body = LoginResponseDto),
        (status = 400, description = "Dados invalidos"),
        (status = 401, description = "Credenciais invalidas")
    ),
    tag = "auth"
)]
pub async fn login(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    body: web::Json<LoginDto>,
) -> Result<HttpResponse, AppError> {
    body.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;
    let resultado = AuthService::login(&pool, &body, &config).await?;
    Ok(HttpResponse::Ok().json(resultado))
}

/// POST /api/auth/registrar -- AllowAnonymous
#[utoipa::path(
    post,
    path = "/api/auth/registrar",
    request_body = RegisterDto,
    responses(
        (status = 201, description = "Usuario criado", body = UserDto),
        (status = 400, description = "Dados invalidos ou email duplicado")
    ),
    tag = "auth"
)]
pub async fn registrar(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    body: web::Json<RegisterDto>,
) -> Result<HttpResponse, AppError> {
    body.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;
    let usuario = AuthService::registrar(&pool, &body, &config).await?;
    Ok(HttpResponse::Created().json(usuario))
}

/// GET /api/auth/usuarios -- Admin only
#[utoipa::path(
    get,
    path = "/api/auth/usuarios",
    responses(
        (status = 200, description = "Lista de usuarios", body = Vec<UserDto>),
        (status = 403, description = "Acesso restrito a administradores")
    ),
    tag = "auth",
    security(("bearer_auth" = []))
)]
pub async fn get_usuarios(
    pool: web::Data<PgPool>,
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    if claims.role != "Administrador" {
        return Err(AppError::Forbidden(
            "Acesso restrito a administradores".into(),
        ));
    }
    let usuarios = AuthService::get_all(&pool).await?;
    Ok(HttpResponse::Ok().json(usuarios))
}

/// GET /api/auth/usuarios/{id} -- Admin only
#[utoipa::path(
    get,
    path = "/api/auth/usuarios/{id}",
    params(("id" = i32, Path, description = "ID do usuario")),
    responses(
        (status = 200, description = "Detalhes do usuario", body = UserDetailDto),
        (status = 403, description = "Acesso restrito a administradores"),
        (status = 404, description = "Usuario nao encontrado")
    ),
    tag = "auth",
    security(("bearer_auth" = []))
)]
pub async fn get_usuario(
    pool: web::Data<PgPool>,
    claims: Claims,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    if claims.role != "Administrador" {
        return Err(AppError::Forbidden(
            "Acesso restrito a administradores".into(),
        ));
    }
    let id = path.into_inner();
    let usuario = AuthService::get_by_id(&pool, id).await?;
    Ok(HttpResponse::Ok().json(usuario))
}

/// PUT /api/auth/usuarios/{id} -- Admin only
#[utoipa::path(
    put,
    path = "/api/auth/usuarios/{id}",
    params(("id" = i32, Path, description = "ID do usuario")),
    request_body = UpdateUserDto,
    responses(
        (status = 200, description = "Usuario atualizado"),
        (status = 400, description = "Dados invalidos ou email duplicado"),
        (status = 403, description = "Acesso restrito a administradores"),
        (status = 404, description = "Usuario nao encontrado")
    ),
    tag = "auth",
    security(("bearer_auth" = []))
)]
pub async fn update_usuario(
    pool: web::Data<PgPool>,
    claims: Claims,
    path: web::Path<i32>,
    body: web::Json<UpdateUserDto>,
) -> Result<HttpResponse, AppError> {
    if claims.role != "Administrador" {
        return Err(AppError::Forbidden(
            "Acesso restrito a administradores".into(),
        ));
    }
    body.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;
    let id = path.into_inner();
    let admin_id = claims.user_id()?;
    AuthService::update(&pool, id, &body, admin_id, &claims.email).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({"message": "Usuario atualizado com sucesso"})))
}

/// DELETE /api/auth/usuarios/{id} -- Admin only
#[utoipa::path(
    delete,
    path = "/api/auth/usuarios/{id}",
    params(("id" = i32, Path, description = "ID do usuario")),
    responses(
        (status = 204, description = "Usuario deletado"),
        (status = 400, description = "Usuario possui dependencias"),
        (status = 403, description = "Acesso restrito a administradores"),
        (status = 404, description = "Usuario nao encontrado")
    ),
    tag = "auth",
    security(("bearer_auth" = []))
)]
pub async fn delete_usuario(
    pool: web::Data<PgPool>,
    claims: Claims,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    if claims.role != "Administrador" {
        return Err(AppError::Forbidden(
            "Acesso restrito a administradores".into(),
        ));
    }
    let id = path.into_inner();
    let admin_id = claims.user_id()?;
    AuthService::delete(&pool, id, admin_id, &claims.email).await?;

    AuditoriaService::registrar_log(
        &pool,
        admin_id,
        &claims.email,
        "DELECAO_USUARIO",
        &format!("Admin deletou usuario ID: {}", id),
    )
    .await
    .ok(); // Auditoria ja feita no service, log extra nao bloqueia

    Ok(HttpResponse::NoContent().finish())
}
