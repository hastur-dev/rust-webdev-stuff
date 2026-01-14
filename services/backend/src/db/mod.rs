//! Database module for Knowledge Vault
//!
//! Provides SQLite database access with connection pooling.

mod migrations;
mod pool;
mod schema;

pub use pool::Database;
