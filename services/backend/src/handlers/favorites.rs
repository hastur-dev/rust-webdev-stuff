//! Favorite management handlers
//!
//! Add, remove, and list user favorites.

use actix_web::{delete, get, post, web, HttpRequest, HttpResponse};
use serde_json::json;

use crate::config::Config;
use crate::db::Database;
use crate::error::{AppError, AppResult};
use crate::middleware::audit::log_audit;
use crate::models::audit::{AuditAction, AuditResourceType};
use crate::security::jwt::Claims;

/// List user's favorites
#[get("/api/favorites")]
pub async fn list_favorites(
    _req: HttpRequest,
    db: web::Data<Database>,
    config: web::Data<Config>,
    claims: web::ReqData<Claims>,
) -> AppResult<HttpResponse> {
    let favorites = db.get_user_favorites(&claims.sub)?;

    // Get article details for each favorite
    let mut detailed_favorites = Vec::with_capacity(favorites.len());
    for fav in favorites {
        if let Some(article) = db.get_article(&fav.article_id, &config.security)? {
            detailed_favorites.push(json!({
                "article_id": fav.article_id,
                "article_title": article.title,
                "favorited_at": fav.created_at
            }));
        }
    }

    Ok(HttpResponse::Ok().json(detailed_favorites))
}

/// Add article to favorites
#[post("/api/favorites/{article_id}")]
pub async fn add_favorite(
    req: HttpRequest,
    db: web::Data<Database>,
    config: web::Data<Config>,
    path: web::Path<String>,
    claims: web::ReqData<Claims>,
) -> AppResult<HttpResponse> {
    let article_id = path.into_inner();

    // Verify article exists
    db.get_article(&article_id, &config.security)?
        .ok_or_else(|| AppError::NotFound("Article not found".to_string()))?;

    // Add favorite
    let favorite = db.add_favorite(&claims.sub, &article_id)?;

    // Log favorite action
    log_audit(
        &db,
        Some(&claims.sub),
        AuditAction::Favorite,
        AuditResourceType::Favorite,
        Some(&article_id),
        None,
        &req,
    );

    Ok(HttpResponse::Created().json(favorite))
}

/// Remove article from favorites
#[delete("/api/favorites/{article_id}")]
pub async fn remove_favorite(
    req: HttpRequest,
    db: web::Data<Database>,
    path: web::Path<String>,
    claims: web::ReqData<Claims>,
) -> AppResult<HttpResponse> {
    let article_id = path.into_inner();

    // Remove favorite
    db.remove_favorite(&claims.sub, &article_id)?;

    // Log unfavorite action
    log_audit(
        &db,
        Some(&claims.sub),
        AuditAction::Unfavorite,
        AuditResourceType::Favorite,
        Some(&article_id),
        None,
        &req,
    );

    Ok(HttpResponse::Ok().json(json!({
        "message": "Removed from favorites"
    })))
}

/// Check if article is favorited
#[get("/api/favorites/{article_id}/status")]
pub async fn favorite_status(
    db: web::Data<Database>,
    path: web::Path<String>,
    claims: web::ReqData<Claims>,
) -> AppResult<HttpResponse> {
    let article_id = path.into_inner();

    let is_favorited = db.is_favorited(&claims.sub, &article_id)?;

    Ok(HttpResponse::Ok().json(json!({
        "is_favorited": is_favorited
    })))
}
