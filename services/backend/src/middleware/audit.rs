//! Audit logging middleware
//!
//! Logs all API requests to the audit log.

use actix_web::HttpRequest;

use crate::db::Database;
use crate::models::audit::{AuditAction, AuditResourceType, CreateAuditEntry};

/// Log an audit entry
pub fn log_audit(
    db: &Database,
    user_id: Option<&str>,
    action: AuditAction,
    resource_type: AuditResourceType,
    resource_id: Option<&str>,
    details: Option<String>,
    req: &HttpRequest,
) {
    let ip_address = req
        .connection_info()
        .realip_remote_addr()
        .map(|s| s.to_string());

    let user_agent = req
        .headers()
        .get("User-Agent")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    let entry = CreateAuditEntry {
        user_id: user_id.map(|s| s.to_string()),
        action,
        resource_type,
        resource_id: resource_id.map(|s| s.to_string()),
        details,
        ip_address,
        user_agent,
    };

    // Log errors but don't fail the request
    if let Err(e) = db.create_audit_entry(&entry) {
        tracing::error!("Failed to create audit entry: {}", e);
    }
}

/// Log request start (for request-level auditing)
pub fn log_request_start(
    db: &Database,
    user_id: Option<&str>,
    method: &str,
    path: &str,
    req: &HttpRequest,
) {
    let details = serde_json::json!({
        "method": method,
        "path": path
    })
    .to_string();

    log_audit(
        db,
        user_id,
        AuditAction::Read,
        AuditResourceType::System,
        None,
        Some(details),
        req,
    );
}
