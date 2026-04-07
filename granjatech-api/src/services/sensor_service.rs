use chrono::Utc;
use sqlx::PgPool;

use crate::dto::sensor::*;
use crate::errors::AppError;
use crate::models::sensor::{LeituraSensor, Sensor};
use crate::services::auditoria_service::AuditoriaService;

pub struct SensorService;

impl SensorService {
    /// Lista sensores filtrados por role
    /// Admin: todos, Produtor: proprios (via granja), Financeiro: bloqueado
    pub async fn list(
        pool: &PgPool,
        user_id: i32,
        user_role: &str,
    ) -> Result<Vec<SensorResponseDto>, AppError> {
        if user_role == "Financeiro" {
            return Err(AppError::Forbidden(
                "Usuarios do perfil Financeiro nao podem acessar sensores.".into(),
            ));
        }

        let sensores = match user_role {
            "Administrador" => {
                sqlx::query_as::<_, Sensor>(
                    r#"SELECT "Id", "Tipo", "IdentificadorUnico", "GranjaId"
                       FROM "Sensores"
                       ORDER BY "Id""#,
                )
                .fetch_all(pool)
                .await?
            }
            "Produtor" => {
                sqlx::query_as::<_, Sensor>(
                    r#"SELECT s."Id", s."Tipo", s."IdentificadorUnico", s."GranjaId"
                       FROM "Sensores" s
                       INNER JOIN "Granjas" g ON g."Id" = s."GranjaId"
                       WHERE g."UsuarioId" = $1
                       ORDER BY s."Id""#,
                )
                .bind(user_id)
                .fetch_all(pool)
                .await?
            }
            _ => Vec::new(),
        };

        let dtos = sensores
            .into_iter()
            .map(|s| SensorResponseDto {
                id: s.id,
                tipo: s.tipo,
                identificador_unico: s.identificador_unico,
                granja_id: s.granja_id,
            })
            .collect();

        Ok(dtos)
    }

    /// Cria novo sensor com verificacao de unicidade do IdentificadorUnico
    pub async fn create(
        pool: &PgPool,
        dto: &CreateSensorDto,
        user_id: i32,
        user_role: &str,
        user_email: &str,
    ) -> Result<SensorResponseDto, AppError> {
        if user_role == "Financeiro" {
            return Err(AppError::Forbidden(
                "Usuarios do perfil Financeiro nao podem criar sensores.".into(),
            ));
        }

        // Verifica acesso a granja
        Self::verificar_acesso_granja(pool, dto.granja_id, user_id, user_role).await?;

        // Verifica unicidade do IdentificadorUnico
        let existe: bool = sqlx::query_scalar(
            r#"SELECT EXISTS(SELECT 1 FROM "Sensores" WHERE "IdentificadorUnico" = $1)"#,
        )
        .bind(&dto.identificador_unico)
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

        if existe {
            return Err(AppError::BadRequest(
                "Sensor com este identificador unico ja existe.".into(),
            ));
        }

        let sensor = sqlx::query_as::<_, Sensor>(
            r#"INSERT INTO "Sensores" ("Tipo", "IdentificadorUnico", "GranjaId")
               VALUES ($1, $2, $3)
               RETURNING "Id", "Tipo", "IdentificadorUnico", "GranjaId""#,
        )
        .bind(&dto.tipo)
        .bind(&dto.identificador_unico)
        .bind(dto.granja_id)
        .fetch_one(pool)
        .await?;

        AuditoriaService::registrar_log(
            pool,
            user_id,
            user_email,
            "CRIACAO_SENSOR",
            &format!(
                "Sensor '{}' (ID: {}) adicionado a Granja ID: {}.",
                sensor.identificador_unico, sensor.id, sensor.granja_id
            ),
        )
        .await?;

        Ok(SensorResponseDto {
            id: sensor.id,
            tipo: sensor.tipo,
            identificador_unico: sensor.identificador_unico,
            granja_id: sensor.granja_id,
        })
    }

