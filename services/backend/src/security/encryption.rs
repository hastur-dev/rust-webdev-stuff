//! At-rest encryption using AES-256-GCM
//!
//! Provides encryption for sensitive content stored in the database.
//! Uses unique nonces per encryption operation.

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use hkdf::Hkdf;
use rand::RngCore;
use sha2::Sha256;

use crate::config::SecurityConfig;
use crate::error::{AppError, AppResult};

/// Nonce size for AES-GCM (96 bits / 12 bytes)
const NONCE_SIZE: usize = 12;

/// Key size for AES-256 (256 bits / 32 bytes)
const KEY_SIZE: usize = 32;

/// Encrypt plaintext content using AES-256-GCM
///
/// # Arguments
/// * `plaintext` - The content to encrypt
/// * `config` - Security configuration with encryption key
///
/// # Returns
/// Tuple of (ciphertext, nonce)
///
/// # Errors
/// Returns error if encryption fails
pub fn encrypt(plaintext: &[u8], config: &SecurityConfig) -> AppResult<(Vec<u8>, Vec<u8>)> {
    // Preconditions
    assert!(
        config.encryption_key.len() >= KEY_SIZE,
        "Encryption key must be at least 32 bytes"
    );

    // Derive key using HKDF
    let key = derive_key(&config.encryption_key)?;

    // Generate random nonce
    let mut nonce_bytes = [0u8; NONCE_SIZE];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    // Create cipher
    let cipher = Aes256Gcm::new_from_slice(&key)
        .map_err(|e| AppError::Encryption(format!("Failed to create cipher: {}", e)))?;

    // Encrypt
    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|_| AppError::Encryption("Encryption failed".to_string()))?;

    // Postconditions
    assert!(!ciphertext.is_empty() || plaintext.is_empty(), "Ciphertext should not be empty for non-empty plaintext");

    Ok((ciphertext, nonce_bytes.to_vec()))
}

/// Decrypt ciphertext using AES-256-GCM
///
/// # Arguments
/// * `ciphertext` - The encrypted content
/// * `nonce` - The nonce used during encryption
/// * `config` - Security configuration with encryption key
///
/// # Returns
/// Decrypted plaintext
///
/// # Errors
/// Returns error if decryption fails (wrong key, tampered data, etc.)
pub fn decrypt(ciphertext: &[u8], nonce: &[u8], config: &SecurityConfig) -> AppResult<Vec<u8>> {
    // Preconditions
    assert!(
        nonce.len() == NONCE_SIZE,
        "Nonce must be exactly {} bytes",
        NONCE_SIZE
    );
    assert!(
        config.encryption_key.len() >= KEY_SIZE,
        "Encryption key must be at least 32 bytes"
    );

    if nonce.len() != NONCE_SIZE {
        return Err(AppError::Encryption(format!(
            "Invalid nonce size: expected {}, got {}",
            NONCE_SIZE,
            nonce.len()
        )));
    }

    // Derive key using HKDF
    let key = derive_key(&config.encryption_key)?;

    let nonce = Nonce::from_slice(nonce);

    // Create cipher
    let cipher = Aes256Gcm::new_from_slice(&key)
        .map_err(|e| AppError::Encryption(format!("Failed to create cipher: {}", e)))?;

    // Decrypt
    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| AppError::Encryption("Decryption failed - data may be tampered".to_string()))?;

    Ok(plaintext)
}

/// Derive a 256-bit key from the master key using HKDF-SHA256
fn derive_key(master_key: &str) -> AppResult<[u8; KEY_SIZE]> {
    let hk = Hkdf::<Sha256>::new(None, master_key.as_bytes());

    let mut key = [0u8; KEY_SIZE];
    hk.expand(b"knowledge-vault-aes-key", &mut key)
        .map_err(|e| AppError::Encryption(format!("Key derivation failed: {}", e)))?;

    Ok(key)
}

/// Encrypt a string and return base64-encoded result with nonce
///
/// Convenience function for string content
pub fn encrypt_string(plaintext: &str, config: &SecurityConfig) -> AppResult<(Vec<u8>, Vec<u8>)> {
    encrypt(plaintext.as_bytes(), config)
}

/// Decrypt to a string
///
/// Convenience function for string content
pub fn decrypt_string(
    ciphertext: &[u8],
    nonce: &[u8],
    config: &SecurityConfig,
) -> AppResult<String> {
    let plaintext = decrypt(ciphertext, nonce, config)?;

    String::from_utf8(plaintext)
        .map_err(|e| AppError::Encryption(format!("Decrypted data is not valid UTF-8: {}", e)))
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
    fn test_encrypt_decrypt_roundtrip() {
        let config = test_config();
        let plaintext = b"Hello, World! This is secret data.";

        let (ciphertext, nonce) = encrypt(plaintext, &config).unwrap();
        let decrypted = decrypt(&ciphertext, &nonce, &config).unwrap();

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_string_roundtrip() {
        let config = test_config();
        let plaintext = "Unicode test: 日本語 🔐 Ελληνικά";

        let (ciphertext, nonce) = encrypt_string(plaintext, &config).unwrap();
        let decrypted = decrypt_string(&ciphertext, &nonce, &config).unwrap();

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_unique_nonces() {
        let config = test_config();
        let plaintext = b"same content";

        let (_, nonce1) = encrypt(plaintext, &config).unwrap();
        let (_, nonce2) = encrypt(plaintext, &config).unwrap();

        assert_ne!(nonce1, nonce2, "Each encryption should use unique nonce");
    }

    #[test]
    fn test_different_ciphertext() {
        let config = test_config();
        let plaintext = b"same content";

        let (ct1, _) = encrypt(plaintext, &config).unwrap();
        let (ct2, _) = encrypt(plaintext, &config).unwrap();

        assert_ne!(ct1, ct2, "Same plaintext should produce different ciphertext");
    }

    #[test]
    fn test_wrong_nonce_fails() {
        let config = test_config();
        let plaintext = b"secret";

        let (ciphertext, _) = encrypt(plaintext, &config).unwrap();
        let wrong_nonce = vec![0u8; NONCE_SIZE];

        let result = decrypt(&ciphertext, &wrong_nonce, &config);
        assert!(result.is_err());
    }

    #[test]
    fn test_tampered_ciphertext_fails() {
        let config = test_config();
        let plaintext = b"secret";

        let (mut ciphertext, nonce) = encrypt(plaintext, &config).unwrap();

        // Tamper with ciphertext
        if !ciphertext.is_empty() {
            ciphertext[0] ^= 0xFF;
        }

        let result = decrypt(&ciphertext, &nonce, &config);
        assert!(result.is_err());
    }

    #[test]
    fn test_wrong_key_fails() {
        let config = test_config();
        let plaintext = b"secret";

        let (ciphertext, nonce) = encrypt(plaintext, &config).unwrap();

        let mut wrong_config = config;
        wrong_config.encryption_key = "different_key_that_is_32_bytes_".to_string();

        let result = decrypt(&ciphertext, &nonce, &wrong_config);
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_plaintext() {
        let config = test_config();
        let plaintext = b"";

        let (ciphertext, nonce) = encrypt(plaintext, &config).unwrap();
        let decrypted = decrypt(&ciphertext, &nonce, &config).unwrap();

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_large_plaintext() {
        let config = test_config();
        let plaintext: Vec<u8> = (0..100_000).map(|i| (i % 256) as u8).collect();

        let (ciphertext, nonce) = encrypt(&plaintext, &config).unwrap();
        let decrypted = decrypt(&ciphertext, &nonce, &config).unwrap();

        assert_eq!(decrypted, plaintext);
    }
}
