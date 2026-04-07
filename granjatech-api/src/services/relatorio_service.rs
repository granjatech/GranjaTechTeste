use chrono::{DateTime, Duration, Utc};
use rust_decimal::Decimal;
use rust_decimal::prelude::*;
use sqlx::PgPool;

use crate::dto::financeiro::*;
use crate::dto::lote::LoteResponseDto;
use crate::dto::relatorios::*;
use crate::errors::AppError;

// === Helper row structs for sqlx ===

#[derive(Debug, sqlx::FromRow)]
struct TransacaoSimplRow {
    #[sqlx(rename = "Id")]
    id: i32,
    #[sqlx(rename = "Descricao")]
    descricao: String,
    #[sqlx(rename = "Valor")]
    valor: Decimal,
    #[sqlx(rename = "Tipo")]
    tipo: String,
    #[sqlx(rename = "Data")]
    data: DateTime<Utc>,
    #[sqlx(rename = "LoteIdentificador")]
    lote_identificador: Option<String>,
    #[sqlx(rename = "UsuarioNome")]
    usuario_nome: Option<String>,
    #[sqlx(rename = "GranjaNome")]
    granja_nome: Option<String>,
}

#[derive(Debug, sqlx::FromRow)]
struct LoteProducaoRow {
    #[sqlx(rename = "Id")]
    id: i32,
    #[sqlx(rename = "Codigo")]
    codigo: String,
    #[sqlx(rename = "Identificador")]
    identificador: String,
    #[sqlx(rename = "DataEntrada")]
    data_entrada: DateTime<Utc>,
    #[sqlx(rename = "DataAbatePrevista")]
    data_abate_prevista: Option<DateTime<Utc>>,
    #[sqlx(rename = "DataSaida")]
    data_saida: Option<DateTime<Utc>>,
    #[sqlx(rename = "QuantidadeAvesInicial")]
    quantidade_aves_inicial: i32,
    #[sqlx(rename = "QuantidadeAvesAtual")]
    quantidade_aves_atual: i32,
    #[sqlx(rename = "AreaGalpao")]
    area_galpao: Option<Decimal>,
    #[sqlx(rename = "Linhagem")]
    linhagem: Option<String>,
    #[sqlx(rename = "OrigemPintinhos")]
    origem_pintinhos: Option<String>,
    #[sqlx(rename = "Status")]
    status: String,
    #[sqlx(rename = "Observacoes")]
    observacoes: Option<String>,
    #[sqlx(rename = "GranjaId")]
    granja_id: i32,
    #[sqlx(rename = "DataCriacao")]
    data_criacao: DateTime<Utc>,
    #[sqlx(rename = "DataAtualizacao")]
    data_atualizacao: Option<DateTime<Utc>>,
    #[sqlx(rename = "GranjaNome")]
    granja_nome: Option<String>,
}

#[derive(Debug, sqlx::FromRow)]
struct AviculturaLoteRow {
    #[sqlx(rename = "Id")]
    id: i32,
    #[sqlx(rename = "Identificador")]
    identificador: String,
    #[sqlx(rename = "DataEntrada")]
    data_entrada: DateTime<Utc>,
    #[sqlx(rename = "QuantidadeAvesInicial")]
    quantidade_aves_inicial: i32,
    #[sqlx(rename = "QuantidadeAvesAtual")]
    quantidade_aves_atual: i32,
    #[sqlx(rename = "AreaGalpao")]
    area_galpao: Option<Decimal>,
    #[sqlx(rename = "Linhagem")]
    linhagem: Option<String>,
    #[sqlx(rename = "OrigemPintinhos")]
    origem_pintinhos: Option<String>,
    #[sqlx(rename = "Status")]
    status: String,
    #[sqlx(rename = "GranjaId")]
    granja_id: i32,
    #[sqlx(rename = "GranjaNome")]
    granja_nome: String,
}

#[derive(Debug, sqlx::FromRow)]
struct ConsumoRacaoRow {
    #[sqlx(rename = "total_kg")]
    total_kg: Option<Decimal>,
}

#[derive(Debug, sqlx::FromRow)]
struct ConsumoAguaRow {
    #[sqlx(rename = "total_litros")]
    total_litros: Option<Decimal>,
}

#[derive(Debug, sqlx::FromRow)]
struct PesagemRow {
    #[sqlx(rename = "PesoMedioGramas")]
    peso_medio_gramas: Decimal,
    #[sqlx(rename = "GanhoSemanal")]
    ganho_semanal: Option<Decimal>,
}

#[derive(Debug, sqlx::FromRow)]
struct EventoCountRow {
    #[sqlx(rename = "total")]
    total: Option<i64>,
    #[sqlx(rename = "vacinacoes")]
    vacinacoes: Option<i64>,
    #[sqlx(rename = "medicacoes")]
    medicacoes: Option<i64>,
    #[sqlx(rename = "custo_total")]
    custo_total: Option<Decimal>,
}

#[derive(Debug, sqlx::FromRow)]
struct PesagemDetalheRow {
    #[sqlx(rename = "SemanaVida")]
    semana_vida: i32,
    #[sqlx(rename = "IdadeDias")]
    idade_dias: i32,
    #[sqlx(rename = "PesoMedioGramas")]
    peso_medio_gramas: Decimal,
    #[sqlx(rename = "GanhoSemanal")]
    ganho_semanal: Option<Decimal>,
    #[sqlx(rename = "CoeficienteVariacao")]
    coeficiente_variacao: Option<Decimal>,
    #[sqlx(rename = "QuantidadeAmostrada")]
    quantidade_amostrada: i32,
}

#[derive(Debug, sqlx::FromRow)]
struct ConsumoRacaoDetalheRow {
    #[sqlx(rename = "Data")]
    data: DateTime<Utc>,
    #[sqlx(rename = "QuantidadeKg")]
    quantidade_kg: Decimal,
    #[sqlx(rename = "TipoRacao")]
    tipo_racao: String,
    #[sqlx(rename = "AvesVivas")]
    aves_vivas: i32,
}

#[derive(Debug, sqlx::FromRow)]
struct ConsumoAguaDetalheRow {
    #[sqlx(rename = "Data")]
    data: DateTime<Utc>,
    #[sqlx(rename = "QuantidadeLitros")]
    quantidade_litros: Decimal,
    #[sqlx(rename = "AvesVivas")]
    aves_vivas: i32,
    #[sqlx(rename = "TemperaturaAmbiente")]
    temperatura_ambiente: Option<Decimal>,
}

#[derive(Debug, sqlx::FromRow)]
struct EventoSanitarioDetalheRow {
    #[sqlx(rename = "Data")]
    data: DateTime<Utc>,
    #[sqlx(rename = "TipoEvento")]
    tipo_evento: String,
    #[sqlx(rename = "Produto")]
    produto: String,
    #[sqlx(rename = "ViaAdministracao")]
    via_administracao: Option<String>,
    #[sqlx(rename = "AvesTratadas")]
    aves_tratadas: Option<i32>,
    #[sqlx(rename = "Custo")]
    custo: Option<Decimal>,
    #[sqlx(rename = "PeriodoCarenciaDias")]
    periodo_carencia_dias: Option<i32>,
    #[sqlx(rename = "ResponsavelAplicacao")]
    responsavel_aplicacao: Option<String>,
}

#[derive(Debug, sqlx::FromRow)]
struct MortalidadeRow {
    #[sqlx(rename = "Data")]
    data: DateTime<Utc>,
    #[sqlx(rename = "IdadeDias")]
    idade_dias: i32,
    #[sqlx(rename = "QuantidadeMortas")]
    quantidade_mortas: i32,
    #[sqlx(rename = "CausaPrincipal")]
    causa_principal: Option<String>,
    #[sqlx(rename = "AvesVivas")]
    aves_vivas: i32,
}

