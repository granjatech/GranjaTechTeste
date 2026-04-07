use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::dto::dashboard::*;
use crate::errors::AppError;
use crate::middleware::jwt::Claims;
use crate::services::dashboard_service::DashboardService;

/// GET /api/dashboard/kpis -- KPIs financeiros e lotes ativos
#[utoipa::path(
    get,
    path = "/api/dashboard/kpis",
    responses(
        (status = 200, description = "KPIs do dashboard", body = DashboardKpiDto),
        (status = 401, description = "Nao autenticado")
    ),
    tag = "dashboard",
    security(("bearer_auth" = []))
)]
pub async fn get_kpis(
    pool: web::Data<PgPool>,
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    let user_id = claims.user_id()?;
    let kpis = DashboardService::get_kpis(&pool, user_id, &claims.role).await?;
    Ok(HttpResponse::Ok().json(kpis))
}

/// GET /api/dashboard/resumo-mensal -- Resumo mensal financeiro
#[utoipa::path(
    get,
    path = "/api/dashboard/resumo-mensal",
    responses(
        (status = 200, description = "Resumo mensal", body = Vec<MonthlySummaryDto>),
        (status = 401, description = "Nao autenticado")
    ),
    tag = "dashboard",
    security(("bearer_auth" = []))
)]
pub async fn get_resumo_mensal(
    pool: web::Data<PgPool>,
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    let user_id = claims.user_id()?;
    let resumo = DashboardService::get_resumo_mensal(&pool, user_id, &claims.role).await?;
    Ok(HttpResponse::Ok().json(resumo))
}
