mod config;
mod db;
mod dto;
mod errors;
mod handlers;
mod middleware;
mod models;
mod services;

use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer};
use tracing_actix_web::TracingLogger;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::auth::login,
        handlers::auth::registrar,
        handlers::auth::get_usuarios,
        handlers::auth::get_usuario,
        handlers::auth::update_usuario,
        handlers::auth::delete_usuario,
        handlers::granjas::get_granjas,
        handlers::granjas::get_granja,
        handlers::granjas::create_granja,
        handlers::granjas::update_granja,
        handlers::granjas::delete_granja,
    ),
    components(schemas(
        dto::auth::LoginDto,
        dto::auth::RegisterDto,
        dto::auth::LoginResponseDto,
        dto::auth::UserDto,
        dto::auth::UserDetailDto,
        dto::auth::UpdateUserDto,
        dto::granja::CreateGranjaDto,
        dto::granja::UpdateGranjaDto,
        dto::granja::GranjaResponseDto,
    )),
    tags(
        (name = "auth", description = "Autenticacao e gerenciamento de usuarios"),
        (name = "granjas", description = "CRUD de granjas")
    )
)]
struct ApiDoc;

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

    let swagger_enabled = config.swagger_enabled;
    let allowed_origins = config.allowed_origins.clone();

    tracing::info!("Servidor iniciando na porta 8080...");

    let config_data = web::Data::new(config);
    let pool_data = web::Data::new(pool);

    HttpServer::new(move || {
        // CORS configurado com origens explicitas (FOUND-02, T-1-05)
        let mut cors = Cors::default()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::CONTENT_TYPE,
                actix_web::http::header::ACCEPT,
            ])
            .max_age(3600);

        for origin in &allowed_origins {
            cors = cors.allowed_origin(origin);
        }

        let mut app = App::new()
            .wrap(TracingLogger::default())
            .wrap(cors)
            .app_data(config_data.clone())
            .app_data(pool_data.clone())
            .configure(handlers::configure_routes)
            .route(
                "/health",
                web::get().to(|| async {
                    HttpResponse::Ok().json(serde_json::json!({"status": "ok"}))
                }),
            );

        // Swagger UI condicionalmente habilitado (FOUND-04)
        if swagger_enabled {
            app = app.service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            );
        }

        app
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
