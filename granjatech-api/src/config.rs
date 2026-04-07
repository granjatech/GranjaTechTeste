pub struct Config {
    pub database_url: String,
    pub jwt_key: String,
    pub jwt_issuer: String,
    pub jwt_audience: String,
    pub allowed_origins: Vec<String>,
    pub swagger_enabled: bool,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL deve estar definida");
        let jwt_key = std::env::var("JWT_KEY")
            .expect("JWT_KEY deve estar definida");
        let jwt_issuer = std::env::var("JWT_ISSUER")
            .unwrap_or_else(|_| "GranjaTechAPI".to_string());
        let jwt_audience = std::env::var("JWT_AUDIENCE")
            .unwrap_or_else(|_| "GranjaTechApp".to_string());
        let allowed_origins = std::env::var("ALLOWED_ORIGINS")
            .unwrap_or_else(|_| "http://localhost:3000".to_string())
            .split(';')
            .map(|s| s.trim().to_string())
            .collect();
        let swagger_enabled = std::env::var("SWAGGER_ENABLED")
            .unwrap_or_else(|_| "true".to_string())
            .parse::<bool>()
            .unwrap_or(true);

        Config {
            database_url,
            jwt_key,
            jwt_issuer,
            jwt_audience,
            allowed_origins,
            swagger_enabled,
        }
    }
}
