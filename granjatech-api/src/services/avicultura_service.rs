use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use sqlx::PgPool;

use crate::dto::avicultura::*;
use crate::errors::AppError;
use crate::models::lote::Lote;

// Padroes da industria avicola brasileira (matching .NET PadroesIndustria dictionary)
const CONVERSAO_ALIMENTAR_PADRAO: Decimal = dec!(1.75);
const CONVERSAO_ALIMENTAR_EXCELENCIA: Decimal = dec!(1.60);
const GMD_PADRAO: Decimal = dec!(55);
const GMD_EXCELENCIA: Decimal = dec!(60);
const VIABILIDADE_PADRAO: Decimal = dec!(95);
const VIABILIDADE_EXCELENCIA: Decimal = dec!(97);
const IEP_PADRAO: Decimal = dec!(350);
const IEP_EXCELENCIA: Decimal = dec!(400);
const MORTALIDADE_MAXIMA: Decimal = dec!(5);
const DENSIDADE_MAXIMA: Decimal = dec!(18);
const NH3_MAXIMO: Decimal = dec!(25);

/// Helper struct para query de pesagem
#[derive(Debug, sqlx::FromRow)]
struct PesagemRow {
    #[sqlx(rename = "PesoMedioGramas")]
    peso_medio_gramas: Decimal,
    #[sqlx(rename = "SemanaVida")]
    semana_vida: i32,
    #[sqlx(rename = "DataPesagem")]
    data_pesagem: DateTime<Utc>,
}

/// Helper struct para medicao de qualidade do ar (subset de colunas)
#[derive(Debug, sqlx::FromRow)]
struct MedicaoArRow {
    #[sqlx(rename = "NH3_ppm")]
    nh3_ppm: Option<Decimal>,
    #[sqlx(rename = "TemperaturaAr")]
    temperatura_ar: Option<Decimal>,
    #[sqlx(rename = "DataHora")]
    data_hora: DateTime<Utc>,
}

pub struct AviculturaService;

impl AviculturaService {
    // ========================================================================
    // Metodos com implementacao real (matching .NET AviculturaService)
    // ========================================================================

    /// Calcula o Indice de Eficiencia Produtiva (IEP) de um lote.
    /// Mirrors .NET CalcularIEPAsync.
    pub async fn calcular_iep(pool: &PgPool, lote_id: i32) -> Result<Decimal, AppError> {
        let lote = sqlx::query_as::<_, Lote>(
            r#"SELECT * FROM "Lotes" WHERE "Id" = $1"#,
        )
        .bind(lote_id)
        .fetch_optional(pool)
        .await?;

        let lote = match lote {
            Some(l) => l,
            None => return Ok(Decimal::ZERO),
        };

        // Pesagem mais recente
        let peso_medio: Option<Decimal> = sqlx::query_scalar(
            r#"SELECT "PesoMedioGramas" FROM "PesagensSemanais" WHERE "LoteId" = $1 ORDER BY "DataPesagem" DESC LIMIT 1"#,
        )
        .bind(lote_id)
        .fetch_optional(pool)
        .await?;

        let peso_medio = match peso_medio {
            Some(p) => p,
            None => return Ok(Decimal::ZERO),
        };

        let idade_dias = (Utc::now().date_naive() - lote.data_entrada.date_naive()).num_days();
        if idade_dias == 0 {
            return Ok(Decimal::ZERO);
        }

        let ganho_peso_kg = (peso_medio - dec!(45)) / dec!(1000); // 45g peso inicial
        let viabilidade = if lote.quantidade_aves_inicial > 0 {
            Decimal::from(lote.quantidade_aves_atual) * dec!(100)
                / Decimal::from(lote.quantidade_aves_inicial)
        } else {
            Decimal::ZERO
        };

        let ca = Self::calcular_conversao_alimentar(pool, lote_id).await?;
        if ca == Decimal::ZERO {
            return Ok(Decimal::ZERO);
        }

        let idade = Decimal::from(idade_dias);
        let iep = (ganho_peso_kg * viabilidade * dec!(100)) / (ca * idade);

        Ok(iep.round_dp(2))
    }

