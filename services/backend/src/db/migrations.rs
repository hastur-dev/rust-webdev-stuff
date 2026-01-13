//! Database migrations
//!
//! Handles schema versioning and migrations.

use rusqlite::Connection;

use crate::error::{AppError, AppResult};

/// Current schema version
const SCHEMA_VERSION: i32 = 1;

/// Run all pending migrations
pub fn run_migrations(conn: &Connection) -> AppResult<()> {
    // Create migrations table if not exists
    conn.execute(
        "CREATE TABLE IF NOT EXISTS schema_version (
            version INTEGER PRIMARY KEY,
            applied_at TEXT NOT NULL
        )",
        [],
    )?;

    // Get current version
    let current_version: i32 = conn
        .query_row(
            "SELECT COALESCE(MAX(version), 0) FROM schema_version",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0);

    // Apply migrations
    if current_version < 1 {
        migrate_v1(conn)?;
    }

    Ok(())
}

/// Migration to version 1: Initial schema
fn migrate_v1(conn: &Connection) -> AppResult<()> {
    // Apply pragmas
    conn.execute_batch(super::schema::PRAGMAS)?;

    // Create tables
    conn.execute_batch(super::schema::CREATE_TABLES)?;

    // Create indexes
    conn.execute_batch(super::schema::CREATE_INDEXES)?;

    // Record migration
    conn.execute(
        "INSERT INTO schema_version (version, applied_at) VALUES (?, datetime('now'))",
        [SCHEMA_VERSION],
    )?;

    Ok(())
}

/// Get current schema version (may be used for future migrations)
#[allow(dead_code)]
pub fn get_schema_version(conn: &Connection) -> AppResult<i32> {
    let version: i32 = conn
        .query_row(
            "SELECT COALESCE(MAX(version), 0) FROM schema_version",
            [],
            |row| row.get(0),
        )
        .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(version)
}
