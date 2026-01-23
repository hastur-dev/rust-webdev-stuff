//! Knowledge Vault Backend Library
//!
//! Secure knowledge management system with encryption at rest.

pub mod config;
pub mod db;
pub mod error;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod security;

pub use config::Config;
pub use db::Database;
pub use error::{AppError, AppResult};
