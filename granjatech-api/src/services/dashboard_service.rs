use rust_decimal::Decimal;
use sqlx::PgPool;

use crate::dto::dashboard::*;
use crate::errors::AppError;

/// Helper struct para resultado da query de agregacao financeira
#[derive(Debug, sqlx::FromRow)]
struct FinancialAggregation {
    total_entradas: Option<Decimal>,
    total_saidas: Option<Decimal>,
}

/// Helper struct para resultado do resumo mensal
#[derive(Debug, sqlx::FromRow)]
struct MonthlyRaw {
    ano: Option<f64>,
    mes: Option<f64>,
    entradas: Option<Decimal>,
    saidas: Option<Decimal>,
}

pub struct DashboardService;

/// Meses em pt-BR
const MESES_PT_BR: [&str; 12] = [
    "jan", "fev", "mar", "abr", "mai", "jun",
    "jul", "ago", "set", "out", "nov", "dez",
];

impl DashboardService {
    /// GET /api/dashboard/kpis -- KPIs financeiros e lotes ativos
    pub async fn get_kpis(
        pool: &PgPool,
        user_id: i32,
        user_role: &str,
    ) -> Result<DashboardKpiDto, AppError> {
        let (financas, lotes_ativos) = match user_role {
            "Administrador" => {
                let fin = sqlx::query_as::<_, FinancialAggregation>(
                    r#"SELECT
                        COALESCE(SUM(CASE WHEN "Tipo" = 'Entrada' THEN "Valor" ELSE 0 END), 0) as "total_entradas",
                        COALESCE(SUM(CASE WHEN "Tipo" = 'Saida' THEN "Valor" ELSE 0 END), 0) as "total_saidas"
                    FROM "TransacoesFinanceiras""#,
                )
                .fetch_one(pool)
                .await?;

                let ativos: i64 = sqlx::query_scalar(
                    r#"SELECT COUNT(*) FROM "Lotes" WHERE "DataSaida" IS NULL"#,
                )
                .fetch_one(pool)
                .await
                .map_err(|e| AppError::Internal(e.to_string()))?;

                (fin, ativos as i32)
            }
            "Produtor" => {
                let fin = sqlx::query_as::<_, FinancialAggregation>(
                    r#"SELECT
                        COALESCE(SUM(CASE WHEN tf."Tipo" = 'Entrada' THEN tf."Valor" ELSE 0 END), 0) as "total_entradas",
                        COALESCE(SUM(CASE WHEN tf."Tipo" = 'Saida' THEN tf."Valor" ELSE 0 END), 0) as "total_saidas"
                    FROM "TransacoesFinanceiras" tf
                    INNER JOIN "Lotes" l ON l."Id" = tf."LoteId"
                    INNER JOIN "Granjas" g ON g."Id" = l."GranjaId"
                    WHERE g."UsuarioId" = $1"#,
                )
                .bind(user_id)
                .fetch_one(pool)
                .await?;

                let ativos: i64 = sqlx::query_scalar(
                    r#"SELECT COUNT(*)
                    FROM "Lotes" l
                    INNER JOIN "Granjas" g ON g."Id" = l."GranjaId"
                    WHERE g."UsuarioId" = $1 AND l."DataSaida" IS NULL"#,
                )
                .bind(user_id)
                .fetch_one(pool)
                .await
                .map_err(|e| AppError::Internal(e.to_string()))?;

                (fin, ativos as i32)
            }
            "Financeiro" => {
                let fin = sqlx::query_as::<_, FinancialAggregation>(
                    r#"SELECT
                        COALESCE(SUM(CASE WHEN tf."Tipo" = 'Entrada' THEN tf."Valor" ELSE 0 END), 0) as "total_entradas",
                        COALESCE(SUM(CASE WHEN tf."Tipo" = 'Saida' THEN tf."Valor" ELSE 0 END), 0) as "total_saidas"
                    FROM "TransacoesFinanceiras" tf
                    INNER JOIN "Lotes" l ON l."Id" = tf."LoteId"
                    INNER JOIN "Granjas" g ON g."Id" = l."GranjaId"
                    WHERE g."UsuarioId" IN (
                        SELECT "ProdutorId" FROM "FinanceiroProdutor" WHERE "FinanceiroId" = $1
                    )"#,
                )
                .bind(user_id)
                .fetch_one(pool)
                .await?;

                let ativos: i64 = sqlx::query_scalar(
                    r#"SELECT COUNT(*)
                    FROM "Lotes" l
                    INNER JOIN "Granjas" g ON g."Id" = l."GranjaId"
                    WHERE g."UsuarioId" IN (
                        SELECT "ProdutorId" FROM "FinanceiroProdutor" WHERE "FinanceiroId" = $1
                    ) AND l."DataSaida" IS NULL"#,
                )
                .bind(user_id)
                .fetch_one(pool)
                .await
                .map_err(|e| AppError::Internal(e.to_string()))?;

                (fin, ativos as i32)
            }
            _ => {
                return Ok(DashboardKpiDto {
                    total_entradas: Decimal::ZERO,
                    total_saidas: Decimal::ZERO,
                    lucro_total: Decimal::ZERO,
                    lotes_ativos: 0,
                });
            }
        };

        let total_entradas = financas.total_entradas.unwrap_or(Decimal::ZERO);
        let total_saidas = financas.total_saidas.unwrap_or(Decimal::ZERO);
        let lucro_total = total_entradas - total_saidas;

        Ok(DashboardKpiDto {
            total_entradas,
            total_saidas,
            lucro_total,
            lotes_ativos,
        })
    }

