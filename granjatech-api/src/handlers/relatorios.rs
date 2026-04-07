use actix_web::{web, HttpResponse};
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use serde::Deserialize;
use sqlx::PgPool;

use crate::dto::financeiro::*;
use crate::dto::relatorios::*;
use crate::errors::AppError;
use crate::middleware::jwt::Claims;
use crate::services::cache_service::CacheService;
use crate::services::relatorio_avancado_service::RelatorioAvancadoService;
use crate::services::relatorio_service::RelatorioService;

// === Flexible date parsing (matches .NET DateTime flexible binding) ===

fn parse_flexible_datetime(s: &str) -> Result<DateTime<Utc>, AppError> {
    // Try ISO 8601 full datetime first
    if let Ok(dt) = s.parse::<DateTime<Utc>>() {
        return Ok(dt);
    }
    // Try NaiveDateTime (no timezone)
    if let Ok(ndt) = NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S") {
        return Ok(ndt.and_utc());
    }
    // Try date-only (YYYY-MM-DD) — .NET accepts this
    if let Ok(nd) = NaiveDate::parse_from_str(s, "%Y-%m-%d") {
        return Ok(nd.and_hms_opt(0, 0, 0).unwrap().and_utc());
    }
    Err(AppError::BadRequest(format!(
        "Data invalida: '{}'. Use formato: 2025-01-01 ou 2025-01-01T00:00:00Z", s
    )))
}

// === Query parameter structs ===

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DateRangeQuery {
    pub data_inicio: String,
    pub data_fim: String,
    pub granja_id: Option<i32>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AviculturaReportQuery {
    pub data_inicio: Option<String>,
    pub data_fim: Option<String>,
    pub lote_id: Option<i32>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvancadoQuery {
    pub granja_id: i32,
    pub inicio: String,
    pub fim: String,
    pub tipo: Option<String>,
    pub setor: Option<String>,
}

// === Date validation ===

fn validate_date_range(inicio: &DateTime<Utc>, fim: &DateTime<Utc>) -> Result<(), AppError> {
    if inicio > fim {
        return Err(AppError::BadRequest(
            "A data de inicio nao pode ser posterior a data de fim.".into(),
        ));
    }
    let diff = (*fim - *inicio).num_days();
    if diff > 365 {
        return Err(AppError::BadRequest(
            "O periodo do relatorio nao pode exceder 365 dias.".into(),
        ));
    }
    Ok(())
}

// === Handlers ===

/// GET /api/relatorios/financeiro-simplificado -- RELA-02
#[utoipa::path(
    get,
    path = "/api/relatorios/financeiro-simplificado",
    params(
        ("dataInicio" = String, Query, description = "Data inicio (YYYY-MM-DD)"),
        ("dataFim" = String, Query, description = "Data fim (YYYY-MM-DD)"),
        ("granjaId" = Option<i32>, Query, description = "Filtro por granja")
    ),
    responses(
        (status = 200, description = "Relatorio financeiro simplificado", body = RelatorioFinanceiroSimplificadoDto),
        (status = 400, description = "Parametros invalidos"),
        (status = 401, description = "Nao autenticado")
    ),
    tag = "relatorios",
    security(("bearer_auth" = []))
)]
pub async fn get_financeiro_simplificado(
    pool: web::Data<PgPool>,
    claims: Claims,
    query: web::Query<DateRangeQuery>,
    cache: web::Data<CacheService>,
) -> Result<HttpResponse, AppError> {
    let inicio = parse_flexible_datetime(&query.data_inicio)?;
    let fim = parse_flexible_datetime(&query.data_fim)?;
    validate_date_range(&inicio, &fim)?;
    let user_id = claims.user_id()?;
    let cache_key = format!(
        "report_fin_simp_{}_{}_{}_{}",
        user_id,
        claims.role,
        inicio.format("%Y%m%d"),
        fim.format("%Y%m%d")
    );
    let ttl = std::time::Duration::from_secs(10 * 60); // 10 min per D-05

    let result = cache
        .get_or_set(
            &cache_key,
            || {
                let pool = pool.clone();
                let role = claims.role.clone();
                let granja_id = query.granja_id;
                async move {
                    RelatorioService::financeiro_simplificado(
                        &pool, user_id, &role, inicio, fim, granja_id,
                    )
                    .await
                }
            },
            ttl,
        )
        .await?;

    Ok(HttpResponse::Ok().json(result))
}

