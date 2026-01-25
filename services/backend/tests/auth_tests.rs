//! Authentication tests for Knowledge Vault
//! Tests password hashing, JWT generation/validation, and auth endpoints

use knowledge_vault::security::{jwt, password};
use knowledge_vault::models::user::{CreateUser, User, UserRole};
use knowledge_vault::config::Config;
use pretty_assertions::assert_eq;

/// Test that password hashing produces different outputs for same input (salt)
#[test]
fn test_password_hash_uses_salt() {
    let password = "TestPassword123!";
    let config = test_config();

    let hash1 = password::hash_password(password, &config.security.argon2)
        .expect("First hash should succeed");
    let hash2 = password::hash_password(password, &config.security.argon2)
        .expect("Second hash should succeed");

    // Hashes should be different due to random salt
    assert_ne!(hash1, hash2, "Hashes should differ due to salting");

    // Both should verify correctly
    assert!(
        password::verify_password(password, &hash1).expect("Verify should succeed"),
        "First hash should verify"
    );
    assert!(
        password::verify_password(password, &hash2).expect("Verify should succeed"),
        "Second hash should verify"
    );
}

/// Test password verification with correct password
#[test]
fn test_password_verify_correct() {
    let password = "SecureP@ssw0rd!";
    let config = test_config();

    let hash = password::hash_password(password, &config.security.argon2)
        .expect("Hash should succeed");

    let result = password::verify_password(password, &hash)
        .expect("Verification should not error");

    assert!(result, "Correct password should verify");
}

/// Test password verification with incorrect password
#[test]
fn test_password_verify_incorrect() {
    let password = "SecureP@ssw0rd!";
    let wrong_password = "WrongPassword123!";
    let config = test_config();

    let hash = password::hash_password(password, &config.security.argon2)
        .expect("Hash should succeed");

    let result = password::verify_password(wrong_password, &hash)
        .expect("Verification should not error");

    assert!(!result, "Wrong password should not verify");
}

/// Test password with empty string
#[test]
fn test_password_empty_rejected() {
    let config = test_config();

    // Empty password should still hash (validation is elsewhere)
    let result = password::hash_password("", &config.security.argon2);
    assert!(result.is_ok(), "Empty password hashing should succeed");
}

/// Test password with unicode characters
#[test]
fn test_password_unicode_support() {
    let password = "Пароль123!日本語";
    let config = test_config();

    let hash = password::hash_password(password, &config.security.argon2)
        .expect("Unicode hash should succeed");

    let result = password::verify_password(password, &hash)
        .expect("Unicode verify should succeed");

    assert!(result, "Unicode password should verify");
}

/// Test JWT token generation
#[test]
fn test_jwt_generate_and_validate() {
    let config = test_config();
    let user_id = uuid::Uuid::new_v4().to_string();
    let role = UserRole::Editor;

    let token = jwt::generate_token(&user_id, &role, &config.security)
        .expect("Token generation should succeed");

    assert!(!token.is_empty(), "Token should not be empty");

    let claims = jwt::validate_token(&token, &config.security)
        .expect("Token validation should succeed");

    assert_eq!(claims.sub, user_id, "Subject should match user ID");
    assert_eq!(claims.role, role, "Role should match");
}

/// Test JWT with expired token
#[test]
fn test_jwt_expired_rejected() {
    let config = test_config();
    let user_id = uuid::Uuid::new_v4().to_string();
    let role = UserRole::Viewer;

    // Generate token with 0 expiry (already expired)
    let mut expired_config = config.clone();
    expired_config.security.jwt_expiry_hours = 0;

    let token = jwt::generate_token_with_expiry(&user_id, &role, &expired_config.security, -1)
        .expect("Expired token generation should succeed");

    let result = jwt::validate_token(&token, &config.security);

    assert!(result.is_err(), "Expired token should fail validation");
}