#[derive(Debug, sqlx::FromRow)]
struct QualidadeArRow {
    #[sqlx(rename = "DataHora")]
    data_hora: DateTime<Utc>,
    #[sqlx(rename = "TemperaturaAr")]
    temperatura_ar: Option<Decimal>,
    #[sqlx(rename = "UmidadeRelativa")]
    umidade_relativa: Option<Decimal>,
    #[sqlx(rename = "NH3_ppm")]
    nh3_ppm: Option<Decimal>,
    #[sqlx(rename = "CO2_ppm")]
    co2_ppm: Option<Decimal>,
    #[sqlx(rename = "O2_percentual")]
    o2_percentual: Option<Decimal>,
    #[sqlx(rename = "VelocidadeAr_ms")]
    velocidade_ar_ms: Option<Decimal>,
    #[sqlx(rename = "Luminosidade_lux")]
    luminosidade_lux: Option<Decimal>,
}

pub struct RelatorioService;

impl RelatorioService {
    // ======================================================================
    // financeiro_simplificado (RELA-02)
    // ======================================================================
    pub async fn financeiro_simplificado(
        pool: &PgPool,
        user_id: i32,
        user_role: &str,
        data_inicio: DateTime<Utc>,
        data_fim: DateTime<Utc>,
        granja_id: Option<i32>,
    ) -> Result<RelatorioFinanceiroSimplificadoDto, AppError> {
        // Build dynamic query based on role
        let rows: Vec<TransacaoSimplRow> = match user_role {
            "Administrador" => {
                if let Some(gid) = granja_id {
                    sqlx::query_as::<_, TransacaoSimplRow>(
                        r#"SELECT
                            tf."Id", tf."Descricao", tf."Valor", tf."Tipo", tf."Data",
                            l."Identificador" AS "LoteIdentificador",
                            u."Nome" AS "UsuarioNome",
                            g."Nome" AS "GranjaNome"
                        FROM "TransacoesFinanceiras" tf
                        LEFT JOIN "Lotes" l ON l."Id" = tf."LoteId"
                        LEFT JOIN "Granjas" g ON g."Id" = l."GranjaId"
                        LEFT JOIN "Usuarios" u ON u."Id" = tf."UsuarioId"
                        WHERE tf."Data" >= $1 AND tf."Data" <= $2
                          AND (tf."LoteId" IS NULL OR l."GranjaId" = $3)
                        ORDER BY tf."Data" DESC
                        LIMIT 1000"#,
                    )
                    .bind(data_inicio)
                    .bind(data_fim)
                    .bind(gid)
                    .fetch_all(pool)
                    .await?
                } else {
                    sqlx::query_as::<_, TransacaoSimplRow>(
                        r#"SELECT
                            tf."Id", tf."Descricao", tf."Valor", tf."Tipo", tf."Data",
                            l."Identificador" AS "LoteIdentificador",
                            u."Nome" AS "UsuarioNome",
                            g."Nome" AS "GranjaNome"
                        FROM "TransacoesFinanceiras" tf
                        LEFT JOIN "Lotes" l ON l."Id" = tf."LoteId"
                        LEFT JOIN "Granjas" g ON g."Id" = l."GranjaId"
                        LEFT JOIN "Usuarios" u ON u."Id" = tf."UsuarioId"
                        WHERE tf."Data" >= $1 AND tf."Data" <= $2
                        ORDER BY tf."Data" DESC
                        LIMIT 1000"#,
                    )
                    .bind(data_inicio)
                    .bind(data_fim)
                    .fetch_all(pool)
                    .await?
                }
            }
            "Produtor" => {
                if let Some(gid) = granja_id {
                    sqlx::query_as::<_, TransacaoSimplRow>(
                        r#"SELECT
                            tf."Id", tf."Descricao", tf."Valor", tf."Tipo", tf."Data",
                            l."Identificador" AS "LoteIdentificador",
                            u."Nome" AS "UsuarioNome",
                            g."Nome" AS "GranjaNome"
                        FROM "TransacoesFinanceiras" tf
                        LEFT JOIN "Lotes" l ON l."Id" = tf."LoteId"
                        LEFT JOIN "Granjas" g ON g."Id" = l."GranjaId"
                        LEFT JOIN "Usuarios" u ON u."Id" = tf."UsuarioId"
                        WHERE tf."Data" >= $1 AND tf."Data" <= $2
                          AND (tf."LoteId" IS NULL OR g."UsuarioId" = $3)
                          AND (tf."LoteId" IS NULL OR l."GranjaId" = $4)
                        ORDER BY tf."Data" DESC
                        LIMIT 1000"#,
                    )
                    .bind(data_inicio)
                    .bind(data_fim)
                    .bind(user_id)
                    .bind(gid)
                    .fetch_all(pool)
                    .await?
                } else {
                    sqlx::query_as::<_, TransacaoSimplRow>(
                        r#"SELECT
                            tf."Id", tf."Descricao", tf."Valor", tf."Tipo", tf."Data",
                            l."Identificador" AS "LoteIdentificador",
                            u."Nome" AS "UsuarioNome",
                            g."Nome" AS "GranjaNome"
                        FROM "TransacoesFinanceiras" tf
                        LEFT JOIN "Lotes" l ON l."Id" = tf."LoteId"
                        LEFT JOIN "Granjas" g ON g."Id" = l."GranjaId"
                        LEFT JOIN "Usuarios" u ON u."Id" = tf."UsuarioId"
                        WHERE tf."Data" >= $1 AND tf."Data" <= $2
                          AND (tf."LoteId" IS NULL OR g."UsuarioId" = $3)
                        ORDER BY tf."Data" DESC
                        LIMIT 1000"#,
                    )
                    .bind(data_inicio)
                    .bind(data_fim)
                    .bind(user_id)
                    .fetch_all(pool)
                    .await?
                }
            }
            "Financeiro" => {
                if let Some(gid) = granja_id {
                    sqlx::query_as::<_, TransacaoSimplRow>(
                        r#"SELECT
                            tf."Id", tf."Descricao", tf."Valor", tf."Tipo", tf."Data",
                            l."Identificador" AS "LoteIdentificador",
                            u."Nome" AS "UsuarioNome",
                            g."Nome" AS "GranjaNome"
                        FROM "TransacoesFinanceiras" tf
                        LEFT JOIN "Lotes" l ON l."Id" = tf."LoteId"
                        LEFT JOIN "Granjas" g ON g."Id" = l."GranjaId"
                        LEFT JOIN "Usuarios" u ON u."Id" = tf."UsuarioId"
                        WHERE tf."Data" >= $1 AND tf."Data" <= $2
                          AND (tf."LoteId" IS NULL OR g."UsuarioId" IN (
                              SELECT "ProdutorId" FROM "FinanceiroProdutor" WHERE "FinanceiroId" = $3
                          ))
                          AND (tf."LoteId" IS NULL OR l."GranjaId" = $4)
                        ORDER BY tf."Data" DESC
                        LIMIT 1000"#,
                    )
                    .bind(data_inicio)
                    .bind(data_fim)
                    .bind(user_id)
                    .bind(gid)
                    .fetch_all(pool)
                    .await?
                } else {
                    sqlx::query_as::<_, TransacaoSimplRow>(
                        r#"SELECT
                            tf."Id", tf."Descricao", tf."Valor", tf."Tipo", tf."Data",
                            l."Identificador" AS "LoteIdentificador",
                            u."Nome" AS "UsuarioNome",
                            g."Nome" AS "GranjaNome"
                        FROM "TransacoesFinanceiras" tf
                        LEFT JOIN "Lotes" l ON l."Id" = tf."LoteId"
                        LEFT JOIN "Granjas" g ON g."Id" = l."GranjaId"
                        LEFT JOIN "Usuarios" u ON u."Id" = tf."UsuarioId"
                        WHERE tf."Data" >= $1 AND tf."Data" <= $2
                          AND (tf."LoteId" IS NULL OR g."UsuarioId" IN (
                              SELECT "ProdutorId" FROM "FinanceiroProdutor" WHERE "FinanceiroId" = $3
                          ))
                        ORDER BY tf."Data" DESC
                        LIMIT 1000"#,
                    )
                    .bind(data_inicio)
                    .bind(data_fim)
                    .bind(user_id)
                    .fetch_all(pool)
                    .await?
                }
            }
            _ => Vec::new(),
        };

        let transacoes: Vec<TransacaoSimplificadaDto> = rows
            .into_iter()
            .map(|r| TransacaoSimplificadaDto {
                id: r.id,
                descricao: r.descricao,
                valor: r.valor,
                tipo: r.tipo,
                data: r.data,
                lote_identificador: r.lote_identificador,
                usuario_nome: r.usuario_nome,
                granja_nome: r.granja_nome,
            })
            .collect();

        let total_entradas = transacoes
            .iter()
            .filter(|t| t.tipo.eq_ignore_ascii_case("Entrada"))
            .map(|t| t.valor)
            .sum::<Decimal>();

        let total_saidas = transacoes
            .iter()
            .filter(|t| {
                t.tipo.eq_ignore_ascii_case("Saida") || t.tipo.eq_ignore_ascii_case("Saida")
            })
            .map(|t| t.valor)
            .sum::<Decimal>();

        Ok(RelatorioFinanceiroSimplificadoDto {
            total_entradas,
            total_saidas,
            saldo: total_entradas - total_saidas,
            transacoes,
        })
    }