    /// Calcula a Conversao Alimentar (CA) de um lote.
    /// Mirrors .NET CalcularConversaoAlimentarAsync.
    pub async fn calcular_conversao_alimentar(
        pool: &PgPool,
        lote_id: i32,
    ) -> Result<Decimal, AppError> {
        let lote = sqlx::query_as::<_, Lote>(
            r#"SELECT * FROM "Lotes" WHERE "Id" = $1"#,
        )
        .bind(lote_id)
        .fetch_optional(pool)
        .await?;

        let lote = match lote {
            Some(l) => l,
            None => return Ok(Decimal::ZERO),
        };

        // Total de racao consumida
        let total_racao: Decimal = sqlx::query_scalar(
            r#"SELECT COALESCE(SUM("QuantidadeKg"), 0) FROM "ConsumosRacao" WHERE "LoteId" = $1"#,
        )
        .bind(lote_id)
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

        // Pesagem mais recente
        let peso_medio: Option<Decimal> = sqlx::query_scalar(
            r#"SELECT "PesoMedioGramas" FROM "PesagensSemanais" WHERE "LoteId" = $1 ORDER BY "DataPesagem" DESC LIMIT 1"#,
        )
        .bind(lote_id)
        .fetch_optional(pool)
        .await?;

        let peso_medio = match peso_medio {
            Some(p) => p,
            None => return Ok(Decimal::ZERO),
        };

        if total_racao == Decimal::ZERO {
            return Ok(Decimal::ZERO);
        }

        let ganho_total_por_ave_gramas = peso_medio - dec!(45); // 45g peso inicial
        let ganho_total_lote_kg =
            (ganho_total_por_ave_gramas * Decimal::from(lote.quantidade_aves_atual)) / dec!(1000);

        let ca = if ganho_total_lote_kg > Decimal::ZERO {
            total_racao / ganho_total_lote_kg
        } else {
            Decimal::ZERO
        };

        Ok(ca.round_dp(3))
    }

    /// Calcula o Ganho Medio Diario (GMD) de um lote.
    /// Mirrors .NET CalcularGanhoMedioDiarioAsync.
    pub async fn calcular_gmd(pool: &PgPool, lote_id: i32) -> Result<Decimal, AppError> {
        let pesagens = sqlx::query_as::<_, PesagemRow>(
            r#"SELECT "PesoMedioGramas", "SemanaVida", "DataPesagem" FROM "PesagensSemanais" WHERE "LoteId" = $1 ORDER BY "SemanaVida" ASC"#,
        )
        .bind(lote_id)
        .fetch_all(pool)
        .await?;

        if pesagens.len() < 2 {
            return Ok(Decimal::ZERO);
        }

        let mut gmd_total = Decimal::ZERO;
        let mut semanas_com_ganho = 0i32;

        for i in 1..pesagens.len() {
            let ganho_semanal = pesagens[i].peso_medio_gramas - pesagens[i - 1].peso_medio_gramas;
            let gmd_semanal = ganho_semanal / dec!(7);

            gmd_total += gmd_semanal;
            semanas_com_ganho += 1;
        }

        let gmd_medio = if semanas_com_ganho > 0 {
            gmd_total / Decimal::from(semanas_com_ganho)
        } else {
            Decimal::ZERO
        };

        Ok(gmd_medio.round_dp(2))
    }

