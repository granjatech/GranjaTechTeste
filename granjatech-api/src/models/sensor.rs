use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Sensor instalado em uma granja
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Sensor {
    #[sqlx(rename = "Id")]
    pub id: i32,
    #[sqlx(rename = "Tipo")]
    pub tipo: String,
    #[sqlx(rename = "IdentificadorUnico")]
    pub identificador_unico: String,
    #[sqlx(rename = "GranjaId")]
    pub granja_id: i32,
}

/// Leitura individual de um sensor
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct LeituraSensor {
    #[sqlx(rename = "Id")]
    pub id: i32,
    #[sqlx(rename = "Valor")]
    pub valor: Decimal,
    #[sqlx(rename = "Timestamp")]
    pub timestamp: DateTime<Utc>,
    #[sqlx(rename = "SensorId")]
    pub sensor_id: i32,
}
