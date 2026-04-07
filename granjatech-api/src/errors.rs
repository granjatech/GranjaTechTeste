use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    BadRequest(String),
    Unauthorized(String),
    Forbidden(String),
    Internal(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound(msg) => write!(f, "{}", msg),
            AppError::BadRequest(msg) => write!(f, "{}", msg),
            AppError::Unauthorized(msg) => write!(f, "{}", msg),
            AppError::Forbidden(msg) => write!(f, "{}", msg),
            AppError::Internal(msg) => write!(f, "{}", msg),
        }
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::Forbidden(_) => StatusCode::FORBIDDEN,
            AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let message = match self {
            AppError::NotFound(msg) => msg,
            AppError::BadRequest(msg) => msg,
            AppError::Unauthorized(msg) => msg,
            AppError::Forbidden(msg) => msg,
            AppError::Internal(msg) => msg,
        };
        HttpResponse::build(self.status_code())
            .json(serde_json::json!({"message": message}))
    }
}

// Conversao de sqlx::Error para AppError
impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        AppError::Internal(e.to_string())
    }
}