    /// Verifica alertas de parametros fora do padrao para um lote.
    /// Mirrors .NET VerificarParametrosForaPadraoAsync.
    pub async fn verificar_alertas(
        pool: &PgPool,
        lote_id: i32,
    ) -> Result<Vec<AlertaParametroDto>, AppError> {
        let mut alertas = Vec::new();

        let lote = sqlx::query_as::<_, Lote>(
            r#"SELECT * FROM "Lotes" WHERE "Id" = $1"#,
        )
        .bind(lote_id)
        .fetch_optional(pool)
        .await?;

        let lote = match lote {
            Some(l) => l,
            None => return Ok(alertas),
        };

        // Verificar mortalidade
        let percentual_mortalidade = if lote.quantidade_aves_inicial > 0 {
            (Decimal::from(lote.quantidade_aves_inicial - lote.quantidade_aves_atual) * dec!(100))
                / Decimal::from(lote.quantidade_aves_inicial)
        } else {
            Decimal::ZERO
        };

        if percentual_mortalidade > MORTALIDADE_MAXIMA {
            alertas.push(AlertaParametroDto {
                tipo_alerta: "Mortalidade".into(),
                severidade: if percentual_mortalidade > dec!(8) {
                    "Critica".into()
                } else {
                    "Alta".into()
                },
                descricao: "Mortalidade acumulada acima do padrao da industria".into(),
                valor_atual: percentual_mortalidade,
                valor_minimo: None,
                valor_maximo: Some(MORTALIDADE_MAXIMA),
                unidade: "%".into(),
                data_ocorrencia: Utc::now(),
                recomendacao: Some(
                    "Investigar causas e implementar medidas sanitarias".into(),
                ),
                ativo: true,
            });
        }

        // Verificar densidade
        if let Some(area) = lote.area_galpao {
            if area > Decimal::ZERO {
                let densidade = Decimal::from(lote.quantidade_aves_atual) / area;
                if densidade > DENSIDADE_MAXIMA {
                    alertas.push(AlertaParametroDto {
                        tipo_alerta: "Densidade".into(),
                        severidade: "Media".into(),
                        descricao: "Densidade de aves acima do recomendado".into(),
                        valor_atual: densidade,
                        valor_minimo: None,
                        valor_maximo: Some(DENSIDADE_MAXIMA),
                        unidade: "aves/m2".into(),
                        data_ocorrencia: Utc::now(),
                        recomendacao: Some(
                            "Considerar ajustar numero de aves ou area disponivel".into(),
                        ),
                        ativo: true,
                    });
                }
            }
        }

        // Verificar qualidade do ar nas ultimas 24h
        let limite_24h = Utc::now() - chrono::Duration::days(1);
        let medicoes = sqlx::query_as::<_, MedicaoArRow>(
            r#"SELECT "NH3_ppm", "TemperaturaAr", "DataHora" FROM "MedicoesQualidadeAr" WHERE "LoteId" = $1 AND "DataHora" >= $2 ORDER BY "DataHora" DESC LIMIT 5"#,
        )
        .bind(lote_id)
        .bind(limite_24h)
        .fetch_all(pool)
        .await?;

        for medicao in &medicoes {
            if let Some(nh3) = medicao.nh3_ppm {
                if nh3 > NH3_MAXIMO {
                    alertas.push(AlertaParametroDto {
                        tipo_alerta: "Amonia".into(),
                        severidade: if nh3 > dec!(35) {
                            "Critica".into()
                        } else {
                            "Alta".into()
                        },
                        descricao: "Nivel de amonia (NH3) acima do limite".into(),
                        valor_atual: nh3,
                        valor_minimo: None,
                        valor_maximo: Some(NH3_MAXIMO),
                        unidade: "ppm".into(),
                        data_ocorrencia: medicao.data_hora,
                        recomendacao: Some(
                            "Aumentar ventilacao e verificar sistema de exaustao".into(),
                        ),
                        ativo: true,
                    });
                }
            }

            if let Some(temp) = medicao.temperatura_ar {
                if temp < dec!(18) || temp > dec!(33) {
                    alertas.push(AlertaParametroDto {
                        tipo_alerta: "Temperatura".into(),
                        severidade: if temp < dec!(15) || temp > dec!(35) {
                            "Critica".into()
                        } else {
                            "Media".into()
                        },
                        descricao: "Temperatura fora da faixa ideal".into(),
                        valor_atual: temp,
                        valor_minimo: Some(dec!(18)),
                        valor_maximo: Some(dec!(33)),
                        unidade: "C".into(),
                        data_ocorrencia: medicao.data_hora,
                        recomendacao: Some(
                            "Ajustar sistema de aquecimento/resfriamento".into(),
                        ),
                        ativo: true,
                    });
                }
            }
        }

        Ok(alertas)
    }

