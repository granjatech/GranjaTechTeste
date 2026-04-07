use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::dto::dashboard::*;
use crate::errors::AppError;
use crate::middleware::jwt::Claims;
use crate::services::cache_service::CacheService;
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
    cache: web::Data<CacheService>,
) -> Result<HttpResponse, AppError> {
    let user_id = claims.user_id()?;
    let cache_key = format!("dashboard_kpis_{}_{}", user_id, claims.role);
    let ttl = std::time::Duration::from_secs(5 * 60); // 5 min per D-05

    let kpis = cache
        .get_or_set(
            &cache_key,
            || {
                let pool = pool.clone();
                let role = claims.role.clone();
                async move { DashboardService::get_kpis(&pool, user_id, &role).await }
            },
            ttl,
        )
        .await?;

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
    cache: web::Data<CacheService>,
) -> Result<HttpResponse, AppError> {
    let user_id = claims.user_id()?;
    let cache_key = format!("dashboard_resumo_mensal_{}_{}", user_id, claims.role);
    let ttl = std::time::Duration::from_secs(5 * 60); // 5 min per D-05

    let resumo: Vec<MonthlySummaryDto> = cache
        .get_or_set(
            &cache_key,
            || {
                let pool = pool.clone();
                let role = claims.role.clone();
                async move { DashboardService::get_resumo_mensal(&pool, user_id, &role).await }
            },
            ttl,
        )
        .await?;

    Ok(HttpResponse::Ok().json(resumo))
}
