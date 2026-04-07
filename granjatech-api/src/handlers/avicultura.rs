use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::dto::avicultura::*;
use crate::errors::AppError;
use crate::middleware::jwt::Claims;
use crate::services::avicultura_service::AviculturaService;
use crate::services::cache_service::CacheService;

/// Role guard helper -- rejects Financeiro role (per D-09: Administrador+Produtor only)
fn require_admin_or_produtor(claims: &Claims) -> Result<(), AppError> {
    if claims.role != "Administrador" && claims.role != "Produtor" {
        return Err(AppError::Forbidden(
            "Acesso restrito a Administrador e Produtor".into(),
        ));
    }
    Ok(())
}

/// GET /api/avicultura/{loteId}/metricas -- Metricas principais do lote
#[utoipa::path(
    get,
    path = "/api/avicultura/{loteId}/metricas",
    responses(
        (status = 200, description = "Metricas do lote", body = MetricasLoteDto),
        (status = 401, description = "Nao autenticado"),
        (status = 403, description = "Acesso negado")
    ),
    params(
        ("loteId" = i32, Path, description = "ID do lote")
    ),
    tag = "avicultura",
    security(("bearer_auth" = []))
)]
pub async fn get_metricas(
    pool: web::Data<PgPool>,
    claims: Claims,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    require_admin_or_produtor(&claims)?;
    let lote_id = path.into_inner();
    let metricas = AviculturaService::get_metricas(&pool, lote_id).await?;
    Ok(HttpResponse::Ok().json(metricas))
}

/// GET /api/avicultura/{loteId}/analise-consumo -- Analise detalhada de consumo
#[utoipa::path(
    get,
    path = "/api/avicultura/{loteId}/analise-consumo",
    responses(
        (status = 200, description = "Analise de consumo", body = AnaliseConsumoDto),
        (status = 401, description = "Nao autenticado"),
        (status = 403, description = "Acesso negado")
    ),
    params(
        ("loteId" = i32, Path, description = "ID do lote")
    ),
    tag = "avicultura",
    security(("bearer_auth" = []))
)]
pub async fn get_analise_consumo(
    pool: web::Data<PgPool>,
    claims: Claims,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    require_admin_or_produtor(&claims)?;
    let lote_id = path.into_inner();
    let analise = AviculturaService::analise_consumo_detalhada(&pool, lote_id).await?;
    Ok(HttpResponse::Ok().json(analise))
}

/// GET /api/avicultura/{loteId}/curvas-crescimento -- Curvas de crescimento do lote
#[utoipa::path(
    get,
    path = "/api/avicultura/{loteId}/curvas-crescimento",
    responses(
        (status = 200, description = "Curvas de crescimento", body = CurvasCrescimentoDto),
        (status = 401, description = "Nao autenticado"),
        (status = 403, description = "Acesso negado")
    ),
    params(
        ("loteId" = i32, Path, description = "ID do lote")
    ),
    tag = "avicultura",
    security(("bearer_auth" = []))
)]
pub async fn get_curvas_crescimento(
    pool: web::Data<PgPool>,
    claims: Claims,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    require_admin_or_produtor(&claims)?;
    let lote_id = path.into_inner();
    let curvas = AviculturaService::obter_curvas_crescimento(&pool, lote_id).await?;
    Ok(HttpResponse::Ok().json(curvas))
}

/// GET /api/avicultura/{loteId}/resumo-sanitario -- Resumo sanitario do lote
#[utoipa::path(
    get,
    path = "/api/avicultura/{loteId}/resumo-sanitario",
    responses(
        (status = 200, description = "Resumo sanitario", body = ResumoSanitarioDto),
        (status = 401, description = "Nao autenticado"),
        (status = 403, description = "Acesso negado")
    ),
    params(
        ("loteId" = i32, Path, description = "ID do lote")
    ),
    tag = "avicultura",
    security(("bearer_auth" = []))
)]
pub async fn get_resumo_sanitario(
    pool: web::Data<PgPool>,
    claims: Claims,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    require_admin_or_produtor(&claims)?;
    let lote_id = path.into_inner();
    let resumo = AviculturaService::obter_resumo_sanitario(&pool, lote_id).await?;
    Ok(HttpResponse::Ok().json(resumo))
}

/// GET /api/avicultura/{loteId}/alertas -- Alertas de parametros fora do padrao
#[utoipa::path(
    get,
    path = "/api/avicultura/{loteId}/alertas",
    responses(
        (status = 200, description = "Lista de alertas", body = Vec<AlertaParametroDto>),
        (status = 401, description = "Nao autenticado"),
        (status = 403, description = "Acesso negado")
    ),
    params(
        ("loteId" = i32, Path, description = "ID do lote")
    ),
    tag = "avicultura",
    security(("bearer_auth" = []))
)]
pub async fn get_alertas(
    pool: web::Data<PgPool>,
    claims: Claims,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    require_admin_or_produtor(&claims)?;
    let lote_id = path.into_inner();
    let alertas = AviculturaService::verificar_alertas(&pool, lote_id).await?;
    Ok(HttpResponse::Ok().json(alertas))
}