    /// Compara metricas do lote com padroes da industria.
    /// Mirrors .NET CompararComPadroesIndustriaAsync.
    pub async fn comparar_com_industria(
        pool: &PgPool,
        lote_id: i32,
    ) -> Result<ComparacaoIndustriaDto, AppError> {
        let lote = sqlx::query_as::<_, Lote>(
            r#"SELECT * FROM "Lotes" WHERE "Id" = $1"#,
        )
        .bind(lote_id)
        .fetch_optional(pool)
        .await?;

        let lote = match lote {
            Some(l) => l,
            None => return Ok(ComparacaoIndustriaDto::default()),
        };

        let idade_dias = (Utc::now().date_naive() - lote.data_entrada.date_naive()).num_days() as i32;

        let viabilidade_valor = if lote.quantidade_aves_inicial > 0 {
            Decimal::from(lote.quantidade_aves_atual) * dec!(100)
                / Decimal::from(lote.quantidade_aves_inicial)
        } else {
            Decimal::ZERO
        };

        let ca = Self::calcular_conversao_alimentar(pool, lote_id).await?;
        let iep = Self::calcular_iep(pool, lote_id).await?;
        let gmd = Self::calcular_gmd(pool, lote_id).await?;

        // Conversao Alimentar (lower is better)
        let ca_metrica = MetricaComparacaoDto {
            nome: "Conversao Alimentar".into(),
            valor_atual: ca,
            valor_padrao_industria: CONVERSAO_ALIMENTAR_PADRAO,
            valor_padrao_excelencia: CONVERSAO_ALIMENTAR_EXCELENCIA,
            percentual_diferenca: calcular_percentual_diferenca(ca, CONVERSAO_ALIMENTAR_PADRAO),
            status: if ca <= CONVERSAO_ALIMENTAR_EXCELENCIA {
                "Excelente".into()
            } else if ca <= CONVERSAO_ALIMENTAR_PADRAO {
                "Bom".into()
            } else {
                "Abaixo".into()
            },
            unidade: String::new(),
        };

        // GMD (higher is better)
        let gmd_metrica = MetricaComparacaoDto {
            nome: "Ganho Medio Diario".into(),
            valor_atual: gmd,
            valor_padrao_industria: GMD_PADRAO,
            valor_padrao_excelencia: GMD_EXCELENCIA,
            percentual_diferenca: calcular_percentual_diferenca(gmd, GMD_PADRAO),
            status: if gmd >= GMD_EXCELENCIA {
                "Excelente".into()
            } else if gmd >= GMD_PADRAO {
                "Bom".into()
            } else {
                "Abaixo".into()
            },
            unidade: "g/dia".into(),
        };

        // Viabilidade (higher is better)
        let viab_metrica = MetricaComparacaoDto {
            nome: "Viabilidade".into(),
            valor_atual: viabilidade_valor,
            valor_padrao_industria: VIABILIDADE_PADRAO,
            valor_padrao_excelencia: VIABILIDADE_EXCELENCIA,
            percentual_diferenca: calcular_percentual_diferenca(viabilidade_valor, VIABILIDADE_PADRAO),
            status: if viabilidade_valor >= VIABILIDADE_EXCELENCIA {
                "Excelente".into()
            } else if viabilidade_valor >= VIABILIDADE_PADRAO {
                "Bom".into()
            } else {
                "Abaixo".into()
            },
            unidade: "%".into(),
        };

        // IEP (higher is better)
        let iep_metrica = MetricaComparacaoDto {
            nome: "Indice de Eficiencia Produtiva".into(),
            valor_atual: iep,
            valor_padrao_industria: IEP_PADRAO,
            valor_padrao_excelencia: IEP_EXCELENCIA,
            percentual_diferenca: calcular_percentual_diferenca(iep, IEP_PADRAO),
            status: if iep >= IEP_EXCELENCIA {
                "Excelente".into()
            } else if iep >= IEP_PADRAO {
                "Bom".into()
            } else {
                "Abaixo".into()
            },
            unidade: String::new(),
        };

        // Pontuacao geral: Average of scores (Excelente=100, Bom=75, Abaixo=50)
        let scores: Vec<Decimal> = [&ca_metrica, &gmd_metrica, &viab_metrica, &iep_metrica]
            .iter()
            .map(|m| match m.status.as_str() {
                "Excelente" => dec!(100),
                "Bom" => dec!(75),
                _ => dec!(50),
            })
            .collect();

        let pontuacao = scores.iter().sum::<Decimal>() / Decimal::from(scores.len() as i64);
        let pontuacao = pontuacao.round_dp(1);

        let classificacao = if pontuacao >= dec!(90) {
            "Excelente"
        } else if pontuacao >= dec!(75) {
            "Bom"
        } else if pontuacao >= dec!(60) {
            "Regular"
        } else {
            "Ruim"
        };

        Ok(ComparacaoIndustriaDto {
            lote_id,
            lote_identificador: lote.identificador,
            idade_dias,
            conversao_alimentar: ca_metrica,
            ganho_medio_diario: gmd_metrica,
            viabilidade: viab_metrica,
            iep: iep_metrica,
            peso_medio: MetricaComparacaoDto::default(), // .NET does not include peso_medio in scoring
            classificacao_geral: classificacao.into(),
            pontuacao_geral: pontuacao,
        })
    }

