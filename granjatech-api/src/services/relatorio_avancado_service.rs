use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use rust_decimal::prelude::*;
use sqlx::PgPool;

use crate::dto::financeiro::*;
use crate::dto::relatorios::*;
use crate::errors::AppError;

// === Helper row structs ===

#[derive(Debug, sqlx::FromRow)]
struct FinanceItemRow {
    #[sqlx(rename = "Data")]
    data: DateTime<Utc>,
    #[sqlx(rename = "Tipo")]
    tipo: String,
    #[sqlx(rename = "Descricao")]
    descricao: String,
    #[sqlx(rename = "Valor")]
    valor: Decimal,
}

#[derive(Debug, sqlx::FromRow)]
struct ConsumoRacaoGeral {
    #[sqlx(rename = "Data")]
    data: DateTime<Utc>,
    #[sqlx(rename = "QuantidadeKg")]
    quantidade_kg: Decimal,
    #[sqlx(rename = "AvesVivas")]
    aves_vivas: i32,
}

#[derive(Debug, sqlx::FromRow)]
struct ConsumoAguaGeral {
    #[sqlx(rename = "Data")]
    data: DateTime<Utc>,
    #[sqlx(rename = "QuantidadeLitros")]
    quantidade_litros: Decimal,
    #[sqlx(rename = "AvesVivas")]
    aves_vivas: i32,
}

#[derive(Debug, sqlx::FromRow)]
struct PesagemGeralRow {
    #[sqlx(rename = "DataPesagem")]
    data_pesagem: DateTime<Utc>,
    #[sqlx(rename = "PesoMedioGramas")]
    peso_medio_gramas: Decimal,
    #[sqlx(rename = "QuantidadeAmostrada")]
    quantidade_amostrada: i32,
}

#[derive(Debug, sqlx::FromRow)]
struct SanitarioGeralRow {
    #[sqlx(rename = "Data")]
    data: DateTime<Utc>,
    #[sqlx(rename = "TipoEvento")]
    tipo_evento: String,
    #[sqlx(rename = "Produto")]
    produto: String,
    #[sqlx(rename = "ViaAdministracao")]
    via_administracao: Option<String>,
}

#[derive(Debug, sqlx::FromRow)]
struct SensorGeralRow {
    #[sqlx(rename = "Timestamp")]
    timestamp: DateTime<Utc>,
    #[sqlx(rename = "Tipo")]
    tipo: String,
    #[sqlx(rename = "Valor")]
    valor: Decimal,
}

pub struct RelatorioAvancadoService;

impl RelatorioAvancadoService {
    // ======================================================================
    // financeiro (advanced)
    // ======================================================================
    pub async fn financeiro(
        pool: &PgPool,
        granja_id: i32,
        inicio: DateTime<Utc>,
        fim: DateTime<Utc>,
    ) -> Result<FinanceReportDto, AppError> {
        let rows = sqlx::query_as::<_, FinanceItemRow>(
            r#"SELECT tf."Data", tf."Tipo", tf."Descricao", tf."Valor"
            FROM "TransacoesFinanceiras" tf
            LEFT JOIN "Lotes" l ON l."Id" = tf."LoteId"
            WHERE tf."Data" >= $1 AND tf."Data" < $2
              AND (l."GranjaId" = $3 OR tf."LoteId" IS NULL)
            ORDER BY tf."Data" ASC"#,
        )
        .bind(inicio)
        .bind(fim)
        .bind(granja_id)
        .fetch_all(pool)
        .await?;

        let itens: Vec<FinanceReportItemDto> = rows
            .into_iter()
            .map(|r| FinanceReportItemDto {
                data: r.data,
                categoria: r.tipo.to_lowercase(),
                descricao: r.descricao,
                valor: r.valor,
            })
            .collect();

        let entradas: Decimal = itens
            .iter()
            .filter(|i| i.categoria == "entrada")
            .map(|i| i.valor)
            .sum();
        let saidas: Decimal = itens
            .iter()
            .filter(|i| i.categoria == "saida")
            .map(|i| i.valor)
            .sum();

        Ok(FinanceReportDto {
            granja_id,
            inicio,
            fim,
            total_entradas: entradas,
            total_saidas: saidas,
            saldo: entradas - saidas,
            itens,
        })
    }

