use chrono::Utc;
use rust_decimal::Decimal;
use sqlx::PgPool;

use crate::dto::sanitario::CreateEventoSanitarioDto;
use crate::errors::AppError;
use crate::models::evento_sanitario::EventoSanitario;
use crate::services::auditoria_service::AuditoriaService;

pub struct SanitarioService;

impl SanitarioService {
    /// POST /api/sanitario -- registra evento sanitario
    pub async fn create(
        pool: &PgPool,
        dto: &CreateEventoSanitarioDto,
        user_id: i32,
        user_email: &str,
    ) -> Result<EventoSanitario, AppError> {
        let now = Utc::now();

        let row = sqlx::query_as::<_, EventoSanitario>(
            r#"INSERT INTO "EventosSanitarios" ("LoteId", "Data", "TipoEvento", "Produto",
                 "LoteProduto", "Dosagem", "ViaAdministracao", "AvesTratadas",
                 "DuracaoTratamentoDias", "PeriodoCarenciaDias", "ResponsavelAplicacao",
                 "Sintomas", "Observacoes", "Custo", "DataCriacao")
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
               RETURNING "Id", "LoteId", "Data", "TipoEvento", "Produto",
                 "LoteProduto", "Dosagem", "ViaAdministracao", "AvesTratadas",
                 "DuracaoTratamentoDias", "PeriodoCarenciaDias", "ResponsavelAplicacao",
                 "Sintomas", "Observacoes", "Custo", "DataCriacao""#,
        )
        .bind(dto.lote_id)
        .bind(dto.data)
        .bind(&dto.tipo_evento)
        .bind(&dto.produto)
        .bind(&dto.lote_produto)
        .bind(&dto.dosagem)
        .bind(&dto.via_administracao)
        .bind(dto.aves_tratadas)
        .bind(dto.duracao_tratamento_dias)
        .bind(dto.periodo_carencia_dias)
        .bind(&dto.responsavel_aplicacao)
        .bind(&dto.sintomas)
        .bind(&dto.observacoes)
        .bind(dto.custo)
        .bind(now)
        .fetch_one(pool)
        .await?;

        AuditoriaService::registrar_log(
            pool,
            user_id,
            user_email,
            "REGISTRO_EVENTO_SANITARIO",
            &format!(
                "Evento sanitario '{}' registrado para lote {} (produto: {}).",
                dto.tipo_evento, dto.lote_id, dto.produto
            ),
        )
        .await?;

