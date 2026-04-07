use rust_decimal::Decimal;
use sqlx::PgPool;

use crate::dto::pesagem::*;
use crate::errors::AppError;
use crate::models::pesagem_semanal::PesagemSemanal;
use crate::services::auditoria_service::AuditoriaService;

pub struct PesagemService;

impl PesagemService {
    /// Compute ganho_medio_diario from ganho_semanal
    fn ganho_medio_diario(ganho_semanal: Option<Decimal>) -> Decimal {
        ganho_semanal
            .map(|g| g / Decimal::from(7))
            .unwrap_or(Decimal::ZERO)
    }

    fn to_response(p: PesagemSemanal) -> PesagemSemanalResponseDto {
        PesagemSemanalResponseDto {
            id: p.id,
            lote_id: p.lote_id,
            data_pesagem: p.data_pesagem,
            idade_dias: p.idade_dias,
            semana_vida: p.semana_vida,
            peso_medio_gramas: p.peso_medio_gramas,
            quantidade_amostrada: p.quantidade_amostrada,
            peso_minimo: p.peso_minimo,
            peso_maximo: p.peso_maximo,
            desvio_padrao: p.desvio_padrao,
            coeficiente_variacao: p.coeficiente_variacao,
            ganho_semanal: p.ganho_semanal,
            ganho_medio_diario: Self::ganho_medio_diario(p.ganho_semanal),
            observacoes: p.observacoes,
            data_criacao: p.data_criacao,
        }
    }

    /// POST /api/pesagem -- registra pesagem semanal
    pub async fn create(
        pool: &PgPool,
        dto: &CreatePesagemSemanalDto,
        user_id: i32,
        user_email: &str,
    ) -> Result<PesagemSemanalResponseDto, AppError> {
        let now = chrono::Utc::now();

        let row = sqlx::query_as::<_, PesagemSemanal>(
            r#"INSERT INTO "PesagensSemanais" ("LoteId", "DataPesagem", "IdadeDias", "SemanaVida",
                 "PesoMedioGramas", "QuantidadeAmostrada", "PesoMinimo", "PesoMaximo",
                 "DesvioPadrao", "CoeficienteVariacao", "GanhoSemanal", "Observacoes", "DataCriacao")
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
               RETURNING "Id", "LoteId", "DataPesagem", "IdadeDias", "SemanaVida",
                 "PesoMedioGramas", "QuantidadeAmostrada", "PesoMinimo", "PesoMaximo",
                 "DesvioPadrao", "CoeficienteVariacao", "GanhoSemanal", "Observacoes", "DataCriacao""#,
        )
        .bind(dto.lote_id)
        .bind(dto.data_pesagem)
        .bind(dto.idade_dias)
        .bind(dto.semana_vida)
        .bind(dto.peso_medio_gramas)
        .bind(dto.quantidade_amostrada)
        .bind(dto.peso_minimo)
        .bind(dto.peso_maximo)
        .bind(dto.desvio_padrao)
        .bind(dto.coeficiente_variacao)
        .bind(dto.ganho_semanal)
        .bind(&dto.observacoes)
        .bind(now)
        .fetch_one(pool)
        .await?;

        AuditoriaService::registrar_log(
            pool,
            user_id,
            user_email,
            "REGISTRO_PESAGEM",
            &format!(
                "Pesagem registrada para lote {} (semana {}, peso medio {} g).",
                dto.lote_id, dto.semana_vida, dto.peso_medio_gramas
            ),
        )
        .await?;

        Ok(Self::to_response(row))
    }

    /// GET /api/pesagem/{loteId} -- lista pesagens do lote
    pub async fn list(
        pool: &PgPool,
        lote_id: i32,
    ) -> Result<Vec<PesagemSemanalResponseDto>, AppError> {
        let rows = sqlx::query_as::<_, PesagemSemanal>(
            r#"SELECT "Id", "LoteId", "DataPesagem", "IdadeDias", "SemanaVida",
                 "PesoMedioGramas", "QuantidadeAmostrada", "PesoMinimo", "PesoMaximo",
                 "DesvioPadrao", "CoeficienteVariacao", "GanhoSemanal", "Observacoes", "DataCriacao"
               FROM "PesagensSemanais"
               WHERE "LoteId" = $1
               ORDER BY "SemanaVida" ASC"#,
        )
        .bind(lote_id)
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(Self::to_response).collect())
    }

    /// GET /api/pesagem/resumo/{loteId} -- resumo de crescimento do lote
    pub async fn resumo(
        pool: &PgPool,
        lote_id: i32,
    ) -> Result<serde_json::Value, AppError> {
        // Latest pesagem
        let latest = sqlx::query_as::<_, PesagemSemanal>(
            r#"SELECT "Id", "LoteId", "DataPesagem", "IdadeDias", "SemanaVida",
                 "PesoMedioGramas", "QuantidadeAmostrada", "PesoMinimo", "PesoMaximo",
                 "DesvioPadrao", "CoeficienteVariacao", "GanhoSemanal", "Observacoes", "DataCriacao"
               FROM "PesagensSemanais"
               WHERE "LoteId" = $1
               ORDER BY "SemanaVida" DESC
               LIMIT 1"#,
        )
        .bind(lote_id)
        .fetch_optional(pool)
        .await?;

        // Earliest pesagem
        let earliest = sqlx::query_as::<_, PesagemSemanal>(
            r#"SELECT "Id", "LoteId", "DataPesagem", "IdadeDias", "SemanaVida",
                 "PesoMedioGramas", "QuantidadeAmostrada", "PesoMinimo", "PesoMaximo",
                 "DesvioPadrao", "CoeficienteVariacao", "GanhoSemanal", "Observacoes", "DataCriacao"
               FROM "PesagensSemanais"
               WHERE "LoteId" = $1
               ORDER BY "SemanaVida" ASC
               LIMIT 1"#,
        )
        .bind(lote_id)
        .fetch_optional(pool)
        .await?;

        // Count total
        let total: i64 = sqlx::query_scalar(
            r#"SELECT COUNT(*) FROM "PesagensSemanais" WHERE "LoteId" = $1"#,
        )
        .bind(lote_id)
        .fetch_one(pool)
        .await
        .unwrap_or(0);

        let peso_atual = latest.as_ref().map(|p| p.peso_medio_gramas).unwrap_or(Decimal::ZERO);
        let peso_inicial = earliest.as_ref().map(|p| p.peso_medio_gramas).unwrap_or(Decimal::ZERO);
        let ganho_total = peso_atual - peso_inicial;
        let uniformidade = latest.as_ref().and_then(|p| p.coeficiente_variacao);

        Ok(serde_json::json!({
            "pesoAtual": peso_atual,
            "pesoInicial": peso_inicial,
            "ganhoTotal": ganho_total,
            "uniformidade": uniformidade,
            "totalPesagens": total,
        }))
    }
}
