use chrono::Utc;
use rust_decimal::Decimal;
use sqlx::PgPool;

use crate::dto::lote::*;
use crate::dto::sanitario::{CreateRegistroMortalidadeDto, RegistroMortalidadeDto};
use crate::errors::AppError;
use crate::models::lote::Lote;
use crate::models::registro_mortalidade::RegistroMortalidade;
use crate::services::auditoria_service::AuditoriaService;

pub struct LoteService;

/// Mapeia Lote do banco para LoteResponseDto com propriedades computadas
fn map_lote_to_response(lote: &Lote) -> LoteResponseDto {
    let idade_atual_dias = (Utc::now().date_naive() - lote.data_entrada.date_naive()).num_days() as i32;

    let inicial = lote.quantidade_aves_inicial;
    let atual = lote.quantidade_aves_atual;
    let mortalidade_acumulada = inicial - atual;

    let pct_mortalidade = if inicial > 0 {
        Decimal::from(mortalidade_acumulada) / Decimal::from(inicial) * Decimal::from(100)
    } else {
        Decimal::ZERO
    };

    let viabilidade = Decimal::from(100) - pct_mortalidade;

    let densidade_atual = match lote.area_galpao {
        Some(area) if area > Decimal::ZERO => Decimal::from(atual) / area,
        _ => Decimal::ZERO,
    };

    LoteResponseDto {
        id: lote.id,
        codigo: lote.codigo.clone(),
        identificador: lote.identificador.clone(),
        data_entrada: lote.data_entrada,
        data_abate_prevista: lote.data_abate_prevista,
        data_saida: lote.data_saida,
        quantidade_aves_inicial: inicial,
        quantidade_aves_atual: atual,
        area_galpao: lote.area_galpao,
        linhagem: lote.linhagem.clone(),
        origem_pintinhos: lote.origem_pintinhos.clone(),
        status: lote.status.clone(),
        observacoes: lote.observacoes.clone(),
        granja_id: lote.granja_id,
        data_criacao: lote.data_criacao,
        data_atualizacao: lote.data_atualizacao,
        idade_atual_dias,
        viabilidade,
        densidade_atual,
    }
}

impl LoteService {
    /// GET /api/lotes -- lista lotes com filtro por role
    pub async fn get_all(
        pool: &PgPool,
        user_id: i32,
        user_role: &str,
    ) -> Result<Vec<LoteResponseDto>, AppError> {
        let lotes = match user_role {
            "Administrador" => {
                sqlx::query_as::<_, Lote>(
                    r#"SELECT "Id", "Codigo", "Identificador", "DataEntrada",
                              "DataAbatePrevista", "DataSaida",
                              "QuantidadeAvesInicial", "QuantidadeAvesAtual",
                              "AreaGalpao", "Linhagem", "OrigemPintinhos",
                              "Status", "Observacoes", "GranjaId",
                              "DataCriacao", "DataAtualizacao"
                       FROM "Lotes"
                       ORDER BY "Id""#,
                )
                .fetch_all(pool)
                .await?
            }
            "Produtor" => {
                sqlx::query_as::<_, Lote>(
                    r#"SELECT l."Id", l."Codigo", l."Identificador", l."DataEntrada",
                              l."DataAbatePrevista", l."DataSaida",
                              l."QuantidadeAvesInicial", l."QuantidadeAvesAtual",
                              l."AreaGalpao", l."Linhagem", l."OrigemPintinhos",
                              l."Status", l."Observacoes", l."GranjaId",
                              l."DataCriacao", l."DataAtualizacao"
                       FROM "Lotes" l
                       INNER JOIN "Granjas" g ON g."Id" = l."GranjaId"
                       WHERE g."UsuarioId" = $1
                       ORDER BY l."Id""#,
                )
                .bind(user_id)
                .fetch_all(pool)
                .await?
            }
            "Financeiro" => {
                sqlx::query_as::<_, Lote>(
                    r#"SELECT l."Id", l."Codigo", l."Identificador", l."DataEntrada",
                              l."DataAbatePrevista", l."DataSaida",
                              l."QuantidadeAvesInicial", l."QuantidadeAvesAtual",
                              l."AreaGalpao", l."Linhagem", l."OrigemPintinhos",
                              l."Status", l."Observacoes", l."GranjaId",
                              l."DataCriacao", l."DataAtualizacao"
                       FROM "Lotes" l
                       INNER JOIN "Granjas" g ON g."Id" = l."GranjaId"
                       WHERE g."UsuarioId" IN (
                           SELECT "ProdutorId" FROM "FinanceiroProdutor"
                           WHERE "FinanceiroId" = $1
                       )
                       ORDER BY l."Id""#,
                )
                .bind(user_id)
                .fetch_all(pool)
                .await?
            }
            _ => Vec::new(),
        };

        Ok(lotes.iter().map(map_lote_to_response).collect())
    }