        Ok(row)
    }

    /// GET /api/sanitario/{loteId} -- lista eventos sanitarios do lote
    pub async fn list(
        pool: &PgPool,
        lote_id: i32,
        tipo_evento: Option<&str>,
    ) -> Result<Vec<EventoSanitario>, AppError> {
        let rows = if let Some(tipo) = tipo_evento {
            sqlx::query_as::<_, EventoSanitario>(
                r#"SELECT "Id", "LoteId", "Data", "TipoEvento", "Produto",
                     "LoteProduto", "Dosagem", "ViaAdministracao", "AvesTratadas",
                     "DuracaoTratamentoDias", "PeriodoCarenciaDias", "ResponsavelAplicacao",
                     "Sintomas", "Observacoes", "Custo", "DataCriacao"
                   FROM "EventosSanitarios"
                   WHERE "LoteId" = $1 AND "TipoEvento" = $2
                   ORDER BY "Data" DESC"#,
            )
            .bind(lote_id)
            .bind(tipo)
            .fetch_all(pool)
            .await?
        } else {
            sqlx::query_as::<_, EventoSanitario>(
                r#"SELECT "Id", "LoteId", "Data", "TipoEvento", "Produto",
                     "LoteProduto", "Dosagem", "ViaAdministracao", "AvesTratadas",
                     "DuracaoTratamentoDias", "PeriodoCarenciaDias", "ResponsavelAplicacao",
                     "Sintomas", "Observacoes", "Custo", "DataCriacao"
                   FROM "EventosSanitarios"
                   WHERE "LoteId" = $1
                   ORDER BY "Data" DESC"#,
            )
            .bind(lote_id)
            .fetch_all(pool)
            .await?
        };

        Ok(rows)
    }

    /// GET /api/sanitario/resumo/{loteId} -- resumo sanitario do lote
    pub async fn resumo(
        pool: &PgPool,
        lote_id: i32,
    ) -> Result<serde_json::Value, AppError> {
        // Count by tipo_evento
        #[derive(sqlx::FromRow)]
        struct TipoCount {
            tipo_evento: String,
            count: i64,
        }

        let contagem = sqlx::query_as::<_, TipoCount>(
            r#"SELECT "TipoEvento" AS tipo_evento, COUNT(*) AS count
               FROM "EventosSanitarios"
               WHERE "LoteId" = $1
               GROUP BY "TipoEvento""#,
        )
        .bind(lote_id)
        .fetch_all(pool)
        .await?;

        let contagem_json: serde_json::Value = contagem
            .iter()
            .map(|tc| serde_json::json!({ "tipoEvento": tc.tipo_evento, "quantidade": tc.count }))
            .collect::<Vec<_>>()
            .into();

        // Total cost
        let custo_total: Option<Decimal> = sqlx::query_scalar(
            r#"SELECT SUM("Custo") FROM "EventosSanitarios" WHERE "LoteId" = $1"#,
        )
        .bind(lote_id)
        .fetch_one(pool)
        .await
        .unwrap_or(None);

        // Cost per bird (if lote has quantity info)
        let qtd_aves: Option<i32> = sqlx::query_scalar(
            r#"SELECT "QuantidadeAvesInicial" FROM "Lotes" WHERE "Id" = $1"#,
        )
        .bind(lote_id)
        .fetch_optional(pool)
        .await
        .unwrap_or(None);

        let custo_por_ave = match (custo_total, qtd_aves) {
            (Some(custo), Some(aves)) if aves > 0 => Some(custo / Decimal::from(aves)),
            _ => None,
        };

        // Upcoming carencia actions (events with future carencia dates)
        #[derive(sqlx::FromRow, serde::Serialize)]
        struct ProximaAcao {
            #[sqlx(rename = "Id")]
            id: i32,
            #[sqlx(rename = "TipoEvento")]
            tipo_evento: String,
            #[sqlx(rename = "Produto")]
            produto: String,
            #[sqlx(rename = "DataFimCarencia")]
            data_fim_carencia: chrono::DateTime<Utc>,
        }

        let proximas = sqlx::query_as::<_, ProximaAcao>(
            r#"SELECT "Id", "TipoEvento", "Produto",
                      ("Data" + "PeriodoCarenciaDias" * INTERVAL '1 day') AS "DataFimCarencia"
               FROM "EventosSanitarios"
               WHERE "LoteId" = $1
                 AND "PeriodoCarenciaDias" IS NOT NULL
                 AND ("Data" + "PeriodoCarenciaDias" * INTERVAL '1 day') > NOW()
               ORDER BY "DataFimCarencia" ASC"#,
        )
        .bind(lote_id)
        .fetch_all(pool)
        .await
        .unwrap_or_default();

        Ok(serde_json::json!({
            "contagemPorTipo": contagem_json,
            "custoTotal": custo_total.unwrap_or(Decimal::ZERO),
            "custoPorAve": custo_por_ave,
            "proximasAcoes": proximas,
        }))
    }

    /// GET /api/sanitario/cronograma-vacinacao -- cronograma padrao
    pub fn cronograma_vacinacao() -> serde_json::Value {
        serde_json::json!([
            {"dia": 1, "vacina": "Marek", "via": "Subcutanea", "observacao": "No incubatorio"},
            {"dia": 7, "vacina": "Newcastle + Bronquite", "via": "Ocular", "observacao": "Primeira dose"},
            {"dia": 14, "vacina": "Gumboro", "via": "Agua", "observacao": "Cepa intermediaria"},
            {"dia": 21, "vacina": "Newcastle + Bronquite", "via": "Agua", "observacao": "Reforco"},
            {"dia": 28, "vacina": "Gumboro", "via": "Agua", "observacao": "Reforco"}
        ])
    }
}
