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
        handlers::lotes::get_lotes,
        handlers::lotes::get_lote,
        handlers::lotes::create_lote,
        handlers::lotes::update_lote,
        handlers::lotes::delete_lote,
        handlers::lotes::registrar_mortalidade,
        handlers::lotes::listar_mortalidades,
        handlers::dashboard::get_kpis,
        handlers::dashboard::get_resumo_mensal,
        handlers::financas::get_transacoes,
        handlers::financas::create_transacao,
        handlers::financas::update_transacao,
        handlers::financas::delete_transacao,
        handlers::consumo::create_consumo_racao,
        handlers::consumo::create_consumo_agua,
        handlers::consumo::list_consumo_racao,
        handlers::consumo::list_consumo_agua,
        handlers::consumo::resumo_consumo,
        handlers::pesagem::create_pesagem,
        handlers::pesagem::list_pesagens,
        handlers::pesagem::resumo_pesagens,
        handlers::sanitario::create_evento,
        handlers::sanitario::list_eventos,
        handlers::sanitario::resumo_sanitario,
        handlers::sanitario::cronograma_vacinacao,
        handlers::sensores::get_sensores,
        handlers::sensores::create_sensor,
        handlers::sensores::delete_sensor,
        handlers::sensores::get_leituras_sensor,
        handlers::leituras::post_leitura,
        handlers::estoque::get_produtos,
        handlers::estoque::create_produto,
        handlers::estoque::update_produto,
        handlers::estoque::delete_produto,
        handlers::auditoria::get_logs,
        handlers::profile::get_profile,
        handlers::profile::update_profile,
        handlers::profile::change_password,
        handlers::avicultura::get_metricas,
        handlers::avicultura::get_analise_consumo,
        handlers::avicultura::get_curvas_crescimento,
        handlers::avicultura::get_resumo_sanitario,
        handlers::avicultura::get_alertas,
        handlers::avicultura::get_comparacao_industria,
        handlers::avicultura::get_projecao_abate,
        handlers::avicultura::estimar_peso,
        handlers::avicultura::get_dashboard,
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
        dto::lote::CreateLoteDto,
        dto::lote::UpdateLoteDto,
        dto::lote::LoteResponseDto,
        dto::sanitario::CreateRegistroMortalidadeDto,
        dto::sanitario::RegistroMortalidadeDto,
        dto::sanitario::CreateEventoSanitarioDto,
        dto::dashboard::DashboardKpiDto,
        dto::dashboard::MonthlySummaryDto,
        dto::financeiro::CreateTransacaoDto,
        dto::financeiro::UpdateTransacaoDto,
        dto::financeiro::TransacaoSimplificadaDto,
        dto::consumo::CreateConsumoRacaoDto,
        dto::consumo::CreateConsumoAguaDto,
        dto::consumo::ConsumoRacaoResponseDto,
        dto::consumo::ConsumoAguaResponseDto,
        dto::pesagem::CreatePesagemSemanalDto,
        dto::pesagem::PesagemSemanalResponseDto,
        dto::sensor::CreateSensorDto,
        dto::sensor::SensorResponseDto,
        dto::sensor::CreateLeituraDto,
        dto::sensor::LeituraSensorResponseDto,
        dto::estoque::CreateProdutoDto,
        dto::estoque::UpdateProdutoDto,
        dto::estoque::ProdutoResponseDto,
        dto::profile::ProfileDetailDto,
        dto::profile::UpdateProfileDto,
        dto::profile::ChangePasswordDto,
        dto::avicultura::MetricasLoteDto,
        dto::avicultura::DashboardAviculturaDto,
        dto::avicultura::EstimarPesoResponseDto,
        dto::avicultura::AlertaParametroDto,
        dto::avicultura::ComparacaoIndustriaDto,
        dto::avicultura::MetricaComparacaoDto,
        dto::avicultura::AnaliseConsumoDto,
        dto::avicultura::ConsumoFaseDto,
        dto::avicultura::CurvasCrescimentoDto,
        dto::avicultura::PontoCurvaDto,
        dto::avicultura::ProjecaoAbateDto,
        dto::avicultura::ResumoSanitarioDto,
        dto::avicultura::EventoSanitarioResumoDto,
        dto::avicultura::VacinacaoScheduleDto,
        dto::avicultura::ProximaAcaoSanitariaDto,
        dto::avicultura::RegistroMortalidadePorFaseDto,
    )),
    tags(
        (name = "auth", description = "Autenticacao e gerenciamento de usuarios"),
        (name = "granjas", description = "CRUD de granjas"),
        (name = "lotes", description = "CRUD de lotes de aves"),
        (name = "dashboard", description = "KPIs e resumo mensal"),
        (name = "financas", description = "Transacoes financeiras"),
        (name = "consumo", description = "Consumo de racao e agua"),
        (name = "pesagem", description = "Pesagem semanal"),
        (name = "sanitario", description = "Eventos sanitarios e vacinacao"),
        (name = "sensores", description = "Sensores IoT"),
        (name = "leituras", description = "Leituras de sensores (publico)"),
        (name = "estoque", description = "Produtos em estoque"),
        (name = "auditoria", description = "Logs de auditoria"),
        (name = "profile", description = "Perfil do usuario"),
        (name = "avicultura", description = "Avicultura analytics e metricas")
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
