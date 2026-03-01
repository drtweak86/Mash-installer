//! Error types

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error("Startup error: {0}")]
    Startup(String),
    
    #[error("Runtime error: {0}")]
    Runtime(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Configuration error: {0}")]
    Configuration(String),
    
    #[error("GitHub API error: {0}")]
    GitHubApi(String),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let status = match self {
            ServerError::Startup(_) | ServerError::Runtime(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ServerError::Validation(_) => StatusCode::BAD_REQUEST,
            ServerError::Configuration(_) => StatusCode::BAD_REQUEST,
            ServerError::GitHubApi(_) => StatusCode::BAD_GATEWAY,
        };

        #[derive(Serialize)]
        struct ErrorResponse {
            error: String,
            message: String,
        }

        let body = Json(ErrorResponse {
            error: status.to_string(),
            message: self.to_string(),
        });

        (status, body).into_response()
    }
}
