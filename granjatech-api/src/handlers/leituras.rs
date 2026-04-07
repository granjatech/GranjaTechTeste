use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use validator::Validate;

use crate::dto::sensor::CreateLeituraDto;
use crate::errors::AppError;
use crate::services::sensor_service::SensorService;

/// POST /api/leituras -- Registra leitura de sensor (PUBLICO - sem autenticacao)
/// Endpoint para dispositivos IoT enviarem dados sem necessidade de token JWT.
#[utoipa::path(
    post,
    path = "/api/leituras",
    request_body = CreateLeituraDto,
    responses(
        (status = 201, description = "Leitura registrada", body = crate::dto::sensor::LeituraSensorResponseDto),
        (status = 400, description = "Dados invalidos"),
        (status = 404, description = "Sensor nao encontrado")
    ),
    tag = "leituras"
)]
pub async fn post_leitura(
    pool: web::Data<PgPool>,
    body: web::Json<CreateLeituraDto>,
) -> Result<HttpResponse, AppError> {
    body.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;
    let leitura = SensorService::registrar_leitura(&pool, &body).await?;
    Ok(HttpResponse::Created().json(leitura))
}