    // ======================================================================
    // financeiro (RELA-03)
    // ======================================================================
    pub async fn financeiro(
        pool: &PgPool,
        user_id: i32,
        user_role: &str,
        data_inicio: DateTime<Utc>,
        data_fim: DateTime<Utc>,
        granja_id: Option<i32>,
    ) -> Result<RelatorioFinanceiroDto, AppError> {
        // Query transacoes de lotes (with role-based filtering)
        let lote_rows: Vec<TransacaoSimplRow> = match user_role {
            "Administrador" => {
                if let Some(gid) = granja_id {
                    sqlx::query_as::<_, TransacaoSimplRow>(
                        r#"SELECT
                            tf."Id", tf."Descricao", tf."Valor", tf."Tipo", tf."Data",
                            l."Identificador" AS "LoteIdentificador",
                            u."Nome" AS "UsuarioNome",
                            g."Nome" AS "GranjaNome"
                        FROM "TransacoesFinanceiras" tf
                        INNER JOIN "Lotes" l ON l."Id" = tf."LoteId"
                        INNER JOIN "Granjas" g ON g."Id" = l."GranjaId"
                        LEFT JOIN "Usuarios" u ON u."Id" = tf."UsuarioId"
                        WHERE tf."LoteId" IS NOT NULL
                          AND tf."Data" >= $1 AND tf."Data" <= $2
                          AND l."GranjaId" = $3
                        ORDER BY tf."Data" DESC
                        LIMIT 1000"#,
                    )
                    .bind(data_inicio)
                    .bind(data_fim)
                    .bind(gid)
                    .fetch_all(pool)
                    .await?
                } else {
                    sqlx::query_as::<_, TransacaoSimplRow>(
                        r#"SELECT
                            tf."Id", tf."Descricao", tf."Valor", tf."Tipo", tf."Data",
                            l."Identificador" AS "LoteIdentificador",
                            u."Nome" AS "UsuarioNome",
                            g."Nome" AS "GranjaNome"
                        FROM "TransacoesFinanceiras" tf
                        INNER JOIN "Lotes" l ON l."Id" = tf."LoteId"
                        INNER JOIN "Granjas" g ON g."Id" = l."GranjaId"
                        LEFT JOIN "Usuarios" u ON u."Id" = tf."UsuarioId"
                        WHERE tf."LoteId" IS NOT NULL
                          AND tf."Data" >= $1 AND tf."Data" <= $2
                        ORDER BY tf."Data" DESC
                        LIMIT 1000"#,
                    )
                    .bind(data_inicio)
                    .bind(data_fim)
                    .fetch_all(pool)
                    .await?
                }
            }
            "Produtor" => {
                if let Some(gid) = granja_id {
                    sqlx::query_as::<_, TransacaoSimplRow>(
                        r#"SELECT
                            tf."Id", tf."Descricao", tf."Valor", tf."Tipo", tf."Data",
                            l."Identificador" AS "LoteIdentificador",
                            u."Nome" AS "UsuarioNome",
                            g."Nome" AS "GranjaNome"
                        FROM "TransacoesFinanceiras" tf
                        INNER JOIN "Lotes" l ON l."Id" = tf."LoteId"
                        INNER JOIN "Granjas" g ON g."Id" = l."GranjaId"
                        LEFT JOIN "Usuarios" u ON u."Id" = tf."UsuarioId"
                        WHERE tf."LoteId" IS NOT NULL
                          AND tf."Data" >= $1 AND tf."Data" <= $2
                          AND g."UsuarioId" = $3
                          AND l."GranjaId" = $4
                        ORDER BY tf."Data" DESC
                        LIMIT 1000"#,
                    )
                    .bind(data_inicio)
                    .bind(data_fim)
                    .bind(user_id)
                    .bind(gid)
                    .fetch_all(pool)
                    .await?
                } else {
                    sqlx::query_as::<_, TransacaoSimplRow>(
                        r#"SELECT
                            tf."Id", tf."Descricao", tf."Valor", tf."Tipo", tf."Data",
                            l."Identificador" AS "LoteIdentificador",
                            u."Nome" AS "UsuarioNome",
                            g."Nome" AS "GranjaNome"
                        FROM "TransacoesFinanceiras" tf
                        INNER JOIN "Lotes" l ON l."Id" = tf."LoteId"
                        INNER JOIN "Granjas" g ON g."Id" = l."GranjaId"
                        LEFT JOIN "Usuarios" u ON u."Id" = tf."UsuarioId"
                        WHERE tf."LoteId" IS NOT NULL
                          AND tf."Data" >= $1 AND tf."Data" <= $2
                          AND g."UsuarioId" = $3
                        ORDER BY tf."Data" DESC
                        LIMIT 1000"#,
                    )
                    .bind(data_inicio)
                    .bind(data_fim)
                    .bind(user_id)
                    .fetch_all(pool)
                    .await?
                }
            }
            "Financeiro" => {
                if let Some(gid) = granja_id {
                    sqlx::query_as::<_, TransacaoSimplRow>(
                        r#"SELECT
                            tf."Id", tf."Descricao", tf."Valor", tf."Tipo", tf."Data",
                            l."Identificador" AS "LoteIdentificador",
                            u."Nome" AS "UsuarioNome",
                            g."Nome" AS "GranjaNome"
                        FROM "TransacoesFinanceiras" tf
                        INNER JOIN "Lotes" l ON l."Id" = tf."LoteId"
                        INNER JOIN "Granjas" g ON g."Id" = l."GranjaId"
                        LEFT JOIN "Usuarios" u ON u."Id" = tf."UsuarioId"
                        WHERE tf."LoteId" IS NOT NULL
                          AND tf."Data" >= $1 AND tf."Data" <= $2
                          AND g."UsuarioId" IN (
                              SELECT "ProdutorId" FROM "FinanceiroProdutor" WHERE "FinanceiroId" = $3
                          )
                          AND l."GranjaId" = $4
                        ORDER BY tf."Data" DESC
                        LIMIT 1000"#,
                    )
                    .bind(data_inicio)
                    .bind(data_fim)
                    .bind(user_id)
                    .bind(gid)
                    .fetch_all(pool)
                    .await?
                } else {
                    sqlx::query_as::<_, TransacaoSimplRow>(
                        r#"SELECT
                            tf."Id", tf."Descricao", tf."Valor", tf."Tipo", tf."Data",
                            l."Identificador" AS "LoteIdentificador",
                            u."Nome" AS "UsuarioNome",
                            g."Nome" AS "GranjaNome"
                        FROM "TransacoesFinanceiras" tf
                        INNER JOIN "Lotes" l ON l."Id" = tf."LoteId"
                        INNER JOIN "Granjas" g ON g."Id" = l."GranjaId"
                        LEFT JOIN "Usuarios" u ON u."Id" = tf."UsuarioId"
                        WHERE tf."LoteId" IS NOT NULL
                          AND tf."Data" >= $1 AND tf."Data" <= $2
                          AND g."UsuarioId" IN (
                              SELECT "ProdutorId" FROM "FinanceiroProdutor" WHERE "FinanceiroId" = $3
                          )
                        ORDER BY tf."Data" DESC
                        LIMIT 1000"#,
                    )
                    .bind(data_inicio)
                    .bind(data_fim)
                    .bind(user_id)
                    .fetch_all(pool)
                    .await?
                }
            }
            _ => Vec::new(),
        };

        // Query transacoes gerais (sem lote)
        let gerais_rows: Vec<TransacaoSimplRow> = if user_role == "Administrador" {
            sqlx::query_as::<_, TransacaoSimplRow>(
                r#"SELECT
                    tf."Id", tf."Descricao", tf."Valor", tf."Tipo", tf."Data",
                    NULL::text AS "LoteIdentificador",
                    u."Nome" AS "UsuarioNome",
                    NULL::text AS "GranjaNome"
                FROM "TransacoesFinanceiras" tf
                LEFT JOIN "Usuarios" u ON u."Id" = tf."UsuarioId"
                WHERE tf."LoteId" IS NULL
                  AND tf."Data" >= $1 AND tf."Data" <= $2
                ORDER BY tf."Data" DESC
                LIMIT 1000"#,
            )
            .bind(data_inicio)
            .bind(data_fim)
            .fetch_all(pool)
            .await?
        } else {
            sqlx::query_as::<_, TransacaoSimplRow>(
                r#"SELECT
                    tf."Id", tf."Descricao", tf."Valor", tf."Tipo", tf."Data",
                    NULL::text AS "LoteIdentificador",
                    u."Nome" AS "UsuarioNome",
                    NULL::text AS "GranjaNome"
                FROM "TransacoesFinanceiras" tf
                LEFT JOIN "Usuarios" u ON u."Id" = tf."UsuarioId"
                WHERE tf."LoteId" IS NULL
                  AND tf."Data" >= $1 AND tf."Data" <= $2
                  AND tf."UsuarioId" = $3
                ORDER BY tf."Data" DESC
                LIMIT 1000"#,
            )
            .bind(data_inicio)
            .bind(data_fim)
            .bind(user_id)
            .fetch_all(pool)
            .await?
        };

        // Merge and sort by date DESC
        let mut all_rows: Vec<TransacaoSimplRow> = gerais_rows
            .into_iter()
            .chain(lote_rows)
            .collect();
        all_rows.sort_by(|a, b| b.data.cmp(&a.data));

        let transacoes: Vec<TransacaoSimplificadaDto> = all_rows
            .into_iter()
            .map(|r| TransacaoSimplificadaDto {
                id: r.id,
                descricao: r.descricao,
                valor: r.valor,
                tipo: r.tipo,
                data: r.data,
                lote_identificador: r.lote_identificador,
                usuario_nome: r.usuario_nome,
                granja_nome: r.granja_nome,
            })
            .collect();

        let total_entradas = transacoes
            .iter()
            .filter(|t| t.tipo.eq_ignore_ascii_case("Entrada"))
            .map(|t| t.valor)
            .sum::<Decimal>();

        let total_saidas = transacoes
            .iter()
            .filter(|t| {
                t.tipo.eq_ignore_ascii_case("Saida") || t.tipo.eq_ignore_ascii_case("Saida")
            })
            .map(|t| t.valor)
            .sum::<Decimal>();

        Ok(RelatorioFinanceiroDto {
            total_entradas,
            total_saidas,
            saldo: total_entradas - total_saidas,
            transacoes,
        })
    }

