use actix_web::{dev::Payload, web, FromRequest, HttpRequest};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};

use crate::config::Config;
use crate::errors::AppError;

/// Claims do token JWT -- campos correspondem ao formato .NET
/// nameid = ClaimTypes.NameIdentifier (ID do usuario como string)
/// email = ClaimTypes.Email
/// role = ClaimTypes.Role (nome do perfil: Administrador, Produtor, Financeiro)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub nameid: String,
    pub email: String,
    pub role: String,
    pub exp: usize,
    pub iss: String,
    pub aud: String,
}

impl Claims {
    /// Converte nameid (string) para i32 (ID do usuario)
    pub fn user_id(&self) -> Result<i32, AppError> {
        self.nameid
            .parse::<i32>()
            .map_err(|_| AppError::Unauthorized("Token invalido: nameid nao e um inteiro".into()))
    }
}

impl FromRequest for Claims {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let resultado = extrair_claims(req);
        ready(resultado.map_err(actix_web::Error::from))
    }
}

/// Extrai e valida Claims do header Authorization: Bearer <token>
fn extrair_claims(req: &HttpRequest) -> Result<Claims, AppError> {
    let config = req
        .app_data::<web::Data<Config>>()
        .ok_or_else(|| AppError::Internal("Configuracao nao encontrada".into()))?;

    let auth_header = req
        .headers()
        .get("Authorization")
        .ok_or_else(|| AppError::Unauthorized("Token nao fornecido".into()))?
        .to_str()
        .map_err(|_| AppError::Unauthorized("Header Authorization invalido".into()))?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| {
            AppError::Unauthorized("Formato do token invalido. Use: Bearer <token>".into())
        })?;

    let mut validacao = Validation::new(Algorithm::HS256);
    validacao.set_issuer(&[&config.jwt_issuer]);
    validacao.set_audience(&[&config.jwt_audience]);

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.jwt_key.as_bytes()),
        &validacao,
    )
    .map_err(|e| AppError::Unauthorized(format!("Token invalido: {}", e)))?;

    Ok(token_data.claims)
}
