use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::errors::AppError;
use crate::middleware::jwt::Claims;
use crate::models::log_auditoria::LogAuditoria;

/// GET /api/auditoria -- Lista logs de auditoria (somente Administrador)
#[utoipa::path(
    get,
    path = "/api/auditoria",
    responses(
        (status = 200, description = "Lista de logs de auditoria", body = Vec<LogAuditoria>),
        (status = 401, description = "Nao autenticado"),
        (status = 403, description = "Acesso restrito a Administrador")
    ),
    tag = "auditoria",
    security(("bearer_auth" = []))
)]
pub async fn get_logs(
    pool: web::Data<PgPool>,
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    if claims.role != "Administrador" {
        return Err(AppError::Forbidden(
            "Acesso restrito a Administrador.".into(),
        ));
    }

    let logs = sqlx::query_as::<_, LogAuditoria>(
        r#"SELECT "Id", "Timestamp", "UsuarioId", "UsuarioEmail", "Acao", "Detalhes"
           FROM "LogsAuditoria"
           ORDER BY "Timestamp" DESC"#,
    )
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(logs))
}