    // ======================================================================
    // producao (RELA-04)
    // ======================================================================
    pub async fn producao(
        pool: &PgPool,
        user_id: i32,
        user_role: &str,
        data_inicio: DateTime<Utc>,
        data_fim: DateTime<Utc>,
        granja_id: Option<i32>,
    ) -> Result<RelatorioProducaoDto, AppError> {
        let rows: Vec<LoteProducaoRow> = match user_role {
            "Administrador" => {
                if let Some(gid) = granja_id {
                    sqlx::query_as::<_, LoteProducaoRow>(
                        r#"SELECT
                            l."Id", l."Codigo", l."Identificador", l."DataEntrada",
                            l."DataAbatePrevista", l."DataSaida",
                            l."QuantidadeAvesInicial", l."QuantidadeAvesAtual",
                            l."AreaGalpao", l."Linhagem", l."OrigemPintinhos",
                            l."Status", l."Observacoes", l."GranjaId",
                            l."DataCriacao", l."DataAtualizacao",
                            g."Nome" AS "GranjaNome"
                        FROM "Lotes" l
                        INNER JOIN "Granjas" g ON g."Id" = l."GranjaId"
                        WHERE l."DataEntrada" >= $1 AND l."DataEntrada" <= $2
                          AND l."GranjaId" = $3
                        ORDER BY l."DataEntrada" DESC
                        LIMIT 1000"#,
                    )
                    .bind(data_inicio)
                    .bind(data_fim)
                    .bind(gid)
                    .fetch_all(pool)
                    .await?
                } else {
                    sqlx::query_as::<_, LoteProducaoRow>(
                        r#"SELECT
                            l."Id", l."Codigo", l."Identificador", l."DataEntrada",
                            l."DataAbatePrevista", l."DataSaida",
                            l."QuantidadeAvesInicial", l."QuantidadeAvesAtual",
                            l."AreaGalpao", l."Linhagem", l."OrigemPintinhos",
                            l."Status", l."Observacoes", l."GranjaId",
                            l."DataCriacao", l."DataAtualizacao",
                            g."Nome" AS "GranjaNome"
                        FROM "Lotes" l
                        INNER JOIN "Granjas" g ON g."Id" = l."GranjaId"
                        WHERE l."DataEntrada" >= $1 AND l."DataEntrada" <= $2
                        ORDER BY l."DataEntrada" DESC
                        LIMIT 1000"#,
                    )
                    .bind(data_inicio)
                    .bind(data_fim)
                    .fetch_all(pool)
                    .await?
                }
            }
            "Produtor" => {
                if let Some(gid) = granja_id {
                    sqlx::query_as::<_, LoteProducaoRow>(
                        r#"SELECT
                            l."Id", l."Codigo", l."Identificador", l."DataEntrada",
                            l."DataAbatePrevista", l."DataSaida",
                            l."QuantidadeAvesInicial", l."QuantidadeAvesAtual",
                            l."AreaGalpao", l."Linhagem", l."OrigemPintinhos",
                            l."Status", l."Observacoes", l."GranjaId",
                            l."DataCriacao", l."DataAtualizacao",
                            g."Nome" AS "GranjaNome"
                        FROM "Lotes" l
                        INNER JOIN "Granjas" g ON g."Id" = l."GranjaId"
                        WHERE l."DataEntrada" >= $1 AND l."DataEntrada" <= $2
                          AND g."UsuarioId" = $3
                          AND l."GranjaId" = $4
                        ORDER BY l."DataEntrada" DESC
                        LIMIT 1000"#,
                    )
                    .bind(data_inicio)
                    .bind(data_fim)
                    .bind(user_id)
                    .bind(gid)
                    .fetch_all(pool)
                    .await?
                } else {
                    sqlx::query_as::<_, LoteProducaoRow>(
                        r#"SELECT
                            l."Id", l."Codigo", l."Identificador", l."DataEntrada",
                            l."DataAbatePrevista", l."DataSaida",
                            l."QuantidadeAvesInicial", l."QuantidadeAvesAtual",
                            l."AreaGalpao", l."Linhagem", l."OrigemPintinhos",
                            l."Status", l."Observacoes", l."GranjaId",
                            l."DataCriacao", l."DataAtualizacao",
                            g."Nome" AS "GranjaNome"
                        FROM "Lotes" l
                        INNER JOIN "Granjas" g ON g."Id" = l."GranjaId"
                        WHERE l."DataEntrada" >= $1 AND l."DataEntrada" <= $2
                          AND g."UsuarioId" = $3
                        ORDER BY l."DataEntrada" DESC
                        LIMIT 1000"#,
                    )
                    .bind(data_inicio)
                    .bind(data_fim)
                    .bind(user_id)
                    .fetch_all(pool)
                    .await?
                }
            }
            "Financeiro" => {
                if let Some(gid) = granja_id {
                    sqlx::query_as::<_, LoteProducaoRow>(
                        r#"SELECT
                            l."Id", l."Codigo", l."Identificador", l."DataEntrada",
                            l."DataAbatePrevista", l."DataSaida",
                            l."QuantidadeAvesInicial", l."QuantidadeAvesAtual",
                            l."AreaGalpao", l."Linhagem", l."OrigemPintinhos",
                            l."Status", l."Observacoes", l."GranjaId",
                            l."DataCriacao", l."DataAtualizacao",
                            g."Nome" AS "GranjaNome"
                        FROM "Lotes" l
                        INNER JOIN "Granjas" g ON g."Id" = l."GranjaId"
                        WHERE l."DataEntrada" >= $1 AND l."DataEntrada" <= $2
                          AND g."UsuarioId" IN (
                              SELECT "ProdutorId" FROM "FinanceiroProdutor" WHERE "FinanceiroId" = $3
                          )
                          AND l."GranjaId" = $4
                        ORDER BY l."DataEntrada" DESC
                        LIMIT 1000"#,
                    )
                    .bind(data_inicio)
                    .bind(data_fim)
                    .bind(user_id)
                    .bind(gid)
                    .fetch_all(pool)
                    .await?
                } else {
                    sqlx::query_as::<_, LoteProducaoRow>(
                        r#"SELECT
                            l."Id", l."Codigo", l."Identificador", l."DataEntrada",
                            l."DataAbatePrevista", l."DataSaida",
                            l."QuantidadeAvesInicial", l."QuantidadeAvesAtual",
                            l."AreaGalpao", l."Linhagem", l."OrigemPintinhos",
                            l."Status", l."Observacoes", l."GranjaId",
                            l."DataCriacao", l."DataAtualizacao",
                            g."Nome" AS "GranjaNome"
                        FROM "Lotes" l
                        INNER JOIN "Granjas" g ON g."Id" = l."GranjaId"
                        WHERE l."DataEntrada" >= $1 AND l."DataEntrada" <= $2
                          AND g."UsuarioId" IN (
                              SELECT "ProdutorId" FROM "FinanceiroProdutor" WHERE "FinanceiroId" = $3
                          )
                        ORDER BY l."DataEntrada" DESC
                        LIMIT 1000"#,
                    )
                    .bind(data_inicio)
                    .bind(data_fim)
                    .bind(user_id)
                    .fetch_all(pool)
                    .await?
                }
            }
            _ => Vec::new(),
        };

        let total_aves_inicial: i32 = rows.iter().map(|r| r.quantidade_aves_inicial).sum();
        let total_lotes = rows.len() as i32;

        let lotes: Vec<LoteResponseDto> = rows
            .into_iter()
            .map(|r| {
                let idade = (Utc::now().date_naive() - r.data_entrada.date_naive()).num_days() as i32;
                let viabilidade = if r.quantidade_aves_inicial > 0 {
                    let mortalidade = Decimal::from(r.quantidade_aves_inicial - r.quantidade_aves_atual)
                        / Decimal::from(r.quantidade_aves_inicial)
                        * Decimal::from(100);
                    Decimal::from(100) - mortalidade
                } else {
                    Decimal::ZERO
                };
                let densidade = match r.area_galpao {
                    Some(area) if area > Decimal::ZERO => Decimal::from(r.quantidade_aves_atual) / area,
                    _ => Decimal::ZERO,
                };
                LoteResponseDto {
                    id: r.id,
                    codigo: r.codigo,
                    identificador: r.identificador,
                    data_entrada: r.data_entrada,
                    data_abate_prevista: r.data_abate_prevista,
                    data_saida: r.data_saida,
                    quantidade_aves_inicial: r.quantidade_aves_inicial,
                    quantidade_aves_atual: r.quantidade_aves_atual,
                    area_galpao: r.area_galpao,
                    linhagem: r.linhagem,
                    origem_pintinhos: r.origem_pintinhos,
                    status: r.status,
                    observacoes: r.observacoes,
                    granja_id: r.granja_id,
                    data_criacao: r.data_criacao,
                    data_atualizacao: r.data_atualizacao,
                    idade_atual_dias: idade,
                    viabilidade,
                    densidade_atual: densidade,
                }
            })
            .collect();

        Ok(RelatorioProducaoDto {
            total_lotes,
            total_aves_inicial,
            lotes,
        })
    }

