//! Audit log handlers
//!
//! View audit logs (admin only).

use actix_web::{get, web, HttpRequest, HttpResponse};
use serde::Deserialize;
use serde_json::json;

use crate::db::Database;
use crate::error::{AppError, AppResult, AuthError};
use crate::middleware::audit::log_audit;
use crate::models::audit::{AuditAction, AuditResourceType};
use crate::security::jwt::Claims;

#[derive(Debug, Deserialize)]
pub struct AuditQuery {
    #[serde(default)]
    pub offset: i64,
    #[serde(default = "default_limit")]
    pub limit: i64,
    pub user_id: Option<String>,
    pub action: Option<String>,
    pub resource_type: Option<String>,
}

fn default_limit() -> i64 {
    50
}

/// List audit logs (admin only)
#[get("/api/admin/audit")]
pub async fn list_audit_logs(
    req: HttpRequest,
    db: web::Data<Database>,
    query: web::Query<AuditQuery>,
    claims: web::ReqData<Claims>,
) -> AppResult<HttpResponse> {
    if !claims.role.can_view_audit() {
        return Err(AppError::Auth(AuthError::Forbidden));
    }

    let limit = query.limit.min(200);
    let action = query.action.as_ref().and_then(|a| AuditAction::from_str(a));
    let resource_type = query
        .resource_type
        .as_ref()
        .and_then(|r| AuditResourceType::from_str(r));

    let entries = db.list_audit_entries(
        query.offset,
        limit,
        query.user_id.as_deref(),
        action,
        resource_type,
    )?;

    let total = db.count_audit_entries(query.user_id.as_deref(), action, resource_type)?;

    log_audit(
        &db,
        Some(&claims.sub),
        AuditAction::Read,
        AuditResourceType::System,
        None,
        Some(json!({"action": "view_audit_logs"}).to_string()),
        &req,
    );

    Ok(HttpResponse::Ok().json(json!({
        "entries": entries,
        "total": total,
        "offset": query.offset,
        "limit": limit
    })))
}

/// Get audit stats (admin only)
#[get("/api/admin/audit/stats")]
pub async fn audit_stats(
    db: web::Data<Database>,
    claims: web::ReqData<Claims>,
) -> AppResult<HttpResponse> {
    if !claims.role.can_view_audit() {
        return Err(AppError::Auth(AuthError::Forbidden));
    }

    let total = db.count_audit_entries(None, None, None)?;
    let logins = db.count_audit_entries(None, Some(AuditAction::Login), None)?;
    let failed_logins = db.count_audit_entries(None, Some(AuditAction::LoginFailed), None)?;

    Ok(HttpResponse::Ok().json(json!({
        "total_entries": total,
        "login_count": logins,
        "failed_login_count": failed_logins
    })))
}
