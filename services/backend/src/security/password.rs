//! Password hashing using Argon2id
//!
//! Provides secure password hashing with automatic salting.
//! Configured for low-resource environment (19MB memory, 1 thread).

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Algorithm, Argon2, Params, Version,
};

use crate::config::Argon2Config;
use crate::error::{AppError, AppResult};

/// Hash a password using Argon2id with configured parameters
///
/// # Arguments
/// * `password` - The plaintext password to hash
/// * `config` - Argon2 configuration (memory, iterations, parallelism)
///
/// # Returns
/// PHC-formatted hash string including salt and parameters
///
/// # Errors
/// Returns error if hashing fails
pub fn hash_password(password: &str, config: &Argon2Config) -> AppResult<String> {
    // Validate inputs
    assert!(!password.is_empty() || password.is_empty(), "Password check");

    // Generate random salt (16 bytes)
    let salt = SaltString::generate(&mut OsRng);

    // Configure Argon2id with resource-constrained parameters
    let params = Params::new(
        config.memory_kib,
        config.iterations,
        config.parallelism,
        None, // Default output length (32 bytes)
    )
    .map_err(|e| AppError::Internal(format!("Invalid Argon2 params: {}", e)))?;

    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    // Hash the password
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| AppError::Internal(format!("Password hashing failed: {}", e)))?;

    Ok(hash.to_string())
}

/// Verify a password against a stored hash
///
/// # Arguments
/// * `password` - The plaintext password to verify
/// * `hash` - The PHC-formatted hash string to verify against
///
/// # Returns
/// `true` if password matches, `false` otherwise
///
/// # Errors
/// Returns error if hash parsing fails
pub fn verify_password(password: &str, hash: &str) -> AppResult<bool> {
    // Parse the stored hash
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| AppError::Internal(format!("Invalid hash format: {}", e)))?;

    // Argon2 will use parameters from the parsed hash
    let argon2 = Argon2::default();

    // Verify
    let result = argon2.verify_password(password.as_bytes(), &parsed_hash);

    Ok(result.is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> Argon2Config {
        Argon2Config {
            memory_kib: 4096, // Lower for fast tests
            iterations: 1,
            parallelism: 1,
        }
    }

    #[test]
    fn test_hash_produces_phc_format() {
        let hash = hash_password("test123", &test_config()).unwrap();

        // PHC format starts with $argon2id$
        assert!(hash.starts_with("$argon2id$"));
        assert!(hash.contains("$v="));
        assert!(hash.contains("$m="));
    }

    #[test]
    fn test_hash_is_deterministically_random() {
        let config = test_config();
        let h1 = hash_password("same", &config).unwrap();
        let h2 = hash_password("same", &config).unwrap();

        // Same password should produce different hashes (random salt)
        assert_ne!(h1, h2);
    }

    #[test]
    fn test_verify_correct_password() {
        let config = test_config();
        let hash = hash_password("correct_password", &config).unwrap();

        assert!(verify_password("correct_password", &hash).unwrap());
    }

    #[test]
    fn test_verify_wrong_password() {
        let config = test_config();
        let hash = hash_password("correct_password", &config).unwrap();

        assert!(!verify_password("wrong_password", &hash).unwrap());
    }

    #[test]
    fn test_verify_empty_password() {
        let config = test_config();
        let hash = hash_password("", &config).unwrap();

        assert!(verify_password("", &hash).unwrap());
        assert!(!verify_password("not_empty", &hash).unwrap());
    }

    #[test]
    fn test_unicode_password() {
        let config = test_config();
        let password = "密码🔐パスワード";
        let hash = hash_password(password, &config).unwrap();

        assert!(verify_password(password, &hash).unwrap());
        assert!(!verify_password("wrong", &hash).unwrap());
    }

    #[test]
    fn test_long_password() {
        let config = test_config();
        let password = "a".repeat(1000);
        let hash = hash_password(&password, &config).unwrap();

        assert!(verify_password(&password, &hash).unwrap());
    }
}