    // ======================================================================
    // avicultura (RELA-05)
    // ======================================================================
    pub async fn avicultura(
        pool: &PgPool,
        data_inicio: Option<DateTime<Utc>>,
        data_fim: Option<DateTime<Utc>>,
        lote_id: Option<i32>,
    ) -> Result<RelatorioAviculturaDto, AppError> {
        let inicio = data_inicio.unwrap_or_else(|| Utc::now() - Duration::days(30));
        let fim = data_fim.unwrap_or_else(|| Utc::now());

        // Query lotes
        let lotes: Vec<AviculturaLoteRow> = if let Some(lid) = lote_id {
            sqlx::query_as::<_, AviculturaLoteRow>(
                r#"SELECT
                    l."Id", l."Identificador", l."DataEntrada",
                    l."QuantidadeAvesInicial", l."QuantidadeAvesAtual",
                    l."AreaGalpao", l."Linhagem", l."OrigemPintinhos",
                    l."Status", l."GranjaId",
                    g."Nome" AS "GranjaNome"
                FROM "Lotes" l
                INNER JOIN "Granjas" g ON g."Id" = l."GranjaId"
                WHERE l."DataEntrada" >= $1 AND l."DataEntrada" <= $2
                  AND l."Id" = $3
                ORDER BY l."DataEntrada" DESC
                LIMIT 1000"#,
            )
            .bind(inicio)
            .bind(fim)
            .bind(lid)
            .fetch_all(pool)
            .await?
        } else {
            sqlx::query_as::<_, AviculturaLoteRow>(
                r#"SELECT
                    l."Id", l."Identificador", l."DataEntrada",
                    l."QuantidadeAvesInicial", l."QuantidadeAvesAtual",
                    l."AreaGalpao", l."Linhagem", l."OrigemPintinhos",
                    l."Status", l."GranjaId",
                    g."Nome" AS "GranjaNome"
                FROM "Lotes" l
                INNER JOIN "Granjas" g ON g."Id" = l."GranjaId"
                WHERE l."DataEntrada" >= $1 AND l."DataEntrada" <= $2
                ORDER BY l."DataEntrada" DESC
                LIMIT 1000"#,
            )
            .bind(inicio)
            .bind(fim)
            .fetch_all(pool)
            .await?
        };

        let mut total_aves_alojadas: i32 = 0;
        let mut total_aves_atuais: i32 = 0;
        let mut consumo_total_racao = Decimal::ZERO;
        let mut consumo_total_agua = Decimal::ZERO;
        let mut total_eventos_sanitarios: i32 = 0;
        let mut custo_total_sanitario = Decimal::ZERO;
        let mut mortalidades: Vec<Decimal> = Vec::new();
        let mut viabilidades: Vec<Decimal> = Vec::new();

        let mut detalhes: Vec<DetalheLoteAviculturaDto> = Vec::new();
        let mut cas: Vec<Decimal> = Vec::new();
        let mut ieps: Vec<Decimal> = Vec::new();

        for lote in &lotes {
            let idade_dias = (Utc::now().date_naive() - lote.data_entrada.date_naive()).num_days();

            let mortalidade_pct = if lote.quantidade_aves_inicial > 0 {
                Decimal::from(lote.quantidade_aves_inicial - lote.quantidade_aves_atual)
                    / Decimal::from(lote.quantidade_aves_inicial)
                    * Decimal::from(100)
            } else {
                Decimal::ZERO
            };
            let viabilidade = Decimal::from(100) - mortalidade_pct;
            let densidade = match lote.area_galpao {
                Some(area) if area > Decimal::ZERO => Decimal::from(lote.quantidade_aves_atual) / area,
                _ => Decimal::ZERO,
            };

            // Consumo racao
            let racao_row = sqlx::query_as::<_, ConsumoRacaoRow>(
                r#"SELECT COALESCE(SUM("QuantidadeKg"), 0) AS "total_kg"
                FROM "ConsumosRacao" WHERE "LoteId" = $1"#,
            )
            .bind(lote.id)
            .fetch_one(pool)
            .await?;
            let racao_total = racao_row.total_kg.unwrap_or(Decimal::ZERO);

            // Consumo agua
            let agua_row = sqlx::query_as::<_, ConsumoAguaRow>(
                r#"SELECT COALESCE(SUM("QuantidadeLitros"), 0) AS "total_litros"
                FROM "ConsumosAgua" WHERE "LoteId" = $1"#,
            )
            .bind(lote.id)
            .fetch_one(pool)
            .await?;
            let agua_total = agua_row.total_litros.unwrap_or(Decimal::ZERO);

            // Pesagens for GMD
            let pesagens = sqlx::query_as::<_, PesagemRow>(
                r#"SELECT "PesoMedioGramas", "GanhoSemanal"
                FROM "PesagensSemanais" WHERE "LoteId" = $1
                ORDER BY "DataPesagem" DESC"#,
            )
            .bind(lote.id)
            .fetch_all(pool)
            .await?;

            let gmd = if !pesagens.is_empty() {
                let total_ganho: Decimal = pesagens
                    .iter()
                    .map(|p| p.ganho_semanal.unwrap_or(Decimal::ZERO) / Decimal::from(7))
                    .sum();
                total_ganho / Decimal::from(pesagens.len() as i64)
            } else {
                Decimal::ZERO
            };

            // Eventos sanitarios
            let eventos = sqlx::query_as::<_, EventoCountRow>(
                r#"SELECT
                    COUNT(*) AS "total",
                    COUNT(*) FILTER (WHERE "TipoEvento" = 'Vacinacao') AS "vacinacoes",
                    COUNT(*) FILTER (WHERE "TipoEvento" = 'Medicacao') AS "medicacoes",
                    COALESCE(SUM("Custo"), 0) AS "custo_total"
                FROM "EventosSanitarios" WHERE "LoteId" = $1"#,
            )
            .bind(lote.id)
            .fetch_one(pool)
            .await?;

            // Consumo por ave
            let racao_media_por_ave = if lote.quantidade_aves_atual > 0 {
                (racao_total * Decimal::from(1000)) / Decimal::from(lote.quantidade_aves_atual)
            } else {
                Decimal::ZERO
            };
            let agua_media_por_ave = if lote.quantidade_aves_atual > 0 {
                (agua_total * Decimal::from(1000)) / Decimal::from(lote.quantidade_aves_atual)
            } else {
                Decimal::ZERO
            };

            let relacao_agua_racao = if racao_total > Decimal::ZERO {
                agua_total / racao_total
            } else {
                Decimal::ZERO
            };

            // Conversao Alimentar: CA = totalRacao / ganhoTotalLoteKg
            let ca = if !pesagens.is_empty() && racao_total > Decimal::ZERO {
                let peso_recente = pesagens[0].peso_medio_gramas;
                let ganho_por_ave = peso_recente - Decimal::from(45); // 45g peso inicial
                let ganho_total_kg = (ganho_por_ave * Decimal::from(lote.quantidade_aves_atual)) / Decimal::from(1000);
                if ganho_total_kg > Decimal::ZERO {
                    racao_total / ganho_total_kg
                } else {
                    Decimal::ZERO
                }
            } else {
                Decimal::ZERO
            };

            // IEP = (GanhoPeso * Viabilidade * 100) / (CA * Idade)
            let iep = if !pesagens.is_empty() && idade_dias > 0 && ca > Decimal::ZERO {
                let peso_recente = pesagens[0].peso_medio_gramas;
                let ganho_peso_kg = (peso_recente - Decimal::from(45)) / Decimal::from(1000);
                (ganho_peso_kg * viabilidade * Decimal::from(100))
                    / (ca * Decimal::from(idade_dias))
            } else {
                Decimal::ZERO
            };

            total_aves_alojadas += lote.quantidade_aves_inicial;
            total_aves_atuais += lote.quantidade_aves_atual;
            consumo_total_racao += racao_total;
            consumo_total_agua += agua_total;
            total_eventos_sanitarios += eventos.total.unwrap_or(0) as i32;
            custo_total_sanitario += eventos.custo_total.unwrap_or(Decimal::ZERO);
            mortalidades.push(mortalidade_pct);
            viabilidades.push(viabilidade);
            cas.push(ca);
            ieps.push(iep);

            detalhes.push(DetalheLoteAviculturaDto {
                lote_id: lote.id,
                identificador: lote.identificador.clone(),
                granja: lote.granja_nome.clone(),
                data_entrada: lote.data_entrada,
                idade_atual_dias: idade_dias,
                status: lote.status.clone(),
                quantidade_inicial: lote.quantidade_aves_inicial,
                quantidade_atual: lote.quantidade_aves_atual,
                mortalidade_percentual: mortalidade_pct,
                viabilidade,
                densidade_atual: densidade,
                ganho_medio_diario: gmd,
                consumo_racao_total_kg: racao_total,
                consumo_racao_media_por_ave: racao_media_por_ave,
                consumo_agua_total_litros: agua_total,
                consumo_agua_media_por_ave: agua_media_por_ave,
                relacao_agua_racao,
                conversao_alimentar: ca,
                iep,
                eventos_sanitarios_total: eventos.total.unwrap_or(0) as i32,
                eventos_vacinacoes: eventos.vacinacoes.unwrap_or(0) as i32,
                eventos_medicacoes: eventos.medicacoes.unwrap_or(0) as i32,
                eventos_custo_total: eventos.custo_total.unwrap_or(Decimal::ZERO),
            });
        }

        let n = lotes.len() as i64;
        let mortalidade_media = if n > 0 {
            mortalidades.iter().sum::<Decimal>() / Decimal::from(n)
        } else {
            Decimal::ZERO
        };
        let viabilidade_media = if n > 0 {
            viabilidades.iter().sum::<Decimal>() / Decimal::from(n)
        } else {
            Decimal::ZERO
        };

        let resumo = ResumoGeralAviculturaDto {
            total_aves_alojadas,
            total_aves_atuais,
            mortalidade_media,
            viabilidade_media,
            consumo_total_racao,
            consumo_total_agua,
            total_eventos_sanitarios,
            custo_total_sanitario,
        };

        let benchmarks = BenchmarksAviculturaDto {
            melhor_conversao_alimentar: cas.iter().copied().filter(|c| *c > Decimal::ZERO).min().unwrap_or(Decimal::ZERO),
            melhor_iep: ieps.iter().copied().max().unwrap_or(Decimal::ZERO),
            melhor_viabilidade: viabilidades.iter().copied().max().unwrap_or(Decimal::ZERO),
            menor_mortalidade: mortalidades.iter().copied().min().unwrap_or(Decimal::ZERO),
        };

        Ok(RelatorioAviculturaDto {
            periodo_inicio: inicio,
            periodo_fim: fim,
            total_lotes: lotes.len() as i32,
            data_geracao: Utc::now(),
            resumo_geral: resumo,
            detalhes_por_lote: detalhes,
            benchmarks,
        })
    }

