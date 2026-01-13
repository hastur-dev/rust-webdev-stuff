//! Configuration loading and validation for Knowledge Vault
//!
//! Loads configuration from config.yaml with environment variable overrides.

use serde::Deserialize;
use std::fs;
use std::path::Path;

/// Main application configuration
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub security: SecurityConfig,
    pub logging: LoggingConfig,
    pub cors: CorsConfig,
}

/// Server configuration
#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
}

/// Database configuration
#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub path: String,
    pub max_connections: u32,
}

/// Security configuration
#[derive(Debug, Clone, Deserialize)]
pub struct SecurityConfig {
    pub jwt_secret: String,
    pub jwt_expiry_hours: i64,
    pub encryption_key: String,
    pub argon2: Argon2Config,
}

/// Argon2 password hashing configuration
#[derive(Debug, Clone, Deserialize)]
pub struct Argon2Config {
    pub memory_kib: u32,
    pub iterations: u32,
    pub parallelism: u32,
}

/// Logging configuration
#[derive(Debug, Clone, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub audit_retention_days: u32,
}

/// CORS configuration
#[derive(Debug, Clone, Deserialize)]
pub struct CorsConfig {
    pub allowed_origins: Vec<String>,
}

impl Config {
    /// Load configuration from file with validation
    ///
    /// # Errors
    /// Returns error if file cannot be read or parsed, or if validation fails
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let content = fs::read_to_string(path.as_ref())
            .map_err(|e| ConfigError::FileRead(e.to_string()))?;

        let config: Config = serde_yaml::from_str(&content)
            .map_err(|e| ConfigError::Parse(e.to_string()))?;

        config.validate()?;

        Ok(config)
    }

    /// Load configuration from string (useful for testing)
    pub fn from_str(content: &str) -> Result<Self, ConfigError> {
        let config: Config = serde_yaml::from_str(content)
            .map_err(|e| ConfigError::Parse(e.to_string()))?;

        config.validate()?;

        Ok(config)
    }

    /// Validate configuration values
    fn validate(&self) -> Result<(), ConfigError> {
        // Validate JWT secret length (minimum 32 bytes for HS256)
        assert!(
            self.security.jwt_secret.len() >= 32,
            "JWT secret must be at least 32 bytes"
        );
        if self.security.jwt_secret.len() < 32 {
            return Err(ConfigError::Validation(
                "JWT secret must be at least 32 bytes".to_string(),
            ));
        }

        // Validate encryption key length (32 bytes for AES-256)
        assert!(
            self.security.encryption_key.len() >= 32,
            "Encryption key must be at least 32 bytes"
        );
        if self.security.encryption_key.len() < 32 {
            return Err(ConfigError::Validation(
                "Encryption key must be at least 32 bytes".to_string(),
            ));
        }

        // Validate Argon2 parameters for memory constraints
        assert!(
            self.security.argon2.memory_kib <= 50_000,
            "Argon2 memory must be <= 50MB for resource constraints"
        );
        if self.security.argon2.memory_kib > 50_000 {
            return Err(ConfigError::Validation(
                "Argon2 memory exceeds 50MB limit".to_string(),
            ));
        }

        // Validate parallelism for single-thread constraint
        assert!(
            self.security.argon2.parallelism == 1,
            "Argon2 parallelism must be 1 for single-thread constraint"
        );
        if self.security.argon2.parallelism != 1 {
            return Err(ConfigError::Validation(
                "Argon2 parallelism must be 1 for single-thread mode".to_string(),
            ));
        }

        // Validate server workers
        assert!(
            self.server.workers == 1,
            "Server workers must be 1 for single-thread constraint"
        );
        if self.server.workers != 1 {
            return Err(ConfigError::Validation(
                "Server workers must be 1 for single-thread mode".to_string(),
            ));
        }

        // Validate port range
        assert!(self.server.port > 0, "Port must be positive");
        if self.server.port == 0 {
            return Err(ConfigError::Validation("Port must be positive".to_string()));
        }

        // Validate max connections for memory constraints
        assert!(
            self.database.max_connections <= 5,
            "Max connections must be <= 5 for memory constraints"
        );
        if self.database.max_connections > 5 {
            return Err(ConfigError::Validation(
                "Max connections exceeds limit for memory constraints".to_string(),
            ));
        }

        Ok(())
    }

    /// Create default configuration for testing
    #[cfg(test)]
    pub fn test_default() -> Self {
        Config {
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 8080,
                workers: 1,
            },
            database: DatabaseConfig {
                path: ":memory:".to_string(),
                max_connections: 1,
            },
            security: SecurityConfig {
                jwt_secret: "test_secret_key_must_be_32_bytes_long".to_string(),
                jwt_expiry_hours: 24,
                encryption_key: "test_encryption_key_32_bytes_xx".to_string(),
                argon2: Argon2Config {
                    memory_kib: 4096,
                    iterations: 1,
                    parallelism: 1,
                },
            },
            logging: LoggingConfig {
                level: "debug".to_string(),
                audit_retention_days: 90,
            },
            cors: CorsConfig {
                allowed_origins: vec!["http://localhost:5173".to_string()],
            },
        }
    }
}

/// Configuration errors
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    FileRead(String),

    #[error("Failed to parse config: {0}")]
    Parse(String),

    #[error("Config validation failed: {0}")]
    Validation(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_config() {
        let yaml = r#"
server:
  host: "127.0.0.1"
  port: 8080
  workers: 1

database:
  path: "./data/test.db"
  max_connections: 2

security:
  jwt_secret: "test_secret_key_must_be_32_bytes_long"
  jwt_expiry_hours: 24
  encryption_key: "test_encryption_key_32_bytes_xx"
  argon2:
    memory_kib: 19456
    iterations: 2
    parallelism: 1

logging:
  level: "info"
  audit_retention_days: 90

cors:
  allowed_origins:
    - "http://localhost:5173"
"#;

        let config = Config::from_str(yaml).expect("Valid config should parse");
        assert_eq!(config.server.port, 8080);
        assert_eq!(config.security.argon2.parallelism, 1);
    }

    #[test]
    fn test_invalid_jwt_secret() {
        let yaml = r#"
server:
  host: "127.0.0.1"
  port: 8080
  workers: 1

database:
  path: "./data/test.db"
  max_connections: 2

security:
  jwt_secret: "short"
  jwt_expiry_hours: 24
  encryption_key: "test_encryption_key_32_bytes_xx"
  argon2:
    memory_kib: 19456
    iterations: 2
    parallelism: 1

logging:
  level: "info"
  audit_retention_days: 90

cors:
  allowed_origins:
    - "http://localhost:5173"
"#;

        let result = Config::from_str(yaml);
        assert!(result.is_err());
    }
}
