use actix_web::web;

pub mod auth;
pub mod dashboard;
pub mod granjas;
pub mod lotes;

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
            ),
    );
}
