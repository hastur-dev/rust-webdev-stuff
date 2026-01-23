//! Favorite model and related types
//!
//! Defines favorite entity for user bookmarks.

use serde::{Deserialize, Serialize};

/// Favorite entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Favorite {
    pub user_id: String,
    pub article_id: String,
    pub created_at: String,
}

/// Favorite with article details
#[derive(Debug, Clone, Serialize)]
pub struct FavoriteWithArticle {
    pub article_id: String,
    pub article_title: String,
    pub author_username: String,
    pub favorited_at: String,
}

/// Response for favorite check
#[derive(Debug, Clone, Serialize)]
pub struct FavoriteStatus {
    pub is_favorited: bool,
}
