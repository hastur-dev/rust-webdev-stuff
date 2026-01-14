//! Article CRUD handlers
//!
//! Create, read, update, delete operations for articles.

use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use serde::Deserialize;
use serde_json::json;
use validator::Validate;

use crate::config::Config;
use crate::db::Database;
use crate::error::{AppError, AppResult, AuthError};
use crate::middleware::audit::log_audit;
use crate::models::article::{CreateArticle, UpdateArticle};
use crate::models::audit::{AuditAction, AuditResourceType};
use crate::security::jwt::Claims;

/// Query parameters for list endpoints
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

/// List all articles (admin/editor view)
#[get("/api/articles")]
pub async fn list_articles(
    req: HttpRequest,
    db: web::Data<Database>,
    config: web::Data<Config>,
    query: web::Query<ListQuery>,
    claims: web::ReqData<Claims>,
) -> AppResult<HttpResponse> {
    // Limit max results
    let limit = query.limit.min(100);

    let articles = if claims.role.can_edit_articles() {
        // Editors and admins can see all articles
        db.list_articles(query.offset, limit, &config.security)?
    } else {
        // Viewers can only see published
        db.list_published_articles(query.offset, limit, &config.security)?
    };

    // Log read action
    log_audit(
        &db,
        Some(&claims.sub),
        AuditAction::Read,
        AuditResourceType::Article,
        None,
        Some(json!({"offset": query.offset, "limit": limit}).to_string()),
        &req,
    );

    Ok(HttpResponse::Ok().json(articles))
}

/// List published articles (public view)
#[get("/api/articles/published")]
pub async fn list_published(
    db: web::Data<Database>,
    config: web::Data<Config>,
    query: web::Query<ListQuery>,
) -> AppResult<HttpResponse> {
    let limit = query.limit.min(100);
    let articles = db.list_published_articles(query.offset, limit, &config.security)?;

    Ok(HttpResponse::Ok().json(articles))
}

/// Get single article
#[get("/api/articles/{id}")]
pub async fn get_article(
    req: HttpRequest,
    db: web::Data<Database>,
    config: web::Data<Config>,
    path: web::Path<String>,
    claims: web::ReqData<Claims>,
) -> AppResult<HttpResponse> {
    let article_id = path.into_inner();

    let article = db
        .get_article(&article_id, &config.security)?
        .ok_or_else(|| AppError::NotFound("Article not found".to_string()))?;

    // Check if user can view unpublished articles
    if !article.is_published && !claims.role.can_edit_articles() {
        return Err(AppError::NotFound("Article not found".to_string()));
    }

    // Get tags
    let tags = db.get_article_tags(&article_id)?;

    // Log read action
    log_audit(
        &db,
        Some(&claims.sub),
        AuditAction::Read,
        AuditResourceType::Article,
        Some(&article_id),
        None,
        &req,
    );

    Ok(HttpResponse::Ok().json(json!({
        "article": article,
        "tags": tags
    })))
}

/// Create new article
#[post("/api/articles")]
pub async fn create_article(
    req: HttpRequest,
    db: web::Data<Database>,
    config: web::Data<Config>,
    body: web::Json<CreateArticle>,
    claims: web::ReqData<Claims>,
) -> AppResult<HttpResponse> {
    // Check permissions
    if !claims.role.can_edit_articles() {
        return Err(AppError::Auth(AuthError::Forbidden));
    }

    // Validate input
    body.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    // Create article
    let article = db.create_article(&body, &claims.sub, &config.security)?;

    // Log create action
    log_audit(
        &db,
        Some(&claims.sub),
        AuditAction::Create,
        AuditResourceType::Article,
        Some(&article.id),
        Some(json!({"title": article.title}).to_string()),
        &req,
    );

    Ok(HttpResponse::Created().json(article))
}

/// Update article
#[put("/api/articles/{id}")]
pub async fn update_article(
    req: HttpRequest,
    db: web::Data<Database>,
    config: web::Data<Config>,
    path: web::Path<String>,
    body: web::Json<UpdateArticle>,
    claims: web::ReqData<Claims>,
) -> AppResult<HttpResponse> {
    let article_id = path.into_inner();

    // Check permissions
    if !claims.role.can_edit_articles() {
        return Err(AppError::Auth(AuthError::Forbidden));
    }

    // Validate input
    body.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    // Check article exists and user owns it or is admin
    let existing = db
        .get_article(&article_id, &config.security)?
        .ok_or_else(|| AppError::NotFound("Article not found".to_string()))?;

    // Only author or admin can edit
    if existing.author_id != claims.sub && !claims.role.can_manage_users() {
        return Err(AppError::Auth(AuthError::Forbidden));
    }

    // Update article
    let article = db.update_article(&article_id, &body, &config.security)?;

    // Log update action
    log_audit(
        &db,
        Some(&claims.sub),
        AuditAction::Update,
        AuditResourceType::Article,
        Some(&article_id),
        Some(json!({"changed_fields": {
            "title": body.title.is_some(),
            "content": body.content.is_some(),
            "is_published": body.is_published.is_some()
        }}).to_string()),
        &req,
    );

    Ok(HttpResponse::Ok().json(article))
}

/// Delete article
#[delete("/api/articles/{id}")]
pub async fn delete_article(
    req: HttpRequest,
    db: web::Data<Database>,
    config: web::Data<Config>,
    path: web::Path<String>,
    claims: web::ReqData<Claims>,
) -> AppResult<HttpResponse> {
    let article_id = path.into_inner();

    // Check permissions
    if !claims.role.can_delete_articles() {
        return Err(AppError::Auth(AuthError::Forbidden));
    }

    // Verify article exists
    let article = db
        .get_article(&article_id, &config.security)?
        .ok_or_else(|| AppError::NotFound("Article not found".to_string()))?;

    // Delete article
    db.delete_article(&article_id)?;

    // Log delete action
    log_audit(
        &db,
        Some(&claims.sub),
        AuditAction::Delete,
        AuditResourceType::Article,
        Some(&article_id),
        Some(json!({"title": article.title}).to_string()),
        &req,
    );

    Ok(HttpResponse::Ok().json(json!({
        "message": "Article deleted successfully"
    })))
}

/// Add tags to article
#[post("/api/articles/{id}/tags")]
pub async fn add_tags(
    req: HttpRequest,
    db: web::Data<Database>,
    config: web::Data<Config>,
    path: web::Path<String>,
    body: web::Json<crate::models::tag::AddTagsRequest>,
    claims: web::ReqData<Claims>,
) -> AppResult<HttpResponse> {
    let article_id = path.into_inner();

    // Check permissions
    if !claims.role.can_edit_articles() {
        return Err(AppError::Auth(AuthError::Forbidden));
    }

    // Verify article exists
    db.get_article(&article_id, &config.security)?
        .ok_or_else(|| AppError::NotFound("Article not found".to_string()))?;

    // Add tags to the article
    for tag_id in &body.tag_ids {
        let _ = db.add_tag_to_article(&article_id, tag_id);
    }

    // Log tag addition
    log_audit(
        &db,
        Some(&claims.sub),
        AuditAction::Update,
        AuditResourceType::Article,
        Some(&article_id),
        Some(json!({"tags_added": body.tag_ids}).to_string()),
        &req,
    );

    let tags = db.get_article_tags(&article_id)?;

    Ok(HttpResponse::Ok().json(tags))
}
