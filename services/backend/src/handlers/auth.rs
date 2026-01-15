//! Authentication handlers
//!
//! Login, logout, and session management.

use actix_web::{cookie::Cookie, get, post, web, HttpRequest, HttpResponse};
use serde_json::json;
use time::Duration;
use validator::Validate;

use crate::config::Config;
use crate::db::Database;
use crate::error::{AppError, AppResult, AuthError};
use crate::middleware::audit::log_audit;
use crate::models::audit::{AuditAction, AuditResourceType};
use crate::models::user::{LoginRequest, LoginResponse, RegisterUser, UserInfo, UserRole};
use crate::security::{jwt, password};

/// Login endpoint
#[post("/api/auth/login")]
pub async fn login(
    req: HttpRequest,
    db: web::Data<Database>,
    config: web::Data<Config>,
    body: web::Json<LoginRequest>,
) -> AppResult<HttpResponse> {
    // Validate input
    body.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    // Get user by username
    let user = db
        .get_user_by_username(&body.username)?
        .ok_or(AppError::Auth(AuthError::InvalidCredentials))?;

    // Check if user is active
    if !user.is_active {
        log_audit(
            &db,
            None,
            AuditAction::LoginFailed,
            AuditResourceType::Auth,
            None,
            Some(json!({"username": body.username, "reason": "account_disabled"}).to_string()),
            &req,
        );
        return Err(AppError::Auth(AuthError::AccountDisabled));
    }

    // Verify password
    let valid = password::verify_password(&body.password, &user.password_hash)?;
    if !valid {
        log_audit(
            &db,
            None,
            AuditAction::LoginFailed,
            AuditResourceType::Auth,
            None,
            Some(json!({"username": body.username, "reason": "invalid_password"}).to_string()),
            &req,
        );
        return Err(AppError::Auth(AuthError::InvalidCredentials));
    }

    // Generate JWT
    let token = jwt::generate_token(&user.id, &user.role, &config.security)?;

    // Create HttpOnly cookie
    let cookie = Cookie::build("token", token)
        .path("/")
        .http_only(true)
        .secure(false) // Set to true in production with HTTPS
        .same_site(actix_web::cookie::SameSite::Strict)
        .max_age(Duration::hours(config.security.jwt_expiry_hours))
        .finish();

    // Log successful login
    log_audit(
        &db,
        Some(&user.id),
        AuditAction::Login,
        AuditResourceType::Auth,
        Some(&user.id),
        None,
        &req,
    );

    let response = LoginResponse {
        user: UserInfo::from(user),
        message: "Login successful".to_string(),
    };

    Ok(HttpResponse::Ok().cookie(cookie).json(response))
}

/// Logout endpoint
#[post("/api/auth/logout")]
pub async fn logout(
    req: HttpRequest,
    db: web::Data<Database>,
    claims: Option<web::ReqData<jwt::Claims>>,
) -> AppResult<HttpResponse> {
    // Log logout if user was authenticated
    if let Some(claims) = claims {
        log_audit(
            &db,
            Some(&claims.sub),
            AuditAction::Logout,
            AuditResourceType::Auth,
            Some(&claims.sub),
            None,
            &req,
        );
    }

    // Clear cookie
    let cookie = Cookie::build("token", "")
        .path("/")
        .http_only(true)
        .max_age(Duration::seconds(0))
        .finish();

    Ok(HttpResponse::Ok().cookie(cookie).json(json!({
        "message": "Logged out successfully"
    })))
}

/// Get current user info
#[get("/api/auth/me")]
pub async fn me(
    db: web::Data<Database>,
    claims: web::ReqData<jwt::Claims>,
) -> AppResult<HttpResponse> {
    let user = db
        .get_user_by_id(&claims.sub)?
        .ok_or(AppError::Auth(AuthError::TokenInvalid))?;

    Ok(HttpResponse::Ok().json(UserInfo::from(user)))
}

/// Register new user (creates viewer by default)
#[post("/api/auth/register")]
pub async fn register(
    req: HttpRequest,
    db: web::Data<Database>,
    config: web::Data<Config>,
    body: web::Json<RegisterUser>,
) -> AppResult<HttpResponse> {
    // Validate input
    body.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    // Check if username already exists
    if db.get_user_by_username(&body.username)?.is_some() {
        return Err(AppError::Validation("Username already taken".to_string()));
    }

    // Hash password
    let password_hash = password::hash_password(&body.password, &config.security.argon2)?;

    // Create user
    let user = db.create_user(&crate::models::user::CreateUser {
        username: body.username.clone(),
        email: body.email.clone(),
        password_hash,
        role: UserRole::Viewer, // Default role
    })?;

    // Log registration
    log_audit(
        &db,
        Some(&user.id),
        AuditAction::Create,
        AuditResourceType::User,
        Some(&user.id),
        Some(json!({"username": user.username}).to_string()),
        &req,
    );

    Ok(HttpResponse::Created().json(json!({
        "message": "Registration successful",
        "user": UserInfo::from(user)
    })))
}