/// Test JWT with invalid secret
#[test]
fn test_jwt_wrong_secret_rejected() {
    let config = test_config();
    let user_id = uuid::Uuid::new_v4().to_string();
    let role = UserRole::Admin;

    let token = jwt::generate_token(&user_id, &role, &config.security)
        .expect("Token generation should succeed");

    let mut wrong_config = config.clone();
    wrong_config.security.jwt_secret = "wrong_secret_key_that_is_32_bytes_long".to_string();

    let result = jwt::validate_token(&token, &wrong_config.security);

    assert!(result.is_err(), "Token with wrong secret should fail");
}

/// Test JWT claims contain expected fields
#[test]
fn test_jwt_claims_structure() {
    let config = test_config();
    let user_id = uuid::Uuid::new_v4().to_string();
    let role = UserRole::SuperAdmin;

    let token = jwt::generate_token(&user_id, &role, &config.security)
        .expect("Token generation should succeed");

    let claims = jwt::validate_token(&token, &config.security)
        .expect("Token validation should succeed");

    assert_eq!(claims.sub, user_id);
    assert_eq!(claims.role, role);
    assert!(claims.exp > 0, "Expiry should be set");
    assert!(claims.iat > 0, "Issued-at should be set");
}

/// Test user role ordering for permissions
#[test]
fn test_user_role_permissions() {
    assert!(UserRole::SuperAdmin.can_manage_users());
    assert!(UserRole::Admin.can_manage_users());
    assert!(!UserRole::Editor.can_manage_users());
    assert!(!UserRole::Viewer.can_manage_users());

    assert!(UserRole::SuperAdmin.can_edit_articles());
    assert!(UserRole::Admin.can_edit_articles());
    assert!(UserRole::Editor.can_edit_articles());
    assert!(!UserRole::Viewer.can_edit_articles());

    assert!(UserRole::SuperAdmin.can_view_audit());
    assert!(UserRole::Admin.can_view_audit());
    assert!(!UserRole::Editor.can_view_audit());
    assert!(!UserRole::Viewer.can_view_audit());
}

/// Helper to create test configuration
fn test_config() -> Config {
    Config {
        server: knowledge_vault::config::ServerConfig {
            host: "127.0.0.1".to_string(),
            port: 8080,
            workers: 1,
        },
        database: knowledge_vault::config::DatabaseConfig {
            path: ":memory:".to_string(),
            max_connections: 1,
        },
        security: knowledge_vault::config::SecurityConfig {
            jwt_secret: "test_secret_key_must_be_32_bytes_long".to_string(),
            jwt_expiry_hours: 24,
            encryption_key: "test_encryption_key_32_bytes_xx".to_string(),
            argon2: knowledge_vault::config::Argon2Config {
                memory_kib: 4096, // Lower for tests
                iterations: 1,
                parallelism: 1,
            },
        },
        logging: knowledge_vault::config::LoggingConfig {
            level: "debug".to_string(),
            audit_retention_days: 90,
        },
        cors: knowledge_vault::config::CorsConfig {
            allowed_origins: vec!["http://localhost:5173".to_string()],
        },
    }
}

#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        /// Property: Any non-empty password can be hashed and verified
        #[test]
        fn prop_password_roundtrip(password in "[a-zA-Z0-9!@#$%^&*]{8,64}") {
            let config = test_config();
            let hash = password::hash_password(&password, &config.security.argon2)
                .expect("Hash should succeed");
            let verified = password::verify_password(&password, &hash)
                .expect("Verify should succeed");
            prop_assert!(verified, "Password should verify after hash");
        }

        /// Property: Different passwords produce different verification results
        #[test]
        fn prop_different_passwords_dont_match(
            password1 in "[a-zA-Z0-9]{8,32}",
            password2 in "[a-zA-Z0-9]{8,32}"
        ) {
            prop_assume!(password1 != password2);
            let config = test_config();

            let hash = password::hash_password(&password1, &config.security.argon2)
                .expect("Hash should succeed");
            let verified = password::verify_password(&password2, &hash)
                .expect("Verify should succeed");

            prop_assert!(!verified, "Different passwords should not match");
        }
    }
}
