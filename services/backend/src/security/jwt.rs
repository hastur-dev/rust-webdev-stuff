//! JWT token generation and validation
//!
//! Uses HS256 algorithm with tokens stored in HttpOnly cookies.

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::config::SecurityConfig;
use crate::error::{AppError, AppResult, AuthError};
use crate::models::user::UserRole;

/// JWT claims structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    /// Subject (user ID)
    pub sub: String,
    /// User role
    pub role: UserRole,
    /// Expiration timestamp (Unix)
    pub exp: i64,
    /// Issued at timestamp (Unix)
    pub iat: i64,
}

/// Generate a JWT token for a user
///
/// # Arguments
/// * `user_id` - The user's unique identifier
/// * `role` - The user's role
/// * `config` - Security configuration with JWT secret
///
/// # Returns
/// JWT token string
pub fn generate_token(user_id: &str, role: &UserRole, config: &SecurityConfig) -> AppResult<String> {
    // Validate inputs
    assert!(!user_id.is_empty(), "User ID must not be empty");
    assert!(!config.jwt_secret.is_empty(), "JWT secret must not be empty");

    let now = Utc::now();
    let expiry = now + Duration::hours(config.jwt_expiry_hours);

    let claims = Claims {
        sub: user_id.to_string(),
        role: role.clone(),
        exp: expiry.timestamp(),
        iat: now.timestamp(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
    )?;

    // Postcondition: token is not empty
    assert!(!token.is_empty(), "Generated token must not be empty");

    Ok(token)
}

/// Generate a JWT token with custom expiry (for testing)
///
/// # Arguments
/// * `user_id` - The user's unique identifier
/// * `role` - The user's role
/// * `config` - Security configuration
/// * `expiry_hours` - Custom expiry in hours (can be negative for expired tokens)
pub fn generate_token_with_expiry(
    user_id: &str,
    role: &UserRole,
    config: &SecurityConfig,
    expiry_hours: i64,
) -> AppResult<String> {
    assert!(!user_id.is_empty(), "User ID must not be empty");

    let now = Utc::now();
    let expiry = now + Duration::hours(expiry_hours);

    let claims = Claims {
        sub: user_id.to_string(),
        role: role.clone(),
        exp: expiry.timestamp(),
        iat: now.timestamp(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
    )?;

    Ok(token)
}

/// Validate a JWT token and extract claims
///
/// # Arguments
/// * `token` - The JWT token string
/// * `config` - Security configuration with JWT secret
///
/// # Returns
/// Validated claims
///
/// # Errors
/// Returns AuthError if token is invalid or expired
pub fn validate_token(token: &str, config: &SecurityConfig) -> AppResult<Claims> {
    // Preconditions
    assert!(!token.is_empty(), "Token must not be empty");
    assert!(!config.jwt_secret.is_empty(), "JWT secret must not be empty");

    if token.is_empty() {
        return Err(AppError::Auth(AuthError::MissingToken));
    }

    let validation = Validation::default();

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
        &validation,
    )?;

    // Postcondition: claims are valid
    assert!(!token_data.claims.sub.is_empty(), "Subject must not be empty");

    Ok(token_data.claims)
}

/// Extract token from Authorization header or cookie
///
/// # Arguments
/// * `auth_header` - Optional Authorization header value
/// * `cookie_value` - Optional cookie value
///
/// # Returns
/// Token string if found
pub fn extract_token(auth_header: Option<&str>, cookie_value: Option<&str>) -> Option<String> {
    // Try Authorization header first (Bearer token)
    if let Some(header) = auth_header {
        if let Some(token) = header.strip_prefix("Bearer ") {
            return Some(token.to_string());
        }
    }

    // Fall back to cookie
    cookie_value.map(|s| s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> SecurityConfig {
        SecurityConfig {
            jwt_secret: "test_secret_key_must_be_32_bytes_long".to_string(),
            jwt_expiry_hours: 24,
            encryption_key: "test_encryption_key_32_bytes_xx".to_string(),
            argon2: crate::config::Argon2Config {
                memory_kib: 4096,
                iterations: 1,
                parallelism: 1,
            },
        }
    }

    #[test]
    fn test_generate_and_validate() {
        let config = test_config();
        let user_id = "user-123";
        let role = UserRole::Editor;

        let token = generate_token(user_id, &role, &config).unwrap();
        let claims = validate_token(&token, &config).unwrap();

        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.role, role);
    }

    #[test]
    fn test_expired_token_rejected() {
        let config = test_config();
        let token = generate_token_with_expiry("user-123", &UserRole::Viewer, &config, -1).unwrap();

        let result = validate_token(&token, &config);
        assert!(result.is_err());
    }

    #[test]
    fn test_wrong_secret_rejected() {
        let config = test_config();
        let token = generate_token("user-123", &UserRole::Admin, &config).unwrap();

        let mut wrong_config = config;
        wrong_config.jwt_secret = "different_secret_key_32_bytes_x".to_string();

        let result = validate_token(&token, &wrong_config);
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_token_bearer() {
        let token = extract_token(Some("Bearer abc123"), None);
        assert_eq!(token, Some("abc123".to_string()));
    }

    #[test]
    fn test_extract_token_cookie() {
        let token = extract_token(None, Some("token_from_cookie"));
        assert_eq!(token, Some("token_from_cookie".to_string()));
    }

    #[test]
    fn test_extract_token_none() {
        let token = extract_token(None, None);
        assert!(token.is_none());
    }
}
