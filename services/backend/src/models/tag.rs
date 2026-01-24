//! Tag model and related types
//!
//! Defines tag entity for article categorization.

use serde::{Deserialize, Serialize};
use validator::Validate;

/// Tag entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub created_at: String,
}

/// DTO for creating a new tag
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateTag {
    #[validate(length(min = 1, max = 50, message = "Tag name must be 1-50 characters"))]
    pub name: String,
}

/// Tag with usage count
#[derive(Debug, Clone, Serialize)]
pub struct TagWithCount {
    pub id: String,
    pub name: String,
    pub article_count: i64,
    pub created_at: String,
}

/// DTO for adding tags to an article
#[derive(Debug, Clone, Deserialize)]
pub struct AddTagsRequest {
    pub tag_ids: Vec<String>,
}

/// DTO for removing tags from an article
#[derive(Debug, Clone, Deserialize)]
pub struct RemoveTagsRequest {
    pub tag_ids: Vec<String>,
}