    // ======================================================================
    // geral
    // ======================================================================
    pub async fn geral(
        pool: &PgPool,
        granja_id: i32,
        inicio: DateTime<Utc>,
        fim: DateTime<Utc>,
    ) -> Result<GeralReportDto, AppError> {
        // Consumo racao
        let racao_rows = sqlx::query_as::<_, ConsumoRacaoGeral>(
            r#"SELECT cr."Data", cr."QuantidadeKg", cr."AvesVivas"
            FROM "ConsumosRacao" cr
            INNER JOIN "Lotes" l ON l."Id" = cr."LoteId"
            WHERE l."GranjaId" = $1
              AND cr."Data" >= $2 AND cr."Data" < $3
            ORDER BY cr."Data" ASC"#,
        )
        .bind(granja_id)
        .bind(inicio)
        .bind(fim)
        .fetch_all(pool)
        .await?;

        // Consumo agua
        let agua_rows = sqlx::query_as::<_, ConsumoAguaGeral>(
            r#"SELECT ca."Data", ca."QuantidadeLitros", ca."AvesVivas"
            FROM "ConsumosAgua" ca
            INNER JOIN "Lotes" l ON l."Id" = ca."LoteId"
            WHERE l."GranjaId" = $1
              AND ca."Data" >= $2 AND ca."Data" < $3
            ORDER BY ca."Data" ASC"#,
        )
        .bind(granja_id)
        .bind(inicio)
        .bind(fim)
        .fetch_all(pool)
        .await?;

        // Group consumo by date, merging racao and agua
        let mut consumo_map: std::collections::BTreeMap<chrono::NaiveDate, (f64, f64, Vec<i32>)> =
            std::collections::BTreeMap::new();

        for r in &racao_rows {
            let day = r.data.date_naive();
            let entry = consumo_map.entry(day).or_insert((0.0, 0.0, Vec::new()));
            entry.0 += r.quantidade_kg.to_f64().unwrap_or(0.0);
            entry.2.push(r.aves_vivas.max(0));
        }
        for a in &agua_rows {
            let day = a.data.date_naive();
            let entry = consumo_map.entry(day).or_insert((0.0, 0.0, Vec::new()));
            entry.1 += a.quantidade_litros.to_f64().unwrap_or(0.0);
            entry.2.push(a.aves_vivas.max(0));
        }

        let consumo: Vec<ConsumoResumoDto> = consumo_map
            .into_iter()
            .map(|(day, (racao, agua, aves))| {
                let avg_aves = if aves.is_empty() {
                    0
                } else {
                    (aves.iter().map(|&a| a as i64).sum::<i64>() / aves.len() as i64) as i32
                };
                ConsumoResumoDto {
                    data: DateTime::from_naive_utc_and_offset(
                        day.and_hms_opt(0, 0, 0).unwrap(),
                        Utc,
                    ),
                    racao_kg: racao,
                    agua_litros: agua,
                    aves_vivas: avg_aves,
                }
            })
            .collect();

        // Pesagens
        let pesagem_rows = sqlx::query_as::<_, PesagemGeralRow>(
            r#"SELECT ps."DataPesagem", ps."PesoMedioGramas", ps."QuantidadeAmostrada"
            FROM "PesagensSemanais" ps
            INNER JOIN "Lotes" l ON l."Id" = ps."LoteId"
            WHERE l."GranjaId" = $1
              AND ps."DataPesagem" >= $2 AND ps."DataPesagem" < $3
            ORDER BY ps."DataPesagem" ASC"#,
        )
        .bind(granja_id)
        .bind(inicio)
        .bind(fim)
        .fetch_all(pool)
        .await?;

        let pesagens: Vec<PesagemResumoDto> = pesagem_rows
            .into_iter()
            .map(|p| PesagemResumoDto {
                data: p.data_pesagem,
                peso_medio_kg: p.peso_medio_gramas.to_f64().unwrap_or(0.0) / 1000.0,
                amostra: p.quantidade_amostrada,
            })
            .collect();

        // Sanitario
        let sanitario_rows = sqlx::query_as::<_, SanitarioGeralRow>(
            r#"SELECT es."Data", es."TipoEvento", es."Produto", es."ViaAdministracao"
            FROM "EventosSanitarios" es
            INNER JOIN "Lotes" l ON l."Id" = es."LoteId"
            WHERE l."GranjaId" = $1
              AND es."Data" >= $2 AND es."Data" < $3
            ORDER BY es."Data" ASC"#,
        )
        .bind(granja_id)
        .bind(inicio)
        .bind(fim)
        .fetch_all(pool)
        .await?;

        let sanitario: Vec<SanitarioResumoDto> = sanitario_rows
            .into_iter()
            .map(|e| SanitarioResumoDto {
                data: e.data,
                tipo_evento: e.tipo_evento,
                produto: e.produto,
                via: e.via_administracao,
            })
            .collect();

        // Sensores
        let sensor_rows = sqlx::query_as::<_, SensorGeralRow>(
            r#"SELECT ls."Timestamp", s."Tipo", ls."Valor"
            FROM "LeiturasSensores" ls
            INNER JOIN "Sensores" s ON s."Id" = ls."SensorId"
            WHERE s."GranjaId" = $1
              AND ls."Timestamp" >= $2 AND ls."Timestamp" < $3
            ORDER BY ls."Timestamp" ASC"#,
        )
        .bind(granja_id)
        .bind(inicio)
        .bind(fim)
        .fetch_all(pool)
        .await?;

        let sensores: Vec<SensorResumoDto> = sensor_rows
            .into_iter()
            .map(|s| SensorResumoDto {
                data: s.timestamp,
                tipo: s.tipo,
                valor: s.valor.to_f64().unwrap_or(0.0),
            })
            .collect();

        Ok(GeralReportDto {
            granja_id,
            inicio,
            fim,
            consumo,
            pesagens,
            sanitario,
            sensores,
        })
    }