    /// GET /api/lotes/{id} -- busca lote por ID com verificacao de acesso
    pub async fn get_by_id(
        pool: &PgPool,
        id: i32,
        user_id: i32,
        user_role: &str,
    ) -> Result<LoteResponseDto, AppError> {
        let lote = sqlx::query_as::<_, Lote>(
            r#"SELECT "Id", "Codigo", "Identificador", "DataEntrada",
                      "DataAbatePrevista", "DataSaida",
                      "QuantidadeAvesInicial", "QuantidadeAvesAtual",
                      "AreaGalpao", "Linhagem", "OrigemPintinhos",
                      "Status", "Observacoes", "GranjaId",
                      "DataCriacao", "DataAtualizacao"
               FROM "Lotes"
               WHERE "Id" = $1"#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Lote com ID {} nao encontrado", id)))?;

        // Verificar acesso via granja ownership
        Self::verificar_acesso_granja(pool, lote.granja_id, user_id, user_role).await?;

        Ok(map_lote_to_response(&lote))
    }

    /// POST /api/lotes -- cria novo lote (Financeiro bloqueado)
    pub async fn create(
        pool: &PgPool,
        dto: &CreateLoteDto,
        user_id: i32,
        user_role: &str,
        user_email: &str,
    ) -> Result<LoteResponseDto, AppError> {
        if user_role == "Financeiro" {
            return Err(AppError::Forbidden(
                "Usuarios do perfil Financeiro nao podem criar lotes.".into(),
            ));
        }

        // Verificar acesso a granja
        Self::verificar_acesso_granja(pool, dto.granja_id, user_id, user_role).await?;

        // Gerar codigo sequencial (LT-001, LT-002...)
        let ultimo_id: Option<i32> =
            sqlx::query_scalar(r#"SELECT MAX("Id") FROM "Lotes""#)
                .fetch_one(pool)
                .await
                .map_err(|e| AppError::Internal(e.to_string()))?;

        let novo_codigo = format!("LT-{:03}", ultimo_id.unwrap_or(0) + 1);
        let agora = Utc::now();

        let lote = sqlx::query_as::<_, Lote>(
            r#"INSERT INTO "Lotes" ("Codigo", "Identificador", "QuantidadeAvesInicial",
                                     "QuantidadeAvesAtual", "DataEntrada", "DataSaida",
                                     "GranjaId", "Status", "DataCriacao")
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
               RETURNING "Id", "Codigo", "Identificador", "DataEntrada",
                         "DataAbatePrevista", "DataSaida",
                         "QuantidadeAvesInicial", "QuantidadeAvesAtual",
                         "AreaGalpao", "Linhagem", "OrigemPintinhos",
                         "Status", "Observacoes", "GranjaId",
                         "DataCriacao", "DataAtualizacao""#,
        )
        .bind(&novo_codigo)
        .bind(&dto.identificador)
        .bind(dto.quantidade_aves_inicial)
        .bind(dto.quantidade_aves_inicial) // QuantidadeAvesAtual = Inicial
        .bind(dto.data_entrada)
        .bind(dto.data_saida)
        .bind(dto.granja_id)
        .bind("Ativo")
        .bind(agora)
        .fetch_one(pool)
        .await?;

        AuditoriaService::registrar_log(
            pool,
            user_id,
            user_email,
            "CRIACAO_LOTE",
            &format!(
                "Lote '{}' criado na granja {}",
                novo_codigo, dto.granja_id
            ),
        )
        .await?;

        Ok(map_lote_to_response(&lote))
    }

    /// PUT /api/lotes/{id} -- atualiza lote (Financeiro bloqueado)
    pub async fn update(
        pool: &PgPool,
        id: i32,
        dto: &UpdateLoteDto,
        user_id: i32,
        user_role: &str,
        user_email: &str,
    ) -> Result<LoteResponseDto, AppError> {
        if user_role == "Financeiro" {
            return Err(AppError::Forbidden(
                "Usuarios do perfil Financeiro nao podem editar lotes.".into(),
            ));
        }

        // Buscar lote existente
        let existente = sqlx::query_as::<_, Lote>(
            r#"SELECT "Id", "Codigo", "Identificador", "DataEntrada",
                      "DataAbatePrevista", "DataSaida",
                      "QuantidadeAvesInicial", "QuantidadeAvesAtual",
                      "AreaGalpao", "Linhagem", "OrigemPintinhos",
                      "Status", "Observacoes", "GranjaId",
                      "DataCriacao", "DataAtualizacao"
               FROM "Lotes"
               WHERE "Id" = $1"#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Lote com ID {} nao encontrado", id)))?;

        Self::verificar_acesso_granja(pool, existente.granja_id, user_id, user_role).await?;

        // Delta logic para QuantidadeAvesInicial
        let delta = dto.quantidade_aves_inicial - existente.quantidade_aves_inicial;
        let new_atual = {
            let v = existente.quantidade_aves_atual + delta;
            if v < 0 { 0 } else { v }
        };

        let agora = Utc::now();

        let lote = sqlx::query_as::<_, Lote>(
            r#"UPDATE "Lotes"
               SET "Identificador" = $1, "QuantidadeAvesInicial" = $2,
                   "QuantidadeAvesAtual" = $3, "DataEntrada" = $4,
                   "DataSaida" = $5, "GranjaId" = $6, "DataAtualizacao" = $7
               WHERE "Id" = $8
               RETURNING "Id", "Codigo", "Identificador", "DataEntrada",
                         "DataAbatePrevista", "DataSaida",
                         "QuantidadeAvesInicial", "QuantidadeAvesAtual",
                         "AreaGalpao", "Linhagem", "OrigemPintinhos",
                         "Status", "Observacoes", "GranjaId",
                         "DataCriacao", "DataAtualizacao""#,
        )
        .bind(&dto.identificador)
        .bind(dto.quantidade_aves_inicial)
        .bind(new_atual)
        .bind(dto.data_entrada)
        .bind(dto.data_saida)
        .bind(dto.granja_id)
        .bind(agora)
        .bind(id)
        .fetch_one(pool)
        .await?;

        AuditoriaService::registrar_log(
            pool,
            user_id,
            user_email,
            "ATUALIZACAO_LOTE",
            &format!("Lote '{}' (ID: {}) atualizado.", lote.identificador, id),
        )
        .await?;

        Ok(map_lote_to_response(&lote))
    }

    /// DELETE /api/lotes/{id} -- deleta lote (Financeiro bloqueado)
    pub async fn delete(
        pool: &PgPool,
        id: i32,
        user_id: i32,
        user_role: &str,
        user_email: &str,
    ) -> Result<(), AppError> {
        if user_role == "Financeiro" {
            return Err(AppError::Forbidden(
                "Usuarios do perfil Financeiro nao podem deletar lotes.".into(),
            ));
        }

        let lote = sqlx::query_as::<_, Lote>(
            r#"SELECT "Id", "Codigo", "Identificador", "DataEntrada",
                      "DataAbatePrevista", "DataSaida",
                      "QuantidadeAvesInicial", "QuantidadeAvesAtual",
                      "AreaGalpao", "Linhagem", "OrigemPintinhos",
                      "Status", "Observacoes", "GranjaId",
                      "DataCriacao", "DataAtualizacao"
               FROM "Lotes"
               WHERE "Id" = $1"#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Lote com ID {} nao encontrado", id)))?;

        Self::verificar_acesso_granja(pool, lote.granja_id, user_id, user_role).await?;

        sqlx::query(r#"DELETE FROM "Lotes" WHERE "Id" = $1"#)
            .bind(id)
            .execute(pool)
            .await?;

        AuditoriaService::registrar_log(
            pool,
            user_id,
            user_email,
            "DELECAO_LOTE",
            &format!("Lote '{}' (ID: {}) deletado.", lote.identificador, id),
        )
        .await?;

        Ok(())
    }

    /// POST /api/lotes/{id}/mortalidades -- registra mortalidade
    pub async fn registrar_mortalidade(
        pool: &PgPool,
        lote_id: i32,
        dto: &CreateRegistroMortalidadeDto,
        user_id: i32,
        user_role: &str,
        user_email: &str,
    ) -> Result<RegistroMortalidadeDto, AppError> {
        if user_role == "Financeiro" {
            return Err(AppError::Forbidden(
                "Usuarios do perfil Financeiro nao podem registrar mortalidade.".into(),
            ));
        }

        // Buscar lote e verificar acesso
        let lote = sqlx::query_as::<_, Lote>(
            r#"SELECT "Id", "Codigo", "Identificador", "DataEntrada",
                      "DataAbatePrevista", "DataSaida",
                      "QuantidadeAvesInicial", "QuantidadeAvesAtual",
                      "AreaGalpao", "Linhagem", "OrigemPintinhos",
                      "Status", "Observacoes", "GranjaId",
                      "DataCriacao", "DataAtualizacao"
               FROM "Lotes"
               WHERE "Id" = $1"#,
        )
        .bind(lote_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Lote com ID {} nao encontrado", lote_id)))?;

        Self::verificar_acesso_granja(pool, lote.granja_id, user_id, user_role).await?;

        // Clamp quantity para nao ficar negativo
        let qtd = dto.quantidade_mortas.min(lote.quantidade_aves_atual);

        // Baixa no lote
        sqlx::query(
            r#"UPDATE "Lotes" SET "QuantidadeAvesAtual" = "QuantidadeAvesAtual" - $1,
                                   "DataAtualizacao" = $2
               WHERE "Id" = $3"#,
        )
        .bind(qtd)
        .bind(Utc::now())
        .bind(lote_id)
        .execute(pool)
        .await?;

        // Computar idade em dias
        let idade_dias =
            (dto.data.date_naive() - lote.data_entrada.date_naive()).num_days() as i32;
        let aves_vivas = lote.quantidade_aves_atual - qtd;

        // Inserir registro de mortalidade
        let agora = Utc::now();
        let mortalidade = sqlx::query_as::<_, RegistroMortalidade>(
            r#"INSERT INTO "RegistrosMortalidade" ("LoteId", "Data", "QuantidadeMortas", "AvesVivas",
                                                    "CausaPrincipal", "IdadeDias", "PesoMedioMortas",
                                                    "Observacoes", "AcaoTomada", "ResponsavelRegistro",
                                                    "DataCriacao")
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
               RETURNING "Id", "LoteId", "Data", "QuantidadeMortas", "AvesVivas",
                         "CausaPrincipal", "IdadeDias", "PesoMedioMortas",
                         "Observacoes", "AcaoTomada", "ResponsavelRegistro", "DataCriacao""#,
        )
        .bind(lote_id)
        .bind(dto.data)
        .bind(qtd)
        .bind(aves_vivas)
        .bind(&dto.causa_principal)
        .bind(idade_dias)
        .bind(dto.peso_medio_mortas)
        .bind(&dto.observacoes)
        .bind(&dto.acao_tomada)
        .bind(user_email)
        .bind(agora)
        .fetch_one(pool)
        .await?;

        // Computar percentual de mortalidade do dia
        let percentual_mortalidade_dia = if lote.quantidade_aves_inicial > 0 {
            Decimal::from(qtd) / Decimal::from(lote.quantidade_aves_inicial) * Decimal::from(100)
        } else {
            Decimal::ZERO
        };

        AuditoriaService::registrar_log(
            pool,
            user_id,
            user_email,
            "REGISTRO_MORTALIDADE",
            &format!(
                "Lote '{}' (ID {}) - {} aves registradas como mortas em {}.",
                lote.identificador,
                lote.id,
                qtd,
                dto.data.format("%Y-%m-%d")
            ),
        )
        .await?;

        Ok(RegistroMortalidadeDto {
            id: mortalidade.id,
            lote_id: mortalidade.lote_id,
            data: mortalidade.data,
            quantidade_mortas: mortalidade.quantidade_mortas,
            aves_vivas: mortalidade.aves_vivas,
            causa_principal: mortalidade.causa_principal,
            idade_dias: mortalidade.idade_dias,
            peso_medio_mortas: mortalidade.peso_medio_mortas,
            observacoes: mortalidade.observacoes,
            acao_tomada: mortalidade.acao_tomada,
            responsavel_registro: mortalidade.responsavel_registro,
            percentual_mortalidade_dia,
        })
    }

    /// GET /api/lotes/{id}/mortalidades -- lista mortalidades de um lote
    pub async fn listar_mortalidades(
        pool: &PgPool,
        lote_id: i32,
        user_id: i32,
        user_role: &str,
    ) -> Result<Vec<RegistroMortalidadeDto>, AppError> {
        // Buscar lote para verificar acesso e obter data_entrada/quantidade_inicial
        let lote = sqlx::query_as::<_, Lote>(
            r#"SELECT "Id", "Codigo", "Identificador", "DataEntrada",
                      "DataAbatePrevista", "DataSaida",
                      "QuantidadeAvesInicial", "QuantidadeAvesAtual",
                      "AreaGalpao", "Linhagem", "OrigemPintinhos",
                      "Status", "Observacoes", "GranjaId",
                      "DataCriacao", "DataAtualizacao"
               FROM "Lotes"
               WHERE "Id" = $1"#,
        )
        .bind(lote_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Lote com ID {} nao encontrado", lote_id)))?;

        Self::verificar_acesso_granja(pool, lote.granja_id, user_id, user_role).await?;

        let registros = sqlx::query_as::<_, RegistroMortalidade>(
            r#"SELECT "Id", "LoteId", "Data", "QuantidadeMortas", "AvesVivas",
                      "CausaPrincipal", "IdadeDias", "PesoMedioMortas",
                      "Observacoes", "AcaoTomada", "ResponsavelRegistro", "DataCriacao"
               FROM "RegistrosMortalidade"
               WHERE "LoteId" = $1
               ORDER BY "Data" DESC"#,
        )
        .bind(lote_id)
        .fetch_all(pool)
        .await?;

        let resultado: Vec<RegistroMortalidadeDto> = registros
            .iter()
            .map(|r| {
                let percentual_mortalidade_dia = if lote.quantidade_aves_inicial > 0 {
                    Decimal::from(r.quantidade_mortas)
                        / Decimal::from(lote.quantidade_aves_inicial)
                        * Decimal::from(100)
                } else {
                    Decimal::ZERO
                };

                RegistroMortalidadeDto {
                    id: r.id,
                    lote_id: r.lote_id,
                    data: r.data,
                    quantidade_mortas: r.quantidade_mortas,
                    aves_vivas: r.aves_vivas,
                    causa_principal: r.causa_principal.clone(),
                    idade_dias: r.idade_dias,
                    peso_medio_mortas: r.peso_medio_mortas,
                    observacoes: r.observacoes.clone(),
                    acao_tomada: r.acao_tomada.clone(),
                    responsavel_registro: r.responsavel_registro.clone(),
                    percentual_mortalidade_dia,
                }
            })
            .collect();

        Ok(resultado)
    }

    // ===================== HELPERS =====================

    /// Verifica se usuario tem acesso a granja especificada
    async fn verificar_acesso_granja(
        pool: &PgPool,
        granja_id: i32,
        user_id: i32,
        user_role: &str,
    ) -> Result<(), AppError> {
        // Buscar UsuarioId da granja
        let granja_owner: Option<i32> = sqlx::query_scalar(
            r#"SELECT "UsuarioId" FROM "Granjas" WHERE "Id" = $1"#,
        )
        .bind(granja_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

        let owner_id = granja_owner
            .ok_or_else(|| AppError::NotFound(format!("Granja com ID {} nao encontrada", granja_id)))?;

        match user_role {
            "Administrador" => Ok(()),
            "Produtor" => {
                if owner_id == user_id {
                    Ok(())
                } else {
                    Err(AppError::Forbidden("Acesso negado a esta granja".into()))
                }
            }
            "Financeiro" => {
                let tem_acesso: bool = sqlx::query_scalar(
                    r#"SELECT EXISTS(
                        SELECT 1 FROM "FinanceiroProdutor"
                        WHERE "FinanceiroId" = $1 AND "ProdutorId" = $2
                    )"#,
                )
                .bind(user_id)
                .bind(owner_id)
                .fetch_one(pool)
                .await
                .map_err(|e| AppError::Internal(e.to_string()))?;

                if tem_acesso {
                    Ok(())
                } else {
                    Err(AppError::Forbidden("Acesso negado a esta granja".into()))
                }
            }
            _ => Err(AppError::Forbidden("Perfil nao reconhecido".into())),
        }
    }
}
