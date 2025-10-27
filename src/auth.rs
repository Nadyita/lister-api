use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use crate::config::Config;

pub async fn auth_middleware(
    State(config): State<Config>,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, AuthError> {
    // If no auth token is configured, allow all requests
    let Some(expected_token) = &config.auth_token else {
        return Ok(next.run(request).await);
    };

    // Extract Authorization header
    let auth_header = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(AuthError::MissingToken)?;

    // Check if it starts with "Bearer "
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(AuthError::InvalidFormat)?;

    // Validate token
    if token != expected_token {
        return Err(AuthError::InvalidToken);
    }

    // Token is valid, proceed with request
    Ok(next.run(request).await)
}

#[derive(Debug)]
pub enum AuthError {
    MissingToken,
    InvalidFormat,
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::MissingToken => (
                StatusCode::UNAUTHORIZED,
                "Missing authorization header",
            ),
            AuthError::InvalidFormat => (
                StatusCode::UNAUTHORIZED,
                "Invalid authorization header format. Expected: Bearer <token>",
            ),
            AuthError::InvalidToken => (
                StatusCode::UNAUTHORIZED,
                "Invalid authentication token",
            ),
        };

        let body = json!({
            "error": error_message,
            "status": status.as_u16(),
        });

        (status, Json(body)).into_response()
    }
}

