//! Admin handlers
//!
//! User management for administrators.

use actix_web::{get, put, web, HttpRequest, HttpResponse};
use serde::Deserialize;
use serde_json::json;
use validator::Validate;

use crate::db::Database;
use crate::error::{AppError, AppResult, AuthError};
use crate::middleware::audit::log_audit;
use crate::models::audit::{AuditAction, AuditResourceType};
use crate::models::user::{UpdateUser, UserInfo};
use crate::security::jwt::Claims;

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    #[serde(default)]
    pub offset: i64,
    #[serde(default = "default_limit")]
    pub limit: i64,
}

fn default_limit() -> i64 {
    20
}

/// List all users (admin only)
#[get("/api/admin/users")]
pub async fn list_users(
    req: HttpRequest,
    db: web::Data<Database>,
    query: web::Query<ListQuery>,
    claims: web::ReqData<Claims>,
) -> AppResult<HttpResponse> {
    if !claims.role.can_manage_users() {
        return Err(AppError::Auth(AuthError::Forbidden));
    }

    let limit = query.limit.min(100);
    let users: Vec<UserInfo> = db
        .list_users(query.offset, limit)?
        .into_iter()
        .map(UserInfo::from)
        .collect();

    let total = db.count_users()?;

    log_audit(
        &db,
        Some(&claims.sub),
        AuditAction::Read,
        AuditResourceType::User,
        None,
        Some(json!({"action": "list_users"}).to_string()),
        &req,
    );

    Ok(HttpResponse::Ok().json(json!({
        "users": users,
        "total": total,
        "offset": query.offset,
        "limit": limit
    })))
}

/// Get single user (admin only)
#[get("/api/admin/users/{id}")]
pub async fn get_user(
    req: HttpRequest,
    db: web::Data<Database>,
    path: web::Path<String>,
    claims: web::ReqData<Claims>,
) -> AppResult<HttpResponse> {
    if !claims.role.can_manage_users() {
        return Err(AppError::Auth(AuthError::Forbidden));
    }

    let user_id = path.into_inner();
    let user = db
        .get_user_by_id(&user_id)?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    log_audit(
        &db,
        Some(&claims.sub),
        AuditAction::Read,
        AuditResourceType::User,
        Some(&user_id),
        None,
        &req,
    );

    Ok(HttpResponse::Ok().json(UserInfo::from(user)))
}

/// Update user (admin only)
#[put("/api/admin/users/{id}")]
pub async fn update_user(
    req: HttpRequest,
    db: web::Data<Database>,
    path: web::Path<String>,
    body: web::Json<UpdateUser>,
    claims: web::ReqData<Claims>,
) -> AppResult<HttpResponse> {
    if !claims.role.can_manage_users() {
        return Err(AppError::Auth(AuthError::Forbidden));
    }

    let user_id = path.into_inner();
    let target_user = db
        .get_user_by_id(&user_id)?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    if let Some(ref new_role) = body.role {
        if !claims.role.can_modify_role(&target_user.role) {
            return Err(AppError::Auth(AuthError::Forbidden));
        }
        if !claims.role.can_modify_role(new_role) {
            return Err(AppError::Auth(AuthError::Forbidden));
        }
    }

    body.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let updated = db.update_user(&user_id, &body)?;

    if let Some(is_active) = body.is_active {
        let action = if is_active {
            AuditAction::UserActivated
        } else {
            AuditAction::UserDeactivated
        };
        log_audit(
            &db,
            Some(&claims.sub),
            action,
            AuditResourceType::User,
            Some(&user_id),
            Some(json!({"target_username": updated.username}).to_string()),
            &req,
        );
    }

    if body.role.is_some() {
        log_audit(
            &db,
            Some(&claims.sub),
            AuditAction::RoleChanged,
            AuditResourceType::User,
            Some(&user_id),
            Some(json!({"target_username": updated.username, "new_role": updated.role}).to_string()),
            &req,
        );
    }

    Ok(HttpResponse::Ok().json(UserInfo::from(updated)))
}