    /// Deleta sensor com verificacao de propriedade
    pub async fn delete(
        pool: &PgPool,
        id: i32,
        user_id: i32,
        user_role: &str,
        user_email: &str,
    ) -> Result<(), AppError> {
        if user_role == "Financeiro" {
            return Err(AppError::Forbidden(
                "Usuarios do perfil Financeiro nao podem deletar sensores.".into(),
            ));
        }

        // Busca sensor com granja
        let sensor = sqlx::query_as::<_, Sensor>(
            r#"SELECT "Id", "Tipo", "IdentificadorUnico", "GranjaId"
               FROM "Sensores"
               WHERE "Id" = $1"#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Sensor com ID {} nao encontrado", id)))?;

        // Verifica acesso a granja do sensor
        Self::verificar_acesso_granja(pool, sensor.granja_id, user_id, user_role).await?;

        sqlx::query(r#"DELETE FROM "Sensores" WHERE "Id" = $1"#)
            .bind(id)
            .execute(pool)
            .await?;

        AuditoriaService::registrar_log(
            pool,
            user_id,
            user_email,
            "DELECAO_SENSOR",
            &format!(
                "Sensor '{}' (ID: {}) deletado.",
                sensor.identificador_unico, id
            ),
        )
        .await?;

        Ok(())
    }

    /// Lista leituras de um sensor com verificacao de acesso
    pub async fn list_leituras(
        pool: &PgPool,
        sensor_id: i32,
        user_id: i32,
        user_role: &str,
    ) -> Result<Vec<LeituraSensorResponseDto>, AppError> {
        // Busca sensor para verificar acesso
        let sensor = sqlx::query_as::<_, Sensor>(
            r#"SELECT "Id", "Tipo", "IdentificadorUnico", "GranjaId"
               FROM "Sensores"
               WHERE "Id" = $1"#,
        )
        .bind(sensor_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| {
            AppError::NotFound(format!("Sensor com ID {} nao encontrado", sensor_id))
        })?;

        // Verifica acesso a granja do sensor
        Self::verificar_acesso_granja(pool, sensor.granja_id, user_id, user_role).await?;

        let leituras = sqlx::query_as::<_, LeituraSensor>(
            r#"SELECT "Id", "Valor", "Timestamp", "SensorId"
               FROM "LeiturasSensores"
               WHERE "SensorId" = $1
               ORDER BY "Timestamp" DESC
               LIMIT 100"#,
        )
        .bind(sensor_id)
        .fetch_all(pool)
        .await?;

        let dtos = leituras
            .into_iter()
            .map(|l| LeituraSensorResponseDto {
                id: l.id,
                sensor_id: l.sensor_id,
                valor: l.valor,
                timestamp: l.timestamp,
            })
            .collect();

        Ok(dtos)
    }

    /// Registra leitura de sensor -- endpoint PUBLICO (sem autenticacao)
    /// Dispositivos IoT enviam leituras identificando o sensor pelo IdentificadorUnico
    pub async fn registrar_leitura(
        pool: &PgPool,
        dto: &CreateLeituraDto,
    ) -> Result<LeituraSensorResponseDto, AppError> {
        // Busca sensor pelo IdentificadorUnico
        let sensor_id: Option<i32> = sqlx::query_scalar(
            r#"SELECT "Id" FROM "Sensores" WHERE "IdentificadorUnico" = $1"#,
        )
        .bind(&dto.identificador_unico)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

        let sensor_id =
            sensor_id.ok_or_else(|| AppError::NotFound("Sensor nao encontrado.".into()))?;

        let now = Utc::now();
        let leitura = sqlx::query_as::<_, LeituraSensor>(
            r#"INSERT INTO "LeiturasSensores" ("Valor", "Timestamp", "SensorId")
               VALUES ($1, $2, $3)
               RETURNING "Id", "Valor", "Timestamp", "SensorId""#,
        )
        .bind(dto.valor)
        .bind(now)
        .bind(sensor_id)
        .fetch_one(pool)
        .await?;

        Ok(LeituraSensorResponseDto {
            id: leitura.id,
            sensor_id: leitura.sensor_id,
            valor: leitura.valor,
            timestamp: leitura.timestamp,
        })
    }

    /// Verifica se o usuario tem acesso a granja especificada
    async fn verificar_acesso_granja(
        pool: &PgPool,
        granja_id: i32,
        user_id: i32,
        user_role: &str,
    ) -> Result<(), AppError> {
        match user_role {
            "Administrador" => Ok(()),
            "Produtor" => {
                let is_owner: bool = sqlx::query_scalar(
                    r#"SELECT EXISTS(
                        SELECT 1 FROM "Granjas"
                        WHERE "Id" = $1 AND "UsuarioId" = $2
                    )"#,
                )
                .bind(granja_id)
                .bind(user_id)
                .fetch_one(pool)
                .await
                .map_err(|e| AppError::Internal(e.to_string()))?;

                if is_owner {
                    Ok(())
                } else {
                    Err(AppError::Forbidden(
                        "Permissao negada ou granja invalida.".into(),
                    ))
                }
            }
            _ => Err(AppError::Forbidden("Perfil nao reconhecido.".into())),
        }
    }
}
