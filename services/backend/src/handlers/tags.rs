//! Tag management handlers
//!
//! Create, list, and delete tags.

use actix_web::{delete, get, post, web, HttpRequest, HttpResponse};
use serde_json::json;
use validator::Validate;

use crate::db::Database;
use crate::error::{AppError, AppResult, AuthError};
use crate::middleware::audit::log_audit;
use crate::models::audit::{AuditAction, AuditResourceType};
use crate::models::tag::CreateTag;
use crate::security::jwt::Claims;

/// List all tags
#[get("/api/tags")]
pub async fn list_tags(db: web::Data<Database>) -> AppResult<HttpResponse> {
    let tags = db.list_tags()?;
    Ok(HttpResponse::Ok().json(tags))
}

/// Create new tag
#[post("/api/tags")]
pub async fn create_tag(
    req: HttpRequest,
    db: web::Data<Database>,
    body: web::Json<CreateTag>,
    claims: web::ReqData<Claims>,
) -> AppResult<HttpResponse> {
    // Check permissions
    if !claims.role.can_manage_tags() {
        return Err(AppError::Auth(AuthError::Forbidden));
    }

    // Validate input
    body.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    // Create tag
    let tag = db.create_tag(&body.name)?;

    // Log create action
    log_audit(
        &db,
        Some(&claims.sub),
        AuditAction::Create,
        AuditResourceType::Tag,
        Some(&tag.id),
        Some(json!({"name": tag.name}).to_string()),
        &req,
    );

    Ok(HttpResponse::Created().json(tag))
}

/// Delete tag
#[delete("/api/tags/{id}")]
pub async fn delete_tag(
    req: HttpRequest,
    db: web::Data<Database>,
    path: web::Path<String>,
    claims: web::ReqData<Claims>,
) -> AppResult<HttpResponse> {
    let tag_id = path.into_inner();

    // Check permissions (only admin)
    if !claims.role.can_manage_users() {
        return Err(AppError::Auth(AuthError::Forbidden));
    }

    // Delete tag
    db.delete_tag(&tag_id)?;

    // Log delete action
    log_audit(
        &db,
        Some(&claims.sub),
        AuditAction::Delete,
        AuditResourceType::Tag,
        Some(&tag_id),
        None,
        &req,
    );

    Ok(HttpResponse::Ok().json(json!({
        "message": "Tag deleted successfully"
    })))
}
