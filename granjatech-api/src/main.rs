mod config;
mod db;
mod errors;
mod models;

use actix_web::{web, App, HttpServer, HttpResponse};
use tracing_actix_web::TracingLogger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Inicializar tracing com env-filter (FOUND-05)
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    tracing::info!("Carregando configuracao...");
    let config = config::Config::from_env();

    tracing::info!("Conectando ao banco de dados...");
    let pool = db::create_pool(&config.database_url).await;

    tracing::info!("Servidor iniciando na porta 8080...");

    let config_data = web::Data::new(config);
    let pool_data = web::Data::new(pool);

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(config_data.clone())
            .app_data(pool_data.clone())
            .route(
                "/health",
                web::get().to(|| async {
                    HttpResponse::Ok().json(serde_json::json!({"status": "ok"}))
                }),
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
