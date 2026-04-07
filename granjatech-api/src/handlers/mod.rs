use actix_web::web;

pub mod auth;
pub mod consumo;
pub mod dashboard;
pub mod financas;
pub mod granjas;
pub mod lotes;
pub mod pesagem;
pub mod sanitario;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/auth")
                    .route("/login", web::post().to(auth::login))
                    .route("/registrar", web::post().to(auth::registrar))
                    .route("/usuarios", web::get().to(auth::get_usuarios))
                    .route("/usuarios/{id}", web::get().to(auth::get_usuario))
                    .route("/usuarios/{id}", web::put().to(auth::update_usuario))
                    .route("/usuarios/{id}", web::delete().to(auth::delete_usuario)),
            )
            .service(
                web::scope("/granjas")
                    .route("", web::get().to(granjas::get_granjas))
                    .route("/{id}", web::get().to(granjas::get_granja))
                    .route("", web::post().to(granjas::create_granja))
                    .route("/{id}", web::put().to(granjas::update_granja))
                    .route("/{id}", web::delete().to(granjas::delete_granja)),
            )
            .service(
                web::scope("/lotes")
                    .route("", web::get().to(lotes::get_lotes))
                    .route("/{id}", web::get().to(lotes::get_lote))
                    .route("", web::post().to(lotes::create_lote))
                    .route("/{id}", web::put().to(lotes::update_lote))
                    .route("/{id}", web::delete().to(lotes::delete_lote))
                    .route("/{id}/mortalidades", web::post().to(lotes::registrar_mortalidade))
                    .route("/{id}/mortalidades", web::get().to(lotes::listar_mortalidades)),
            )
            .service(
                web::scope("/dashboard")
                    .route("/kpis", web::get().to(dashboard::get_kpis))
                    .route("/resumo-mensal", web::get().to(dashboard::get_resumo_mensal)),
            )
            .service(
                web::scope("/financas")
                    .route("", web::get().to(financas::get_transacoes))
                    .route("", web::post().to(financas::create_transacao))
                    .route("/{id}", web::put().to(financas::update_transacao))
                    .route("/{id}", web::delete().to(financas::delete_transacao)),
            )
            .service(
                web::scope("/consumo")
                    .route("/racao", web::post().to(consumo::create_consumo_racao))
                    .route("/agua", web::post().to(consumo::create_consumo_agua))
                    .route("/racao/{loteId}", web::get().to(consumo::list_consumo_racao))
                    .route("/agua/{loteId}", web::get().to(consumo::list_consumo_agua))
                    .route("/resumo/{loteId}", web::get().to(consumo::resumo_consumo)),
            )
            .service(
                web::scope("/pesagem")
                    .route("", web::post().to(pesagem::create_pesagem))
                    .route("/resumo/{loteId}", web::get().to(pesagem::resumo_pesagens))
                    .route("/{loteId}", web::get().to(pesagem::list_pesagens)),
            )
            .service(
                web::scope("/sanitario")
                    .route("/cronograma-vacinacao", web::get().to(sanitario::cronograma_vacinacao))
                    .route("/resumo/{loteId}", web::get().to(sanitario::resumo_sanitario))
                    .route("", web::post().to(sanitario::create_evento))
                    .route("/{loteId}", web::get().to(sanitario::list_eventos)),
            ),
    );
}