/// GET /api/avicultura/{loteId}/comparacao-industria -- Comparacao com padroes da industria
#[utoipa::path(
    get,
    path = "/api/avicultura/{loteId}/comparacao-industria",
    responses(
        (status = 200, description = "Comparacao com industria", body = ComparacaoIndustriaDto),
        (status = 401, description = "Nao autenticado"),
        (status = 403, description = "Acesso negado")
    ),
    params(
        ("loteId" = i32, Path, description = "ID do lote")
    ),
    tag = "avicultura",
    security(("bearer_auth" = []))
)]
pub async fn get_comparacao_industria(
    pool: web::Data<PgPool>,
    claims: Claims,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    require_admin_or_produtor(&claims)?;
    let lote_id = path.into_inner();
    let comparacao = AviculturaService::comparar_com_industria(&pool, lote_id).await?;
    Ok(HttpResponse::Ok().json(comparacao))
}

/// GET /api/avicultura/{loteId}/projecao-abate -- Projecao para abate
#[utoipa::path(
    get,
    path = "/api/avicultura/{loteId}/projecao-abate",
    responses(
        (status = 200, description = "Projecao de abate", body = ProjecaoAbateDto),
        (status = 401, description = "Nao autenticado"),
        (status = 403, description = "Acesso negado")
    ),
    params(
        ("loteId" = i32, Path, description = "ID do lote")
    ),
    tag = "avicultura",
    security(("bearer_auth" = []))
)]
pub async fn get_projecao_abate(
    pool: web::Data<PgPool>,
    claims: Claims,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    require_admin_or_produtor(&claims)?;
    let lote_id = path.into_inner();
    let projecao = AviculturaService::calcular_projecao_abate(&pool, lote_id).await?;
    Ok(HttpResponse::Ok().json(projecao))
}

/// GET /api/avicultura/{loteId}/estimar-peso?dataAbate=... -- Estimativa de peso medio
#[utoipa::path(
    get,
    path = "/api/avicultura/{loteId}/estimar-peso",
    responses(
        (status = 200, description = "Estimativa de peso", body = EstimarPesoResponseDto),
        (status = 401, description = "Nao autenticado"),
        (status = 403, description = "Acesso negado")
    ),
    params(
        ("loteId" = i32, Path, description = "ID do lote"),
        ("dataAbate" = String, Query, description = "Data prevista de abate (ISO 8601)")
    ),
    tag = "avicultura",
    security(("bearer_auth" = []))
)]
pub async fn estimar_peso(
    pool: web::Data<PgPool>,
    claims: Claims,
    path: web::Path<i32>,
    query: web::Query<EstimarPesoQuery>,
) -> Result<HttpResponse, AppError> {
    require_admin_or_produtor(&claims)?;
    let lote_id = path.into_inner();
    let peso = AviculturaService::estimar_peso(&pool, lote_id, query.data_abate).await?;
    Ok(HttpResponse::Ok().json(EstimarPesoResponseDto {
        peso_estimado_gramas: peso,
        data_abate: query.data_abate,
    }))
}

/// GET /api/avicultura/{loteId}/dashboard -- Dashboard completo de avicultura
#[utoipa::path(
    get,
    path = "/api/avicultura/{loteId}/dashboard",
    responses(
        (status = 200, description = "Dashboard de avicultura", body = DashboardAviculturaDto),
        (status = 401, description = "Nao autenticado"),
        (status = 403, description = "Acesso negado")
    ),
    params(
        ("loteId" = i32, Path, description = "ID do lote")
    ),
    tag = "avicultura",
    security(("bearer_auth" = []))
)]
pub async fn get_dashboard(
    pool: web::Data<PgPool>,
    claims: Claims,
    path: web::Path<i32>,
    cache: web::Data<CacheService>,
) -> Result<HttpResponse, AppError> {
    require_admin_or_produtor(&claims)?;
    let lote_id = path.into_inner();
    let cache_key = format!("avicultura_dashboard_{}", lote_id);
    let ttl = std::time::Duration::from_secs(5 * 60); // 5 min per D-05

    let dashboard = cache
        .get_or_set(
            &cache_key,
            || {
                let pool = pool.clone();
                async move { AviculturaService::get_dashboard(&pool, lote_id).await }
            },
            ttl,
        )
        .await?;

    Ok(HttpResponse::Ok().json(dashboard))
}
