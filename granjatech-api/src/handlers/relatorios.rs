use actix_web::{web, HttpResponse};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use sqlx::PgPool;

use crate::dto::financeiro::*;
use crate::dto::relatorios::*;
use crate::errors::AppError;
use crate::middleware::jwt::Claims;
use crate::services::cache_service::CacheService;
use crate::services::relatorio_avancado_service::RelatorioAvancadoService;
use crate::services::relatorio_service::RelatorioService;

// === Query parameter structs ===

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DateRangeQuery {
    pub data_inicio: DateTime<Utc>,
    pub data_fim: DateTime<Utc>,
    pub granja_id: Option<i32>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AviculturaReportQuery {
    pub data_inicio: Option<DateTime<Utc>>,
    pub data_fim: Option<DateTime<Utc>>,
    pub lote_id: Option<i32>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvancadoQuery {
    pub granja_id: i32,
    pub inicio: DateTime<Utc>,
    pub fim: DateTime<Utc>,
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
        ("dataInicio" = DateTime<Utc>, Query, description = "Data inicio"),
        ("dataFim" = DateTime<Utc>, Query, description = "Data fim"),
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
    validate_date_range(&query.data_inicio, &query.data_fim)?;
    let user_id = claims.user_id()?;
    let cache_key = format!(
        "report_fin_simp_{}_{}_{}_{}",
        user_id,
        claims.role,
        query.data_inicio.format("%Y%m%d"),
        query.data_fim.format("%Y%m%d")
    );
    let ttl = std::time::Duration::from_secs(10 * 60); // 10 min per D-05

    let result = cache
        .get_or_set(
            &cache_key,
            || {
                let pool = pool.clone();
                let role = claims.role.clone();
                let granja_id = query.granja_id;
                let inicio = query.data_inicio;
                let fim = query.data_fim;
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
        ("dataInicio" = DateTime<Utc>, Query, description = "Data inicio"),
        ("dataFim" = DateTime<Utc>, Query, description = "Data fim"),
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
    validate_date_range(&query.data_inicio, &query.data_fim)?;
    let user_id = claims.user_id()?;
    let cache_key = format!(
        "report_fin_{}_{}_{}_{}",
        user_id,
        claims.role,
        query.data_inicio.format("%Y%m%d"),
        query.data_fim.format("%Y%m%d")
    );
    let ttl = std::time::Duration::from_secs(10 * 60); // 10 min per D-05

    let result = cache
        .get_or_set(
            &cache_key,
            || {
                let pool = pool.clone();
                let role = claims.role.clone();
                let granja_id = query.granja_id;
                let inicio = query.data_inicio;
                let fim = query.data_fim;
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
        ("dataInicio" = DateTime<Utc>, Query, description = "Data inicio"),
        ("dataFim" = DateTime<Utc>, Query, description = "Data fim"),
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
    validate_date_range(&query.data_inicio, &query.data_fim)?;
    let user_id = claims.user_id()?;
    let cache_key = format!(
        "report_prod_{}_{}_{}_{}",
        user_id,
        claims.role,
        query.data_inicio.format("%Y%m%d"),
        query.data_fim.format("%Y%m%d")
    );
    let ttl = std::time::Duration::from_secs(10 * 60); // 10 min per D-05

    let result = cache
        .get_or_set(
            &cache_key,
            || {
                let pool = pool.clone();
                let role = claims.role.clone();
                let granja_id = query.granja_id;
                let inicio = query.data_inicio;
                let fim = query.data_fim;
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
        ("dataInicio" = Option<DateTime<Utc>>, Query, description = "Data inicio (default: now - 1 month)"),
        ("dataFim" = Option<DateTime<Utc>>, Query, description = "Data fim (default: now)"),
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
    // Validate date range only if both provided
    if let (Some(ref inicio), Some(ref fim)) = (query.data_inicio, query.data_fim) {
        validate_date_range(inicio, fim)?;
    }
    let cache_key = format!(
        "report_avic_{:?}_{:?}_{:?}",
        query.data_inicio.map(|d| d.format("%Y%m%d").to_string()),
        query.data_fim.map(|d| d.format("%Y%m%d").to_string()),
        query.lote_id
    );
    let ttl = std::time::Duration::from_secs(10 * 60); // 10 min per D-05

    let result = cache
        .get_or_set(
            &cache_key,
            || {
                let pool = pool.clone();
                let inicio = query.data_inicio;
                let fim = query.data_fim;
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
        ("inicio" = DateTime<Utc>, Query, description = "Data inicio"),
        ("fim" = DateTime<Utc>, Query, description = "Data fim"),
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
    if query.fim <= query.inicio {
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
                query.inicio,
                query.fim,
            )
            .await?;
            Ok(HttpResponse::Ok().json(result))
        }
        "geral" => {
            let result = RelatorioAvancadoService::geral(
                &pool,
                query.granja_id,
                query.inicio,
                query.fim,
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
                        query.inicio,
                        query.fim,
                    )
                    .await?;
                    Ok(HttpResponse::Ok().json(result))
                }
                "pesagem" => {
                    let result = RelatorioAvancadoService::pesagem(
                        &pool,
                        query.granja_id,
                        query.inicio,
                        query.fim,
                    )
                    .await?;
                    Ok(HttpResponse::Ok().json(result))
                }
                "sanitario" => {
                    let result = RelatorioAvancadoService::sanitario(
                        &pool,
                        query.granja_id,
                        query.inicio,
                        query.fim,
                    )
                    .await?;
                    Ok(HttpResponse::Ok().json(result))
                }
                "sensores" => {
                    let result = RelatorioAvancadoService::sensores(
                        &pool,
                        query.granja_id,
                        query.inicio,
                        query.fim,
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
