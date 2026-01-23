//! Audit log model and related types
//!
//! Defines audit entry for comprehensive action logging.

use serde::{Deserialize, Serialize};

/// Audit log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub id: String,
    pub user_id: Option<String>,
    pub action: AuditAction,
    pub resource_type: AuditResourceType,
    pub resource_id: Option<String>,
    pub details: Option<String>, // JSON
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: String,
}

/// Audit action types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditAction {
    // CRUD operations
    Create,
    Read,
    Update,
    Delete,

    // Auth operations
    Login,
    Logout,
    LoginFailed,

    // Admin operations
    UserActivated,
    UserDeactivated,
    RoleChanged,

    // System operations
    SystemStartup,
    SystemShutdown,

    // Search
    Search,

    // Favorites
    Favorite,
    Unfavorite,
}

impl AuditAction {
    pub fn as_str(&self) -> &'static str {
        match self {
            AuditAction::Create => "create",
            AuditAction::Read => "read",
            AuditAction::Update => "update",
            AuditAction::Delete => "delete",
            AuditAction::Login => "login",
            AuditAction::Logout => "logout",
            AuditAction::LoginFailed => "login_failed",
            AuditAction::UserActivated => "user_activated",
            AuditAction::UserDeactivated => "user_deactivated",
            AuditAction::RoleChanged => "role_changed",
            AuditAction::SystemStartup => "system_startup",
            AuditAction::SystemShutdown => "system_shutdown",
            AuditAction::Search => "search",
            AuditAction::Favorite => "favorite",
            AuditAction::Unfavorite => "unfavorite",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "create" => Some(AuditAction::Create),
            "read" => Some(AuditAction::Read),
            "update" => Some(AuditAction::Update),
            "delete" => Some(AuditAction::Delete),
            "login" => Some(AuditAction::Login),
            "logout" => Some(AuditAction::Logout),
            "login_failed" => Some(AuditAction::LoginFailed),
            "user_activated" => Some(AuditAction::UserActivated),
            "user_deactivated" => Some(AuditAction::UserDeactivated),
            "role_changed" => Some(AuditAction::RoleChanged),
            "system_startup" => Some(AuditAction::SystemStartup),
            "system_shutdown" => Some(AuditAction::SystemShutdown),
            "search" => Some(AuditAction::Search),
            "favorite" => Some(AuditAction::Favorite),
            "unfavorite" => Some(AuditAction::Unfavorite),
            _ => None,
        }
    }
}

/// Audit resource types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditResourceType {
    User,
    Article,
    Tag,
    Favorite,
    Auth,
    System,
}

impl AuditResourceType {
    pub fn as_str(&self) -> &'static str {
        match self {
            AuditResourceType::User => "user",
            AuditResourceType::Article => "article",
            AuditResourceType::Tag => "tag",
            AuditResourceType::Favorite => "favorite",
            AuditResourceType::Auth => "auth",
            AuditResourceType::System => "system",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "user" => Some(AuditResourceType::User),
            "article" => Some(AuditResourceType::Article),
            "tag" => Some(AuditResourceType::Tag),
            "favorite" => Some(AuditResourceType::Favorite),
            "auth" => Some(AuditResourceType::Auth),
            "system" => Some(AuditResourceType::System),
            _ => None,
        }
    }
}

/// DTO for creating an audit entry
#[derive(Debug, Clone)]
pub struct CreateAuditEntry {
    pub user_id: Option<String>,
    pub action: AuditAction,
    pub resource_type: AuditResourceType,
    pub resource_id: Option<String>,
    pub details: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

/// Audit log with user info
#[derive(Debug, Clone, Serialize)]
pub struct AuditEntryWithUser {
    pub id: String,
    pub user_id: Option<String>,
    pub username: Option<String>,
    pub action: AuditAction,
    pub resource_type: AuditResourceType,
    pub resource_id: Option<String>,
    pub details: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: String,
}

/// Audit filter options
#[derive(Debug, Clone, Deserialize, Default)]
pub struct AuditFilter {
    pub user_id: Option<String>,
    pub action: Option<String>,
    pub resource_type: Option<String>,
    pub from_date: Option<String>,
    pub to_date: Option<String>,
}