/// GET /api/relatorios/financeiro -- RELA-03
#[utoipa::path(
    get,
    path = "/api/relatorios/financeiro",
    params(
        ("dataInicio" = String, Query, description = "Data inicio (YYYY-MM-DD)"),
        ("dataFim" = String, Query, description = "Data fim (YYYY-MM-DD)"),
        ("granjaId" = Option<i32>, Query, description = "Filtro por granja")
    ),
    responses(
        (status = 200, description = "Relatorio financeiro completo", body = RelatorioFinanceiroDto),
        (status = 400, description = "Parametros invalidos"),
        (status = 401, description = "Nao autenticado")
    ),
    tag = "relatorios",
    security(("bearer_auth" = []))
)]
pub async fn get_financeiro(
    pool: web::Data<PgPool>,
    claims: Claims,
    query: web::Query<DateRangeQuery>,
    cache: web::Data<CacheService>,
) -> Result<HttpResponse, AppError> {
    let inicio = parse_flexible_datetime(&query.data_inicio)?;
    let fim = parse_flexible_datetime(&query.data_fim)?;
    validate_date_range(&inicio, &fim)?;
    let user_id = claims.user_id()?;
    let cache_key = format!(
        "report_fin_{}_{}_{}_{}",
        user_id,
        claims.role,
        inicio.format("%Y%m%d"),
        fim.format("%Y%m%d")
    );
    let ttl = std::time::Duration::from_secs(10 * 60); // 10 min per D-05

    let result = cache
        .get_or_set(
            &cache_key,
            || {
                let pool = pool.clone();
                let role = claims.role.clone();
                let granja_id = query.granja_id;
                async move {
                    RelatorioService::financeiro(&pool, user_id, &role, inicio, fim, granja_id)
                        .await
                }
            },
            ttl,
        )
        .await?;

    Ok(HttpResponse::Ok().json(result))
}

/// GET /api/relatorios/producao -- RELA-04
#[utoipa::path(
    get,
    path = "/api/relatorios/producao",
    params(
        ("dataInicio" = String, Query, description = "Data inicio (YYYY-MM-DD)"),
        ("dataFim" = String, Query, description = "Data fim (YYYY-MM-DD)"),
        ("granjaId" = Option<i32>, Query, description = "Filtro por granja")
    ),
    responses(
        (status = 200, description = "Relatorio de producao", body = RelatorioProducaoDto),
        (status = 400, description = "Parametros invalidos"),
        (status = 401, description = "Nao autenticado")
    ),
    tag = "relatorios",
    security(("bearer_auth" = []))
)]
pub async fn get_producao(
    pool: web::Data<PgPool>,
    claims: Claims,
    query: web::Query<DateRangeQuery>,
    cache: web::Data<CacheService>,
) -> Result<HttpResponse, AppError> {
    let inicio = parse_flexible_datetime(&query.data_inicio)?;
    let fim = parse_flexible_datetime(&query.data_fim)?;
    validate_date_range(&inicio, &fim)?;
    let user_id = claims.user_id()?;
    let cache_key = format!(
        "report_prod_{}_{}_{}_{}",
        user_id,
        claims.role,
        inicio.format("%Y%m%d"),
        fim.format("%Y%m%d")
    );
    let ttl = std::time::Duration::from_secs(10 * 60); // 10 min per D-05

    let result = cache
        .get_or_set(
            &cache_key,
            || {
                let pool = pool.clone();
                let role = claims.role.clone();
                let granja_id = query.granja_id;
                async move {
                    RelatorioService::producao(&pool, user_id, &role, inicio, fim, granja_id).await
                }
            },
            ttl,
        )
        .await?;

    Ok(HttpResponse::Ok().json(result))
}

/// GET /api/relatorios/avicultura -- RELA-05
#[utoipa::path(
    get,
    path = "/api/relatorios/avicultura",
    params(
        ("dataInicio" = Option<String>, Query, description = "Data inicio (default: now - 1 month)"),
        ("dataFim" = Option<String>, Query, description = "Data fim (default: now)"),
        ("loteId" = Option<i32>, Query, description = "Filtro por lote")
    ),
    responses(
        (status = 200, description = "Relatorio de avicultura", body = RelatorioAviculturaDto),
        (status = 400, description = "Parametros invalidos"),
        (status = 401, description = "Nao autenticado")
    ),
    tag = "relatorios",
    security(("bearer_auth" = []))
)]
pub async fn get_avicultura(
    pool: web::Data<PgPool>,
    _claims: Claims,
    query: web::Query<AviculturaReportQuery>,
    cache: web::Data<CacheService>,
) -> Result<HttpResponse, AppError> {
    let inicio = query.data_inicio.as_deref().map(parse_flexible_datetime).transpose()?;
    let fim = query.data_fim.as_deref().map(parse_flexible_datetime).transpose()?;
    // Validate date range only if both provided
    if let (Some(ref i), Some(ref f)) = (inicio, fim) {
        validate_date_range(i, f)?;
    }
    let cache_key = format!(
        "report_avic_{:?}_{:?}_{:?}",
        inicio.map(|d| d.format("%Y%m%d").to_string()),
        fim.map(|d| d.format("%Y%m%d").to_string()),
        query.lote_id
    );
    let ttl = std::time::Duration::from_secs(10 * 60); // 10 min per D-05

    let result = cache
        .get_or_set(
            &cache_key,
            || {
                let pool = pool.clone();
                let lote_id = query.lote_id;
                async move { RelatorioService::avicultura(&pool, inicio, fim, lote_id).await }
            },
            ttl,
        )
        .await?;

    Ok(HttpResponse::Ok().json(result))
}

