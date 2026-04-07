use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use validator::Validate;

use crate::dto::sensor::*;
use crate::errors::AppError;
use crate::middleware::jwt::Claims;
use crate::services::sensor_service::SensorService;

/// GET /api/sensores -- Lista sensores (Admin+Produtor)
#[utoipa::path(
    get,
    path = "/api/sensores",
    responses(
        (status = 200, description = "Lista de sensores", body = Vec<SensorResponseDto>),
        (status = 401, description = "Nao autenticado"),
        (status = 403, description = "Acesso negado")
    ),
    tag = "sensores",
    security(("bearer_auth" = []))
)]
pub async fn get_sensores(
    pool: web::Data<PgPool>,
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    if !["Administrador", "Produtor"].contains(&claims.role.as_str()) {
        return Err(AppError::Forbidden(
            "Acesso restrito a Administrador e Produtor.".into(),
        ));
    }
    let user_id = claims.user_id()?;
    let sensores = SensorService::list(&pool, user_id, &claims.role).await?;
    Ok(HttpResponse::Ok().json(sensores))
}

/// POST /api/sensores -- Cria sensor (Admin+Produtor)
#[utoipa::path(
    post,
    path = "/api/sensores",
    request_body = CreateSensorDto,
    responses(
        (status = 201, description = "Sensor criado", body = SensorResponseDto),
        (status = 400, description = "Dados invalidos"),
        (status = 401, description = "Nao autenticado"),
        (status = 403, description = "Acesso negado")
    ),
    tag = "sensores",
    security(("bearer_auth" = []))
)]
pub async fn create_sensor(
    pool: web::Data<PgPool>,
    claims: Claims,
    body: web::Json<CreateSensorDto>,
) -> Result<HttpResponse, AppError> {
    if !["Administrador", "Produtor"].contains(&claims.role.as_str()) {
        return Err(AppError::Forbidden(
            "Acesso restrito a Administrador e Produtor.".into(),
        ));
    }
    body.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;
    let user_id = claims.user_id()?;
    let sensor =
        SensorService::create(&pool, &body, user_id, &claims.role, &claims.email).await?;
    Ok(HttpResponse::Created().json(sensor))
}

/// DELETE /api/sensores/{id} -- Deleta sensor (Admin+Produtor)
#[utoipa::path(
    delete,
    path = "/api/sensores/{id}",
    params(("id" = i32, Path, description = "ID do sensor")),
    responses(
        (status = 204, description = "Sensor deletado"),
        (status = 401, description = "Nao autenticado"),
        (status = 403, description = "Acesso negado"),
        (status = 404, description = "Sensor nao encontrado")
    ),
    tag = "sensores",
    security(("bearer_auth" = []))
)]
pub async fn delete_sensor(
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
    SensorService::delete(&pool, id, user_id, &claims.role, &claims.email).await?;
    Ok(HttpResponse::NoContent().finish())
}

/// GET /api/sensores/{id}/leituras -- Lista leituras de um sensor (Admin+Produtor)
#[utoipa::path(
    get,
    path = "/api/sensores/{id}/leituras",
    params(("id" = i32, Path, description = "ID do sensor")),
    responses(
        (status = 200, description = "Lista de leituras", body = Vec<LeituraSensorResponseDto>),
        (status = 401, description = "Nao autenticado"),
        (status = 403, description = "Acesso negado"),
        (status = 404, description = "Sensor nao encontrado")
    ),
    tag = "sensores",
    security(("bearer_auth" = []))
)]
pub async fn get_leituras_sensor(
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
    let sensor_id = path.into_inner();
    let leituras = SensorService::list_leituras(&pool, sensor_id, user_id, &claims.role).await?;
    Ok(HttpResponse::Ok().json(leituras))
}