    // ========================================================================
    // Stub methods (matching .NET lines 354-367 returning Task.FromResult defaults)
    // ========================================================================

    pub async fn calcular_viabilidade(_pool: &PgPool, _lote_id: i32) -> Result<Decimal, AppError> {
        Ok(Decimal::ZERO)
    }

    pub async fn calcular_uniformidade(_pool: &PgPool, _lote_id: i32) -> Result<Decimal, AppError> {
        Ok(Decimal::ZERO)
    }

    pub async fn calcular_densidade_atual(
        _pool: &PgPool,
        _lote_id: i32,
    ) -> Result<Decimal, AppError> {
        Ok(Decimal::ZERO)
    }

    pub async fn obter_curvas_crescimento(
        _pool: &PgPool,
        _lote_id: i32,
    ) -> Result<CurvasCrescimentoDto, AppError> {
        Ok(CurvasCrescimentoDto::default())
    }

    pub async fn analise_consumo_detalhada(
        _pool: &PgPool,
        _lote_id: i32,
    ) -> Result<AnaliseConsumoDto, AppError> {
        Ok(AnaliseConsumoDto::default())
    }

    pub async fn obter_resumo_sanitario(
        _pool: &PgPool,
        _lote_id: i32,
    ) -> Result<ResumoSanitarioDto, AppError> {
        Ok(ResumoSanitarioDto::default())
    }

    pub async fn calcular_projecao_abate(
        _pool: &PgPool,
        _lote_id: i32,
    ) -> Result<ProjecaoAbateDto, AppError> {
        Ok(ProjecaoAbateDto::default())
    }

    pub async fn estimar_peso(
        _pool: &PgPool,
        _lote_id: i32,
        _data_abate: DateTime<Utc>,
    ) -> Result<Decimal, AppError> {
        Ok(Decimal::ZERO)
    }

    // ========================================================================
    // Composite methods
    // ========================================================================

    /// GET /{loteId}/metricas -- composite of all individual metrics
    pub async fn get_metricas(pool: &PgPool, lote_id: i32) -> Result<MetricasLoteDto, AppError> {
        Ok(MetricasLoteDto {
            iep: Self::calcular_iep(pool, lote_id).await?,
            conversao_alimentar: Self::calcular_conversao_alimentar(pool, lote_id).await?,
            ganho_medio_diario: Self::calcular_gmd(pool, lote_id).await?,
            viabilidade: Self::calcular_viabilidade(pool, lote_id).await?,
            uniformidade: Self::calcular_uniformidade(pool, lote_id).await?,
            densidade_atual: Self::calcular_densidade_atual(pool, lote_id).await?,
        })
    }

    /// GET /{loteId}/dashboard -- composite matching .NET GetDashboardCompleto
    pub async fn get_dashboard(
        pool: &PgPool,
        lote_id: i32,
    ) -> Result<DashboardAviculturaDto, AppError> {
        Ok(DashboardAviculturaDto {
            metricas: MetricasLoteDto {
                iep: Self::calcular_iep(pool, lote_id).await?,
                conversao_alimentar: Self::calcular_conversao_alimentar(pool, lote_id).await?,
                ganho_medio_diario: Self::calcular_gmd(pool, lote_id).await?,
                viabilidade: Self::calcular_viabilidade(pool, lote_id).await?,
                uniformidade: Decimal::ZERO, // not in .NET dashboard
                densidade_atual: Decimal::ZERO, // not in .NET dashboard
            },
            alertas: Self::verificar_alertas(pool, lote_id).await?,
            comparacao_industria: Self::comparar_com_industria(pool, lote_id).await?,
            resumo_sanitario: Self::obter_resumo_sanitario(pool, lote_id).await?,
            projecao_abate: Self::calcular_projecao_abate(pool, lote_id).await?,
        })
    }
}

/// Calcula o percentual de diferenca entre valor atual e padrao.
fn calcular_percentual_diferenca(atual: Decimal, padrao: Decimal) -> Decimal {
    if padrao == Decimal::ZERO {
        return Decimal::ZERO;
    }
    ((atual - padrao) / padrao * dec!(100)).round_dp(2)
}
