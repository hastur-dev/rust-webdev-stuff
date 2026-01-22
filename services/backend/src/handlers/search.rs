//! Search handlers
//!
//! Full-text search for articles.

use actix_web::{get, web, HttpRequest, HttpResponse};
use serde::Deserialize;
use serde_json::json;

use crate::config::Config;
use crate::db::Database;
use crate::error::AppResult;
use crate::middleware::audit::log_audit;
use crate::models::audit::{AuditAction, AuditResourceType};
use crate::security::jwt::Claims;

/// Search query parameters
#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: String,
    #[serde(default)]
    pub offset: i64,
    #[serde(default = "default_limit")]
    pub limit: i64,
}

fn default_limit() -> i64 {
    20
}

/// Search articles (authenticated)
#[get("/api/search")]
pub async fn search(
    req: HttpRequest,
    db: web::Data<Database>,
    config: web::Data<Config>,
    query: web::Query<SearchQuery>,
    claims: web::ReqData<Claims>,
) -> AppResult<HttpResponse> {
    let limit = query.limit.min(100);

    let results = if claims.role.can_edit_articles() {
        db.search_articles(&query.q, query.offset, limit, &config.security)?
    } else {
        db.search_published_articles(&query.q, query.offset, limit, &config.security)?
    };

    log_audit(
        &db,
        Some(&claims.sub),
        AuditAction::Search,
        AuditResourceType::Article,
        None,
        Some(json!({"query": query.q, "results": results.len()}).to_string()),
        &req,
    );

    Ok(HttpResponse::Ok().json(json!({
        "query": query.q,
        "results": results,
        "count": results.len()
    })))
}

/// Search published articles (public)
#[get("/api/search/public")]
pub async fn search_public(
    db: web::Data<Database>,
    config: web::Data<Config>,
    query: web::Query<SearchQuery>,
) -> AppResult<HttpResponse> {
    let limit = query.limit.min(100);
    let results = db.search_published_articles(&query.q, query.offset, limit, &config.security)?;

    Ok(HttpResponse::Ok().json(json!({
        "query": query.q,
        "results": results,
        "count": results.len()
    })))
}
