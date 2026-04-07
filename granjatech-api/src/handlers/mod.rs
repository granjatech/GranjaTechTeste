use actix_web::web;

pub mod auth;
pub mod granjas;

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
            ),
    );
}