/// GET /api/relatorios/desempenho-lote/{loteId} -- RELA-06
#[utoipa::path(
    get,
    path = "/api/relatorios/desempenho-lote/{loteId}",
    params(
        ("loteId" = i32, Path, description = "ID do lote")
    ),
    responses(
        (status = 200, description = "Relatorio de desempenho do lote", body = RelatorioDesempenhoLoteDto),
        (status = 404, description = "Lote nao encontrado"),
        (status = 401, description = "Nao autenticado")
    ),
    tag = "relatorios",
    security(("bearer_auth" = []))
)]
pub async fn get_desempenho_lote(
    pool: web::Data<PgPool>,
    _claims: Claims,
    path: web::Path<i32>,
    cache: web::Data<CacheService>,
) -> Result<HttpResponse, AppError> {
    let lote_id = path.into_inner();
    let cache_key = format!("report_desemp_{}", lote_id);
    let ttl = std::time::Duration::from_secs(10 * 60); // 10 min per D-05

    let result = cache
        .get_or_set(
            &cache_key,
            || {
                let pool = pool.clone();
                async move { RelatorioService::desempenho_lote(&pool, lote_id).await }
            },
            ttl,
        )
        .await?;

    Ok(HttpResponse::Ok().json(result))
}

/// GET /api/relatorios/avancado -- RELA-07
#[utoipa::path(
    get,
    path = "/api/relatorios/avancado",
    params(
        ("granjaId" = i32, Query, description = "ID da granja"),
        ("inicio" = String, Query, description = "Data inicio (YYYY-MM-DD)"),
        ("fim" = String, Query, description = "Data fim (YYYY-MM-DD)"),
        ("tipo" = Option<String>, Query, description = "Tipo: financeiro, geral, setor"),
        ("setor" = Option<String>, Query, description = "Setor: consumo, pesagem, sanitario, sensores")
    ),
    responses(
        (status = 200, description = "Relatorio avancado"),
        (status = 400, description = "Parametros invalidos"),
        (status = 404, description = "Granja nao encontrada"),
        (status = 401, description = "Nao autenticado")
    ),
    tag = "relatorios",
    security(("bearer_auth" = []))
)]
pub async fn get_avancado(
    pool: web::Data<PgPool>,
    _claims: Claims,
    query: web::Query<AvancadoQuery>,
) -> Result<HttpResponse, AppError> {
    let inicio = parse_flexible_datetime(&query.inicio)?;
    let fim = parse_flexible_datetime(&query.fim)?;
    if fim <= inicio {
        return Err(AppError::BadRequest(
            "Periodo invalido: data de fim deve ser posterior a data de inicio.".into(),
        ));
    }

    // Validate granja exists
    let granja_exists: Option<(i32,)> = sqlx::query_as(
        r#"SELECT "Id" FROM "Granjas" WHERE "Id" = $1"#,
    )
    .bind(query.granja_id)
    .fetch_optional(pool.get_ref())
    .await?;

    if granja_exists.is_none() {
        return Err(AppError::NotFound("Granja nao encontrada.".into()));
    }

    let tipo = query.tipo.as_deref().unwrap_or("financeiro");

    match tipo.to_lowercase().as_str() {
        "financeiro" => {
            let result = RelatorioAvancadoService::financeiro(
                &pool,
                query.granja_id,
                inicio,
                fim,
            )
            .await?;
            Ok(HttpResponse::Ok().json(result))
        }
        "geral" => {
            let result = RelatorioAvancadoService::geral(
                &pool,
                query.granja_id,
                inicio,
                fim,
            )
            .await?;
            Ok(HttpResponse::Ok().json(result))
        }
        "setor" => {
            let setor = query.setor.as_deref().unwrap_or("");
            if setor.is_empty() {
                return Err(AppError::BadRequest(
                    "Informe o setor para relatorios setoriais.".into(),
                ));
            }
            match setor.to_lowercase().as_str() {
                "consumo" => {
                    let result = RelatorioAvancadoService::consumo(
                        &pool,
                        query.granja_id,
                        inicio,
                        fim,
                    )
                    .await?;
                    Ok(HttpResponse::Ok().json(result))
                }
                "pesagem" => {
                    let result = RelatorioAvancadoService::pesagem(
                        &pool,
                        query.granja_id,
                        inicio,
                        fim,
                    )
                    .await?;
                    Ok(HttpResponse::Ok().json(result))
                }
                "sanitario" => {
                    let result = RelatorioAvancadoService::sanitario(
                        &pool,
                        query.granja_id,
                        inicio,
                        fim,
                    )
                    .await?;
                    Ok(HttpResponse::Ok().json(result))
                }
                "sensores" => {
                    let result = RelatorioAvancadoService::sensores(
                        &pool,
                        query.granja_id,
                        inicio,
                        fim,
                    )
                    .await?;
                    Ok(HttpResponse::Ok().json(result))
                }
                _ => Err(AppError::BadRequest(format!(
                    "Setor '{}' nao e valido. Use: consumo, pesagem, sanitario, sensores",
                    setor
                ))),
            }
        }
        _ => Err(AppError::BadRequest(format!(
            "Tipo '{}' nao e valido. Use: financeiro, geral, setor",
            tipo
        ))),
    }
}