    // ======================================================================
    // desempenho_lote (RELA-06)
    // ======================================================================
    pub async fn desempenho_lote(
        pool: &PgPool,
        lote_id: i32,
    ) -> Result<RelatorioDesempenhoLoteDto, AppError> {
        // Query lote principal
        let lote = sqlx::query_as::<_, AviculturaLoteRow>(
            r#"SELECT
                l."Id", l."Identificador", l."DataEntrada",
                l."QuantidadeAvesInicial", l."QuantidadeAvesAtual",
                l."AreaGalpao", l."Linhagem", l."OrigemPintinhos",
                l."Status", l."GranjaId",
                g."Nome" AS "GranjaNome"
            FROM "Lotes" l
            INNER JOIN "Granjas" g ON g."Id" = l."GranjaId"
            WHERE l."Id" = $1"#,
        )
        .bind(lote_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Lote nao encontrado".into()))?;

        let idade_dias = (Utc::now().date_naive() - lote.data_entrada.date_naive()).num_days();
        let mortalidade_total = lote.quantidade_aves_inicial - lote.quantidade_aves_atual;
        let mortalidade_pct = if lote.quantidade_aves_inicial > 0 {
            Decimal::from(mortalidade_total) / Decimal::from(lote.quantidade_aves_inicial) * Decimal::from(100)
        } else {
            Decimal::ZERO
        };
        let viabilidade = Decimal::from(100) - mortalidade_pct;
        let densidade = match lote.area_galpao {
            Some(area) if area > Decimal::ZERO => Decimal::from(lote.quantidade_aves_atual) / area,
            _ => Decimal::ZERO,
        };

        // Consumo racao total for CA calculation
        let racao_row = sqlx::query_as::<_, ConsumoRacaoRow>(
            r#"SELECT COALESCE(SUM("QuantidadeKg"), 0) AS "total_kg"
            FROM "ConsumosRacao" WHERE "LoteId" = $1"#,
        )
        .bind(lote_id)
        .fetch_one(pool)
        .await?;
        let racao_total = racao_row.total_kg.unwrap_or(Decimal::ZERO);

        // Pesagens for CA/IEP
        let pesagens_for_ca = sqlx::query_as::<_, PesagemRow>(
            r#"SELECT "PesoMedioGramas", "GanhoSemanal"
            FROM "PesagensSemanais" WHERE "LoteId" = $1
            ORDER BY "DataPesagem" DESC"#,
        )
        .bind(lote_id)
        .fetch_all(pool)
        .await?;

        let ca = if !pesagens_for_ca.is_empty() && racao_total > Decimal::ZERO {
            let peso_recente = pesagens_for_ca[0].peso_medio_gramas;
            let ganho_por_ave = peso_recente - Decimal::from(45);
            let ganho_total_kg = (ganho_por_ave * Decimal::from(lote.quantidade_aves_atual)) / Decimal::from(1000);
            if ganho_total_kg > Decimal::ZERO { racao_total / ganho_total_kg } else { Decimal::ZERO }
        } else {
            Decimal::ZERO
        };

        let iep = if !pesagens_for_ca.is_empty() && idade_dias > 0 && ca > Decimal::ZERO {
            let peso_recente = pesagens_for_ca[0].peso_medio_gramas;
            let ganho_peso_kg = (peso_recente - Decimal::from(45)) / Decimal::from(1000);
            (ganho_peso_kg * viabilidade * Decimal::from(100)) / (ca * Decimal::from(idade_dias))
        } else {
            Decimal::ZERO
        };

        let performance = PerformanceLoteDto {
            quantidade_inicial: lote.quantidade_aves_inicial,
            quantidade_atual: lote.quantidade_aves_atual,
            mortalidade_total,
            mortalidade_percentual: mortalidade_pct,
            viabilidade,
            densidade_atual: densidade,
            conversao_alimentar: ca,
            iep,
        };

        // Curva de crescimento
        let pesagens_detalhe = sqlx::query_as::<_, PesagemDetalheRow>(
            r#"SELECT "SemanaVida", "IdadeDias", "PesoMedioGramas",
                "GanhoSemanal", "CoeficienteVariacao", "QuantidadeAmostrada"
            FROM "PesagensSemanais" WHERE "LoteId" = $1
            ORDER BY "SemanaVida" ASC"#,
        )
        .bind(lote_id)
        .fetch_all(pool)
        .await?;

        let curva_crescimento: Vec<CurvaCrescimentoItemDto> = pesagens_detalhe
            .into_iter()
            .map(|p| {
                let ganho_semanal = p.ganho_semanal.unwrap_or(Decimal::ZERO);
                let gmd = ganho_semanal / Decimal::from(7);
                let uniformidade = match p.coeficiente_variacao {
                    Some(cv) => Decimal::from(100) - cv,
                    None => Decimal::ZERO,
                };
                CurvaCrescimentoItemDto {
                    semana: p.semana_vida,
                    idade_dias: p.idade_dias,
                    peso_medio: p.peso_medio_gramas,
                    ganho_semanal,
                    ganho_medio_diario: gmd,
                    uniformidade,
                    quantidade_amostrada: p.quantidade_amostrada,
                }
            })
            .collect();

        // Consumo racao detalhado, grouped by tipo_racao
        let consumo_racao_rows = sqlx::query_as::<_, ConsumoRacaoDetalheRow>(
            r#"SELECT "Data", "QuantidadeKg", "TipoRacao", "AvesVivas"
            FROM "ConsumosRacao" WHERE "LoteId" = $1
            ORDER BY "Data" ASC"#,
        )
        .bind(lote_id)
        .fetch_all(pool)
        .await?;

        // Group by tipo_racao
        let mut racao_groups: std::collections::HashMap<String, Vec<ConsumoRacaoDetalheRow>> =
            std::collections::HashMap::new();
        for row in consumo_racao_rows {
            racao_groups
                .entry(row.tipo_racao.clone())
                .or_default()
                .push(row);
        }

        let consumo_racao: Vec<ConsumoRacaoGroupDto> = racao_groups
            .into_iter()
            .map(|(tipo, registros)| {
                let total_kg: Decimal = registros.iter().map(|r| r.quantidade_kg).sum();
                let media_por_ave = if !registros.is_empty() {
                    let total_consumo_por_ave: Decimal = registros
                        .iter()
                        .map(|r| {
                            if r.aves_vivas > 0 {
                                (r.quantidade_kg * Decimal::from(1000)) / Decimal::from(r.aves_vivas)
                            } else {
                                Decimal::ZERO
                            }
                        })
                        .sum();
                    total_consumo_por_ave / Decimal::from(registros.len() as i64)
                } else {
                    Decimal::ZERO
                };
                let registros_por_dia: Vec<ConsumoRacaoDiaDto> = registros
                    .iter()
                    .map(|r| {
                        let consumo_por_ave = if r.aves_vivas > 0 {
                            (r.quantidade_kg * Decimal::from(1000)) / Decimal::from(r.aves_vivas)
                        } else {
                            Decimal::ZERO
                        };
                        ConsumoRacaoDiaDto {
                            data: r.data,
                            quantidade_kg: r.quantidade_kg,
                            aves_vivas: r.aves_vivas,
                            consumo_por_ave,
                        }
                    })
                    .collect();
                ConsumoRacaoGroupDto {
                    tipo_racao: tipo,
                    total_kg,
                    media_por_ave,
                    registros_por_dia,
                }
            })
            .collect();

        // Consumo agua
        let consumo_agua_rows = sqlx::query_as::<_, ConsumoAguaDetalheRow>(
            r#"SELECT "Data", "QuantidadeLitros", "AvesVivas", "TemperaturaAmbiente"
            FROM "ConsumosAgua" WHERE "LoteId" = $1
            ORDER BY "Data" ASC"#,
        )
        .bind(lote_id)
        .fetch_all(pool)
        .await?;

        let consumo_agua: Vec<ConsumoAguaItemDto> = consumo_agua_rows
            .into_iter()
            .map(|r| {
                let consumo_por_ave = if r.aves_vivas > 0 {
                    (r.quantidade_litros * Decimal::from(1000)) / Decimal::from(r.aves_vivas)
                } else {
                    Decimal::ZERO
                };
                ConsumoAguaItemDto {
                    data: r.data,
                    quantidade_litros: r.quantidade_litros,
                    aves_vivas: r.aves_vivas,
                    consumo_por_ave,
                    temperatura_ambiente: r.temperatura_ambiente,
                }
            })
            .collect();

        // Historico sanitario
        let eventos_rows = sqlx::query_as::<_, EventoSanitarioDetalheRow>(
            r#"SELECT "Data", "TipoEvento", "Produto", "ViaAdministracao",
                "AvesTratadas", "Custo", "PeriodoCarenciaDias", "ResponsavelAplicacao"
            FROM "EventosSanitarios" WHERE "LoteId" = $1
            ORDER BY "Data" ASC"#,
        )
        .bind(lote_id)
        .fetch_all(pool)
        .await?;

        let historico_sanitario: Vec<HistoricoSanitarioItemDto> = eventos_rows
            .into_iter()
            .map(|e| HistoricoSanitarioItemDto {
                data: e.data,
                tipo_evento: e.tipo_evento,
                produto: e.produto,
                via_administracao: e.via_administracao,
                aves_tratadas: e.aves_tratadas,
                custo: e.custo,
                periodo_carencia: e.periodo_carencia_dias,
                responsavel: e.responsavel_aplicacao,
            })
            .collect();

        // Mortalidade
        let mortalidade_rows = sqlx::query_as::<_, MortalidadeRow>(
            r#"SELECT "Data", "IdadeDias", "QuantidadeMortas", "CausaPrincipal", "AvesVivas"
            FROM "RegistrosMortalidade" WHERE "LoteId" = $1
            ORDER BY "Data" ASC"#,
        )
        .bind(lote_id)
        .fetch_all(pool)
        .await?;

        let analise_mortalidade: Vec<AnaliseMortalidadeItemDto> = mortalidade_rows
            .into_iter()
            .map(|m| {
                let percentual_dia = if m.aves_vivas > 0 {
                    Decimal::from(m.quantidade_mortas) / Decimal::from(m.aves_vivas) * Decimal::from(100)
                } else {
                    Decimal::ZERO
                };
                AnaliseMortalidadeItemDto {
                    data: m.data,
                    idade_dias: m.idade_dias,
                    quantidade_mortas: m.quantidade_mortas,
                    percentual_dia,
                    causa_principal: m.causa_principal,
                    aves_vivas: m.aves_vivas,
                }
            })
            .collect();

        // Qualidade ambiental
        let qa_rows = sqlx::query_as::<_, QualidadeArRow>(
            r#"SELECT "DataHora", "TemperaturaAr", "UmidadeRelativa",
                "NH3_ppm", "CO2_ppm", "O2_percentual",
                "VelocidadeAr_ms", "Luminosidade_lux"
            FROM "MedicoesQualidadeAr" WHERE "LoteId" = $1
            ORDER BY "DataHora" ASC"#,
        )
        .bind(lote_id)
        .fetch_all(pool)
        .await?;

        let qualidade_ambiental: Vec<QualidadeAmbientalItemDto> = qa_rows
            .into_iter()
            .map(|q| {
                // ParametrosAceitaveis: NH3 <= 25 AND temp between 18-33 AND CO2 <= 3000
                let nh3_ok = q.nh3_ppm.map_or(true, |v| v <= Decimal::from(25));
                let temp_ok = q.temperatura_ar.map_or(true, |v| {
                    v >= Decimal::from(18) && v <= Decimal::from(33)
                });
                let co2_ok = q.co2_ppm.map_or(true, |v| v <= Decimal::from(3000));
                let parametros_ok = nh3_ok && temp_ok && co2_ok;

                QualidadeAmbientalItemDto {
                    data_hora: q.data_hora,
                    temperatura_ar: q.temperatura_ar,
                    umidade_relativa: q.umidade_relativa,
                    nh3_ppm: q.nh3_ppm,
                    co2_ppm: q.co2_ppm,
                    o2_percentual: q.o2_percentual,
                    velocidade_ar: q.velocidade_ar_ms,
                    luminosidade: q.luminosidade_lux,
                    parametros_ok,
                }
            })
            .collect();

        Ok(RelatorioDesempenhoLoteDto {
            lote_id: lote.id,
            identificador: lote.identificador,
            granja: lote.granja_nome,
            data_entrada: lote.data_entrada,
            idade_atual_dias: idade_dias,
            status: lote.status,
            linhagem: lote.linhagem,
            origem_pintinhos: lote.origem_pintinhos,
            performance,
            curva_crescimento,
            consumo_racao,
            consumo_agua,
            historico_sanitario,
            analise_mortalidade,
            qualidade_ambiental,
            data_geracao: Utc::now(),
        })
    }
}