    /// GET /api/dashboard/resumo-mensal -- resumo mensal financeiro
    pub async fn get_resumo_mensal(
        pool: &PgPool,
        user_id: i32,
        user_role: &str,
    ) -> Result<Vec<MonthlySummaryDto>, AppError> {
        let rows = match user_role {
            "Administrador" => {
                sqlx::query_as::<_, MonthlyRaw>(
                    r#"SELECT
                        EXTRACT(YEAR FROM "Data") as "ano",
                        EXTRACT(MONTH FROM "Data") as "mes",
                        COALESCE(SUM(CASE WHEN "Tipo" = 'Entrada' THEN "Valor" ELSE 0 END), 0) as "entradas",
                        COALESCE(SUM(CASE WHEN "Tipo" = 'Saida' THEN "Valor" ELSE 0 END), 0) as "saidas"
                    FROM "TransacoesFinanceiras"
                    GROUP BY EXTRACT(YEAR FROM "Data"), EXTRACT(MONTH FROM "Data")
                    ORDER BY "ano" DESC, "mes" DESC"#,
                )
                .fetch_all(pool)
                .await?
            }
            "Produtor" => {
                sqlx::query_as::<_, MonthlyRaw>(
                    r#"SELECT
                        EXTRACT(YEAR FROM tf."Data") as "ano",
                        EXTRACT(MONTH FROM tf."Data") as "mes",
                        COALESCE(SUM(CASE WHEN tf."Tipo" = 'Entrada' THEN tf."Valor" ELSE 0 END), 0) as "entradas",
                        COALESCE(SUM(CASE WHEN tf."Tipo" = 'Saida' THEN tf."Valor" ELSE 0 END), 0) as "saidas"
                    FROM "TransacoesFinanceiras" tf
                    INNER JOIN "Lotes" l ON l."Id" = tf."LoteId"
                    INNER JOIN "Granjas" g ON g."Id" = l."GranjaId"
                    WHERE g."UsuarioId" = $1
                    GROUP BY EXTRACT(YEAR FROM tf."Data"), EXTRACT(MONTH FROM tf."Data")
                    ORDER BY "ano" DESC, "mes" DESC"#,
                )
                .bind(user_id)
                .fetch_all(pool)
                .await?
            }
            "Financeiro" => {
                sqlx::query_as::<_, MonthlyRaw>(
                    r#"SELECT
                        EXTRACT(YEAR FROM tf."Data") as "ano",
                        EXTRACT(MONTH FROM tf."Data") as "mes",
                        COALESCE(SUM(CASE WHEN tf."Tipo" = 'Entrada' THEN tf."Valor" ELSE 0 END), 0) as "entradas",
                        COALESCE(SUM(CASE WHEN tf."Tipo" = 'Saida' THEN tf."Valor" ELSE 0 END), 0) as "saidas"
                    FROM "TransacoesFinanceiras" tf
                    INNER JOIN "Lotes" l ON l."Id" = tf."LoteId"
                    INNER JOIN "Granjas" g ON g."Id" = l."GranjaId"
                    WHERE g."UsuarioId" IN (
                        SELECT "ProdutorId" FROM "FinanceiroProdutor" WHERE "FinanceiroId" = $1
                    )
                    GROUP BY EXTRACT(YEAR FROM tf."Data"), EXTRACT(MONTH FROM tf."Data")
                    ORDER BY "ano" DESC, "mes" DESC"#,
                )
                .bind(user_id)
                .fetch_all(pool)
                .await?
            }
            _ => Vec::new(),
        };

        let resultado: Vec<MonthlySummaryDto> = rows
            .iter()
            .map(|r| {
                let ano = r.ano.unwrap_or(0.0) as i32;
                let mes_idx = r.mes.unwrap_or(1.0) as usize;
                let mes_nome = MESES_PT_BR.get(mes_idx.saturating_sub(1)).unwrap_or(&"???");
                let ano_short = ano % 100;

                MonthlySummaryDto {
                    mes: format!("{}/{:02}", mes_nome, ano_short),
                    entradas: r.entradas.unwrap_or(Decimal::ZERO),
                    saidas: r.saidas.unwrap_or(Decimal::ZERO),
                }
            })
            .collect();

        Ok(resultado)
    }
}