    // ======================================================================
    // consumo (setor)
    // ======================================================================
    pub async fn consumo(
        pool: &PgPool,
        granja_id: i32,
        inicio: DateTime<Utc>,
        fim: DateTime<Utc>,
    ) -> Result<SetorReportDto<ConsumoResumoDto>, AppError> {
        let racao_rows = sqlx::query_as::<_, ConsumoRacaoGeral>(
            r#"SELECT cr."Data", cr."QuantidadeKg", cr."AvesVivas"
            FROM "ConsumosRacao" cr
            INNER JOIN "Lotes" l ON l."Id" = cr."LoteId"
            WHERE l."GranjaId" = $1
              AND cr."Data" >= $2 AND cr."Data" < $3
            ORDER BY cr."Data" ASC"#,
        )
        .bind(granja_id)
        .bind(inicio)
        .bind(fim)
        .fetch_all(pool)
        .await?;

        let agua_rows = sqlx::query_as::<_, ConsumoAguaGeral>(
            r#"SELECT ca."Data", ca."QuantidadeLitros", ca."AvesVivas"
            FROM "ConsumosAgua" ca
            INNER JOIN "Lotes" l ON l."Id" = ca."LoteId"
            WHERE l."GranjaId" = $1
              AND ca."Data" >= $2 AND ca."Data" < $3
            ORDER BY ca."Data" ASC"#,
        )
        .bind(granja_id)
        .bind(inicio)
        .bind(fim)
        .fetch_all(pool)
        .await?;

        let mut consumo_map: std::collections::BTreeMap<chrono::NaiveDate, (f64, f64, Vec<i32>)> =
            std::collections::BTreeMap::new();

        for r in &racao_rows {
            let day = r.data.date_naive();
            let entry = consumo_map.entry(day).or_insert((0.0, 0.0, Vec::new()));
            entry.0 += r.quantidade_kg.to_f64().unwrap_or(0.0);
            entry.2.push(r.aves_vivas.max(0));
        }
        for a in &agua_rows {
            let day = a.data.date_naive();
            let entry = consumo_map.entry(day).or_insert((0.0, 0.0, Vec::new()));
            entry.1 += a.quantidade_litros.to_f64().unwrap_or(0.0);
            entry.2.push(a.aves_vivas.max(0));
        }

        let itens: Vec<ConsumoResumoDto> = consumo_map
            .into_iter()
            .map(|(day, (racao, agua, aves))| {
                let avg_aves = if aves.is_empty() {
                    0
                } else {
                    (aves.iter().map(|&a| a as i64).sum::<i64>() / aves.len() as i64) as i32
                };
                ConsumoResumoDto {
                    data: DateTime::from_naive_utc_and_offset(
                        day.and_hms_opt(0, 0, 0).unwrap(),
                        Utc,
                    ),
                    racao_kg: racao,
                    agua_litros: agua,
                    aves_vivas: avg_aves,
                }
            })
            .collect();

        Ok(SetorReportDto {
            granja_id,
            setor: "consumo".into(),
            inicio,
            fim,
            itens,
        })
    }

