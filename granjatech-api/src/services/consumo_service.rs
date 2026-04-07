use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::PgPool;

use crate::dto::consumo::*;
use crate::errors::AppError;
use crate::models::consumo::{ConsumoAgua, ConsumoRacao};
use crate::services::auditoria_service::AuditoriaService;

pub struct ConsumoService;

impl ConsumoService {
    /// Compute feed consumption per bird in grams
    fn consumo_por_ave_gramas(quantidade_kg: Decimal, aves_vivas: i32) -> Decimal {
        if aves_vivas > 0 {
            (quantidade_kg * Decimal::from(1000)) / Decimal::from(aves_vivas)
        } else {
            Decimal::ZERO
        }
    }

    /// Compute water consumption per bird in ml
    fn consumo_por_ave_ml(quantidade_litros: Decimal, aves_vivas: i32) -> Decimal {
        if aves_vivas > 0 {
            (quantidade_litros * Decimal::from(1000)) / Decimal::from(aves_vivas)
        } else {
            Decimal::ZERO
        }
    }

    fn racao_to_response(r: ConsumoRacao) -> ConsumoRacaoResponseDto {
        let consumo = Self::consumo_por_ave_gramas(r.quantidade_kg, r.aves_vivas);
        ConsumoRacaoResponseDto {
            id: r.id,
            lote_id: r.lote_id,
            data: r.data,
            quantidade_kg: r.quantidade_kg,
            tipo_racao: r.tipo_racao,
            aves_vivas: r.aves_vivas,
            consumo_por_ave_gramas: consumo,
            observacoes: r.observacoes,
            data_criacao: r.data_criacao,
        }
    }

    fn agua_to_response(a: ConsumoAgua) -> ConsumoAguaResponseDto {
        let consumo = Self::consumo_por_ave_ml(a.quantidade_litros, a.aves_vivas);
        ConsumoAguaResponseDto {
            id: a.id,
            lote_id: a.lote_id,
            data: a.data,
            quantidade_litros: a.quantidade_litros,
            aves_vivas: a.aves_vivas,
            consumo_por_ave_ml: consumo,
            temperatura_ambiente: a.temperatura_ambiente,
            observacoes: a.observacoes,
            data_criacao: a.data_criacao,
        }
    }

    /// POST /api/consumo/racao -- registra consumo de racao
    pub async fn create_racao(
        pool: &PgPool,
        dto: &CreateConsumoRacaoDto,
        user_id: i32,
        user_email: &str,
    ) -> Result<ConsumoRacaoResponseDto, AppError> {
        let now = Utc::now();

        let row = sqlx::query_as::<_, ConsumoRacao>(
            r#"INSERT INTO "ConsumosRacao" ("LoteId", "Data", "QuantidadeKg", "TipoRacao", "AvesVivas", "Observacoes", "DataCriacao")
               VALUES ($1, $2, $3, $4, $5, $6, $7)
               RETURNING "Id", "LoteId", "Data", "QuantidadeKg", "TipoRacao", "AvesVivas", "Observacoes", "DataCriacao""#,
        )
        .bind(dto.lote_id)
        .bind(dto.data)
        .bind(dto.quantidade_kg)
        .bind(&dto.tipo_racao)
        .bind(dto.aves_vivas)
        .bind(&dto.observacoes)
        .bind(now)
        .fetch_one(pool)
        .await?;

        AuditoriaService::registrar_log(
            pool,
            user_id,
            user_email,
            "REGISTRO_CONSUMO_RACAO",
            &format!(
                "Consumo de racao registrado para lote {} ({} kg).",
                dto.lote_id, dto.quantidade_kg
            ),
        )
        .await?;

        Ok(Self::racao_to_response(row))
    }

