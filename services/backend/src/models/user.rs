//! User model and related types
//!
//! Defines user entity, roles, and DTOs.

use serde::{Deserialize, Serialize};
use validator::Validate;

/// User entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub role: UserRole,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

/// User roles with hierarchical permissions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    SuperAdmin,
    Admin,
    Editor,
    Viewer,
}

impl UserRole {
    /// Check if role can manage users (super_admin, admin)
    pub fn can_manage_users(&self) -> bool {
        matches!(self, UserRole::SuperAdmin | UserRole::Admin)
    }

    /// Check if role can edit articles (super_admin, admin, editor)
    pub fn can_edit_articles(&self) -> bool {
        matches!(
            self,
            UserRole::SuperAdmin | UserRole::Admin | UserRole::Editor
        )
    }

    /// Check if role can view audit logs (super_admin, admin)
    pub fn can_view_audit(&self) -> bool {
        matches!(self, UserRole::SuperAdmin | UserRole::Admin)
    }

    /// Check if role can delete articles (super_admin, admin)
    pub fn can_delete_articles(&self) -> bool {
        matches!(self, UserRole::SuperAdmin | UserRole::Admin)
    }

    /// Check if role can manage tags (super_admin, admin, editor)
    pub fn can_manage_tags(&self) -> bool {
        matches!(
            self,
            UserRole::SuperAdmin | UserRole::Admin | UserRole::Editor
        )
    }

    /// Check if this role can modify another role
    pub fn can_modify_role(&self, target: &UserRole) -> bool {
        match self {
            UserRole::SuperAdmin => true, // Can modify anyone
            UserRole::Admin => !matches!(target, UserRole::SuperAdmin | UserRole::Admin),
            _ => false,
        }
    }

    /// Convert to database string
    pub fn as_str(&self) -> &'static str {
        match self {
            UserRole::SuperAdmin => "super_admin",
            UserRole::Admin => "admin",
            UserRole::Editor => "editor",
            UserRole::Viewer => "viewer",
        }
    }

    /// Parse from database string
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "super_admin" => Some(UserRole::SuperAdmin),
            "admin" => Some(UserRole::Admin),
            "editor" => Some(UserRole::Editor),
            "viewer" => Some(UserRole::Viewer),
            _ => None,
        }
    }
}

/// DTO for creating a new user
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateUser {
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    pub password_hash: String,
    pub role: UserRole,
}

/// DTO for user registration (public)
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct RegisterUser {
    #[validate(length(min = 3, max = 50, message = "Username must be 3-50 characters"))]
    pub username: String,
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
    #[validate(length(min = 8, max = 128, message = "Password must be 8-128 characters"))]
    pub password: String,
}

/// DTO for updating a user
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct UpdateUser {
    #[validate(length(min = 3, max = 50))]
    pub username: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    pub role: Option<UserRole>,
    pub is_active: Option<bool>,
}

/// DTO for login
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(length(min = 1, message = "Username required"))]
    pub username: String,
    #[validate(length(min = 1, message = "Password required"))]
    pub password: String,
}

/// Public user info (safe to expose)
#[derive(Debug, Clone, Serialize)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    pub email: String,
    pub role: UserRole,
    pub is_active: bool,
    pub created_at: String,
}

impl From<User> for UserInfo {
    fn from(user: User) -> Self {
        UserInfo {
            id: user.id,
            username: user.username,
            email: user.email,
            role: user.role,
            is_active: user.is_active,
            created_at: user.created_at,
        }
    }
}

/// Response after successful login
#[derive(Debug, Clone, Serialize)]
pub struct LoginResponse {
    pub user: UserInfo,
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_permissions() {
        assert!(UserRole::SuperAdmin.can_manage_users());
        assert!(UserRole::Admin.can_manage_users());
        assert!(!UserRole::Editor.can_manage_users());
        assert!(!UserRole::Viewer.can_manage_users());
    }

    #[test]
    fn test_role_hierarchy() {
        assert!(UserRole::SuperAdmin.can_modify_role(&UserRole::Admin));
        assert!(UserRole::SuperAdmin.can_modify_role(&UserRole::SuperAdmin));
        assert!(UserRole::Admin.can_modify_role(&UserRole::Editor));
        assert!(!UserRole::Admin.can_modify_role(&UserRole::SuperAdmin));
        assert!(!UserRole::Editor.can_modify_role(&UserRole::Viewer));
    }

    #[test]
    fn test_role_string_conversion() {
        assert_eq!(UserRole::SuperAdmin.as_str(), "super_admin");
        assert_eq!(UserRole::from_str("admin"), Some(UserRole::Admin));
        assert_eq!(UserRole::from_str("invalid"), None);
    }
}