    // ======================================================================
    // pesagem (setor)
    // ======================================================================
    pub async fn pesagem(
        pool: &PgPool,
        granja_id: i32,
        inicio: DateTime<Utc>,
        fim: DateTime<Utc>,
    ) -> Result<SetorReportDto<PesagemResumoDto>, AppError> {
        let rows = sqlx::query_as::<_, PesagemGeralRow>(
            r#"SELECT ps."DataPesagem", ps."PesoMedioGramas", ps."QuantidadeAmostrada"
            FROM "PesagensSemanais" ps
            INNER JOIN "Lotes" l ON l."Id" = ps."LoteId"
            WHERE l."GranjaId" = $1
              AND ps."DataPesagem" >= $2 AND ps."DataPesagem" < $3
            ORDER BY ps."DataPesagem" ASC"#,
        )
        .bind(granja_id)
        .bind(inicio)
        .bind(fim)
        .fetch_all(pool)
        .await?;

        let itens: Vec<PesagemResumoDto> = rows
            .into_iter()
            .map(|p| PesagemResumoDto {
                data: p.data_pesagem,
                peso_medio_kg: p.peso_medio_gramas.to_f64().unwrap_or(0.0) / 1000.0,
                amostra: p.quantidade_amostrada,
            })
            .collect();

        Ok(SetorReportDto {
            granja_id,
            setor: "pesagem".into(),
            inicio,
            fim,
            itens,
        })
    }

    // ======================================================================
    // sanitario (setor)
    // ======================================================================
    pub async fn sanitario(
        pool: &PgPool,
        granja_id: i32,
        inicio: DateTime<Utc>,
        fim: DateTime<Utc>,
    ) -> Result<SetorReportDto<SanitarioResumoDto>, AppError> {
        let rows = sqlx::query_as::<_, SanitarioGeralRow>(
            r#"SELECT es."Data", es."TipoEvento", es."Produto", es."ViaAdministracao"
            FROM "EventosSanitarios" es
            INNER JOIN "Lotes" l ON l."Id" = es."LoteId"
            WHERE l."GranjaId" = $1
              AND es."Data" >= $2 AND es."Data" < $3
            ORDER BY es."Data" ASC"#,
        )
        .bind(granja_id)
        .bind(inicio)
        .bind(fim)
        .fetch_all(pool)
        .await?;

        let itens: Vec<SanitarioResumoDto> = rows
            .into_iter()
            .map(|e| SanitarioResumoDto {
                data: e.data,
                tipo_evento: e.tipo_evento,
                produto: e.produto,
                via: e.via_administracao,
            })
            .collect();

        Ok(SetorReportDto {
            granja_id,
            setor: "sanitario".into(),
            inicio,
            fim,
            itens,
        })
    }

    // ======================================================================
    // sensores (setor)
    // ======================================================================
    pub async fn sensores(
        pool: &PgPool,
        granja_id: i32,
        inicio: DateTime<Utc>,
        fim: DateTime<Utc>,
    ) -> Result<SetorReportDto<SensorResumoDto>, AppError> {
        let rows = sqlx::query_as::<_, SensorGeralRow>(
            r#"SELECT ls."Timestamp", s."Tipo", ls."Valor"
            FROM "LeiturasSensores" ls
            INNER JOIN "Sensores" s ON s."Id" = ls."SensorId"
            WHERE s."GranjaId" = $1
              AND ls."Timestamp" >= $2 AND ls."Timestamp" < $3
            ORDER BY ls."Timestamp" ASC"#,
        )
        .bind(granja_id)
        .bind(inicio)
        .bind(fim)
        .fetch_all(pool)
        .await?;

        let itens: Vec<SensorResumoDto> = rows
            .into_iter()
            .map(|s| SensorResumoDto {
                data: s.timestamp,
                tipo: s.tipo,
                valor: s.valor.to_f64().unwrap_or(0.0),
            })
            .collect();

        Ok(SetorReportDto {
            granja_id,
            setor: "sensores".into(),
            inicio,
            fim,
            itens,
        })
    }
}