    /// POST /api/consumo/agua -- registra consumo de agua
    pub async fn create_agua(
        pool: &PgPool,
        dto: &CreateConsumoAguaDto,
        user_id: i32,
        user_email: &str,
    ) -> Result<ConsumoAguaResponseDto, AppError> {
        let now = Utc::now();

        let row = sqlx::query_as::<_, ConsumoAgua>(
            r#"INSERT INTO "ConsumosAgua" ("LoteId", "Data", "QuantidadeLitros", "AvesVivas", "TemperaturaAmbiente", "Observacoes", "DataCriacao")
               VALUES ($1, $2, $3, $4, $5, $6, $7)
               RETURNING "Id", "LoteId", "Data", "QuantidadeLitros", "AvesVivas", "TemperaturaAmbiente", "Observacoes", "DataCriacao""#,
        )
        .bind(dto.lote_id)
        .bind(dto.data)
        .bind(dto.quantidade_litros)
        .bind(dto.aves_vivas)
        .bind(dto.temperatura_ambiente)
        .bind(&dto.observacoes)
        .bind(now)
        .fetch_one(pool)
        .await?;

        AuditoriaService::registrar_log(
            pool,
            user_id,
            user_email,
            "REGISTRO_CONSUMO_AGUA",
            &format!(
                "Consumo de agua registrado para lote {} ({} litros).",
                dto.lote_id, dto.quantidade_litros
            ),
        )
        .await?;

        Ok(Self::agua_to_response(row))
    }

    /// GET /api/consumo/racao/{loteId} -- lista consumos de racao
    pub async fn list_racao(
        pool: &PgPool,
        lote_id: i32,
    ) -> Result<Vec<ConsumoRacaoResponseDto>, AppError> {
        let rows = sqlx::query_as::<_, ConsumoRacao>(
            r#"SELECT "Id", "LoteId", "Data", "QuantidadeKg", "TipoRacao", "AvesVivas", "Observacoes", "DataCriacao"
               FROM "ConsumosRacao"
               WHERE "LoteId" = $1
               ORDER BY "Data" DESC"#,
        )
        .bind(lote_id)
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(Self::racao_to_response).collect())
    }

    /// GET /api/consumo/agua/{loteId} -- lista consumos de agua
    pub async fn list_agua(
        pool: &PgPool,
        lote_id: i32,
    ) -> Result<Vec<ConsumoAguaResponseDto>, AppError> {
        let rows = sqlx::query_as::<_, ConsumoAgua>(
            r#"SELECT "Id", "LoteId", "Data", "QuantidadeLitros", "AvesVivas", "TemperaturaAmbiente", "Observacoes", "DataCriacao"
               FROM "ConsumosAgua"
               WHERE "LoteId" = $1
               ORDER BY "Data" DESC"#,
        )
        .bind(lote_id)
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(Self::agua_to_response).collect())
    }

    /// GET /api/consumo/resumo/{loteId} -- resumo de consumo do lote
    pub async fn resumo(
        pool: &PgPool,
        lote_id: i32,
    ) -> Result<serde_json::Value, AppError> {
        // Aggregate racao
        #[derive(sqlx::FromRow)]
        struct RacaoAgg {
            total_kg: Option<Decimal>,
            media_dia_kg: Option<Decimal>,
            ultima_data: Option<DateTime<Utc>>,
        }

        let racao_agg = sqlx::query_as::<_, RacaoAgg>(
            r#"SELECT
                 SUM("QuantidadeKg") AS total_kg,
                 AVG("QuantidadeKg") AS media_dia_kg,
                 MAX("Data") AS ultima_data
               FROM "ConsumosRacao"
               WHERE "LoteId" = $1"#,
        )
        .bind(lote_id)
        .fetch_one(pool)
        .await?;

        // Aggregate agua
        #[derive(sqlx::FromRow)]
        struct AguaAgg {
            total_litros: Option<Decimal>,
            media_dia_litros: Option<Decimal>,
            ultima_data: Option<DateTime<Utc>>,
        }

        let agua_agg = sqlx::query_as::<_, AguaAgg>(
            r#"SELECT
                 SUM("QuantidadeLitros") AS total_litros,
                 AVG("QuantidadeLitros") AS media_dia_litros,
                 MAX("Data") AS ultima_data
               FROM "ConsumosAgua"
               WHERE "LoteId" = $1"#,
        )
        .bind(lote_id)
        .fetch_one(pool)
        .await?;

        Ok(serde_json::json!({
            "totalRacaoKg": racao_agg.total_kg.unwrap_or(Decimal::ZERO),
            "totalAguaLitros": agua_agg.total_litros.unwrap_or(Decimal::ZERO),
            "mediaRacaoDiaKg": racao_agg.media_dia_kg.unwrap_or(Decimal::ZERO),
            "mediaAguaDiaLitros": agua_agg.media_dia_litros.unwrap_or(Decimal::ZERO),
            "ultimaDataRacao": racao_agg.ultima_data,
            "ultimaDataAgua": agua_agg.ultima_data,
        }))
    }
}
