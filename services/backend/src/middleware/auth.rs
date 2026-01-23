//! Authentication middleware
//!
//! JWT validation and claims extraction.

use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web, Error, HttpMessage, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};

use crate::config::Config;
use crate::error::{AppError, AuthError};
use crate::security::jwt::{self, Claims};

/// Extract and validate JWT from request
pub fn extract_claims(req: &ServiceRequest, config: &Config) -> Result<Claims, AppError> {
    // Try to get token from cookie first
    let cookie_token = req.cookie("token").map(|c| c.value().to_string());

    // Or from Authorization header
    let header_token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .map(|s| s.to_string());

    let token = cookie_token
        .or(header_token)
        .ok_or(AppError::Auth(AuthError::MissingToken))?;

    jwt::validate_token(&token, &config.security)
}

/// Check if path requires authentication
pub fn requires_auth(path: &str) -> bool {
    let public_paths = [
        "/api/health",
        "/api/auth/login",
        "/api/auth/register",
        "/api/articles/published",
        "/api/search/public",
    ];

    for public in &public_paths {
        if path.starts_with(public) {
            return false;
        }
    }

    // GET /api/tags is public
    if path == "/api/tags" {
        return false;
    }

    path.starts_with("/api/")
}

/// Authentication middleware factory
pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService { service }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let path = req.path().to_string();

        // Check if route requires auth
        if !requires_auth(&path) {
            let fut = self.service.call(req);
            return Box::pin(async move {
                let res = fut.await?;
                Ok(res.map_into_left_body())
            });
        }

        // Get config from app data
        let config = req.app_data::<web::Data<Config>>().cloned();

        let config = match config {
            Some(c) => c,
            None => {
                let response = HttpResponse::InternalServerError()
                    .json(serde_json::json!({"error": "Server configuration error"}));
                return Box::pin(async move {
                    Ok(req.into_response(response).map_into_right_body())
                });
            }
        };

        // Extract and validate JWT
        match extract_claims(&req, &config) {
            Ok(claims) => {
                req.extensions_mut().insert(claims);
                let fut = self.service.call(req);
                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res.map_into_left_body())
                })
            }
            Err(e) => {
                let response = HttpResponse::Unauthorized()
                    .json(serde_json::json!({"error": e.to_string()}));
                Box::pin(async move { Ok(req.into_response(response).map_into_right_body()) })
            }
        }
    }
}
