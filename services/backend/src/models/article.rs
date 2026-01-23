//! Article model and related types
//!
//! Defines article entity with encrypted content support.

use serde::{Deserialize, Serialize};
use validator::Validate;

/// Article entity (decrypted view)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    pub id: String,
    pub title: String,
    pub content: String, // Decrypted content
    pub author_id: String,
    pub is_published: bool,
    pub created_at: String,
    pub updated_at: String,
}

/// Article with encrypted content (storage format)
#[derive(Debug, Clone)]
pub struct ArticleRaw {
    pub id: String,
    pub title: String,
    pub content_encrypted: Vec<u8>,
    pub content_nonce: Vec<u8>,
    pub author_id: String,
    pub is_published: bool,
    pub created_at: String,
    pub updated_at: String,
}

/// DTO for creating a new article
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateArticle {
    #[validate(length(min = 1, max = 500, message = "Title must be 1-500 characters"))]
    pub title: String,
    #[validate(length(max = 1_000_000, message = "Content too large"))]
    pub content: String,
    #[serde(default)]
    pub tags: Vec<String>, // Tag IDs
    #[serde(default)]
    pub is_published: bool,
}

/// DTO for updating an article
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct UpdateArticle {
    #[validate(length(min = 1, max = 500))]
    pub title: Option<String>,
    #[validate(length(max = 1_000_000))]
    pub content: Option<String>,
    pub is_published: Option<bool>,
}

/// Article list item (summary without full content)
#[derive(Debug, Clone, Serialize)]
pub struct ArticleSummary {
    pub id: String,
    pub title: String,
    pub author_id: String,
    pub author_username: String,
    pub is_published: bool,
    pub created_at: String,
    pub updated_at: String,
    pub tags: Vec<String>, // Tag names
}

/// Article with author info
#[derive(Debug, Clone, Serialize)]
pub struct ArticleWithAuthor {
    pub id: String,
    pub title: String,
    pub content: String,
    pub author: ArticleAuthor,
    pub is_published: bool,
    pub created_at: String,
    pub updated_at: String,
    pub tags: Vec<TagInfo>,
}

/// Minimal author info for article display
#[derive(Debug, Clone, Serialize)]
pub struct ArticleAuthor {
    pub id: String,
    pub username: String,
}

/// Minimal tag info for article display
#[derive(Debug, Clone, Serialize)]
pub struct TagInfo {
    pub id: String,
    pub name: String,
}

/// Search result item
#[derive(Debug, Clone, Serialize)]
pub struct SearchResult {
    pub id: String,
    pub title: String,
    pub snippet: String, // Highlighted excerpt
    pub author_username: String,
    pub created_at: String,
    pub score: f64, // Relevance score
}

/// Pagination info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    pub offset: i64,
    pub limit: i64,
    pub total: i64,
}

/// Paginated response wrapper
#[derive(Debug, Clone, Serialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub pagination: Pagination,
}

impl<T> PaginatedResponse<T> {
    pub fn new(items: Vec<T>, offset: i64, limit: i64, total: i64) -> Self {
        PaginatedResponse {
            items,
            pagination: Pagination {
                offset,
                limit,
                total,
            },
        }
    }
}
