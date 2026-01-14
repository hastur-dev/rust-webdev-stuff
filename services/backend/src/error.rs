//! Unified error handling for Knowledge Vault
//!
//! Provides consistent error types and HTTP response mapping.

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use std::fmt;

/// Application-wide error type
#[derive(Debug)]
pub enum AppError {
    /// Authentication/authorization errors
    Auth(AuthError),
    /// Database errors
    Database(String),
    /// Validation errors
    Validation(String),
    /// Not found errors
    NotFound(String),
    /// Encryption/decryption errors
    Encryption(String),
    /// Configuration errors
    Config(String),
    /// Internal server errors
    Internal(String),
}

/// Authentication-specific errors
#[derive(Debug)]
pub enum AuthError {
    /// Invalid credentials
    InvalidCredentials,
    /// Token expired
    TokenExpired,
    /// Token invalid
    TokenInvalid,
    /// Missing token
    MissingToken,
    /// Insufficient permissions
    Forbidden,
    /// Account disabled
    AccountDisabled,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Auth(e) => write!(f, "Authentication error: {}", e),
            AppError::Database(msg) => write!(f, "Database error: {}", msg),
            AppError::Validation(msg) => write!(f, "Validation error: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::Encryption(msg) => write!(f, "Encryption error: {}", msg),
            AppError::Config(msg) => write!(f, "Configuration error: {}", msg),
            AppError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthError::InvalidCredentials => write!(f, "Invalid username or password"),
            AuthError::TokenExpired => write!(f, "Token has expired"),
            AuthError::TokenInvalid => write!(f, "Invalid token"),
            AuthError::MissingToken => write!(f, "Authentication required"),
            AuthError::Forbidden => write!(f, "Insufficient permissions"),
            AuthError::AccountDisabled => write!(f, "Account is disabled"),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let (status, message) = match self {
            AppError::Auth(auth_err) => match auth_err {
                AuthError::InvalidCredentials => {
                    (StatusCode::UNAUTHORIZED, "Invalid username or password")
                }
                AuthError::TokenExpired => (StatusCode::UNAUTHORIZED, "Token has expired"),
                AuthError::TokenInvalid => (StatusCode::UNAUTHORIZED, "Invalid token"),
                AuthError::MissingToken => (StatusCode::UNAUTHORIZED, "Authentication required"),
                AuthError::Forbidden => (StatusCode::FORBIDDEN, "Insufficient permissions"),
                AuthError::AccountDisabled => (StatusCode::FORBIDDEN, "Account is disabled"),
            },
            AppError::Database(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Database operation failed")
            }
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, msg.as_str()),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.as_str()),
            AppError::Encryption(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Encryption operation failed",
            ),
            AppError::Config(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Configuration error",
            ),
            AppError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
        };

        HttpResponse::build(status).json(serde_json::json!({
            "error": message,
            "status": status.as_u16()
        }))
    }

    fn status_code(&self) -> StatusCode {
        match self {
            AppError::Auth(auth_err) => match auth_err {
                AuthError::InvalidCredentials
                | AuthError::TokenExpired
                | AuthError::TokenInvalid
                | AuthError::MissingToken => StatusCode::UNAUTHORIZED,
                AuthError::Forbidden | AuthError::AccountDisabled => StatusCode::FORBIDDEN,
            },
            AppError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Validation(_) => StatusCode::BAD_REQUEST,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Encryption(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Config(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl std::error::Error for AppError {}

// Conversion implementations

impl From<rusqlite::Error> for AppError {
    fn from(err: rusqlite::Error) -> Self {
        AppError::Database(err.to_string())
    }
}

impl From<r2d2::Error> for AppError {
    fn from(err: r2d2::Error) -> Self {
        AppError::Database(err.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::Validation(err.to_string())
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        use jsonwebtoken::errors::ErrorKind;
        match err.kind() {
            ErrorKind::ExpiredSignature => AppError::Auth(AuthError::TokenExpired),
            _ => AppError::Auth(AuthError::TokenInvalid),
        }
    }
}

impl From<argon2::password_hash::Error> for AppError {
    fn from(err: argon2::password_hash::Error) -> Self {
        AppError::Internal(format!("Password hashing error: {}", err))
    }
}

impl From<aes_gcm::Error> for AppError {
    fn from(_err: aes_gcm::Error) -> Self {
        AppError::Encryption("AES-GCM operation failed".to_string())
    }
}

/// Result type alias for AppError
pub type AppResult<T> = Result<T, AppError>;
