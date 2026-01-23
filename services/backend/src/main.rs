//! Knowledge Vault Backend Server
//!
//! Single-threaded Actix-web server optimized for low resource usage.

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use std::fs;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use knowledge_vault::{
    config::Config,
    db::Database,
    handlers::{admin, articles, audit, auth, favorites, health, search, tags},
    middleware::auth::AuthMiddleware,
    models::audit::{AuditAction, AuditResourceType, CreateAuditEntry},
    security::password,
};

/// Seed initial users if database is empty
fn seed_users(db: &Database, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    // Check if users already exist
    if db.count_users()? > 0 {
        info!("Users already exist, skipping seed");
        return Ok(());
    }

    info!("Seeding initial users...");

    let users = vec![
        ("superadmin", "superadmin@knowledgevault.local", "SuperAdmin123!", "super_admin"),
        ("admin1", "admin1@knowledgevault.local", "Admin123!", "admin"),
        ("editor1", "editor1@knowledgevault.local", "Editor123!", "editor"),
        ("editor2", "editor2@knowledgevault.local", "Editor123!", "editor"),
        ("viewer1", "viewer1@knowledgevault.local", "Viewer123!", "viewer"),
        ("viewer2", "viewer2@knowledgevault.local", "Viewer123!", "viewer"),
    ];

    for (username, email, pwd, role) in users {
        let password_hash = password::hash_password(pwd, &config.security.argon2)?;
        let role = knowledge_vault::models::user::UserRole::from_str(role)
            .unwrap_or(knowledge_vault::models::user::UserRole::Viewer);

        db.create_user(&knowledge_vault::models::user::CreateUser {
            username: username.to_string(),
            email: email.to_string(),
            password_hash,
            role,
        })?;

        info!("Created user: {}", username);
    }

    // Log system startup with seeding
    db.create_audit_entry(&CreateAuditEntry {
        user_id: None,
        action: AuditAction::SystemStartup,
        resource_type: AuditResourceType::System,
        resource_id: None,
        details: Some(r#"{"seeded_users": 6}"#.to_string()),
        ip_address: None,
        user_agent: None,
    })?;

    info!("Seeded 6 users successfully");
    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config_path = std::env::var("CONFIG_PATH").unwrap_or_else(|_| "config.yaml".to_string());
    let config = Config::load(&config_path).expect("Failed to load configuration");

    info!("Starting Knowledge Vault v{}", env!("CARGO_PKG_VERSION"));
    info!("Server: {}:{}", config.server.host, config.server.port);

    // Ensure data directory exists
    if let Some(parent) = std::path::Path::new(&config.database.path).parent() {
        fs::create_dir_all(parent).ok();
    }

    // Initialize database
    let db = Database::new(&config).expect("Failed to initialize database");
    db.run_migrations().expect("Failed to run migrations");

    // Seed users
    seed_users(&db, &config).expect("Failed to seed users");

    // Save bind address before moving config
    let bind_host = config.server.host.clone();
    let bind_port = config.server.port;

    let db = web::Data::new(db);
    let config = web::Data::new(config);

    // Start server with single worker (1 CPU constraint)
    HttpServer::new(move || {
        // Configure CORS
        let cors = Cors::default()
            .allowed_origin_fn(|_origin, _req_head| {
                // In production, validate against config.cors.allowed_origins
                true
            })
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::CONTENT_TYPE,
            ])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(AuthMiddleware)
            .app_data(db.clone())
            .app_data(config.clone())
            // Health check
            .service(health::health_check)
            // Auth routes
            .service(auth::login)
            .service(auth::logout)
            .service(auth::me)
            .service(auth::register)
            // Article routes
            .service(articles::list_articles)
            .service(articles::list_published)
            .service(articles::get_article)
            .service(articles::create_article)
            .service(articles::update_article)
            .service(articles::delete_article)
            .service(articles::add_tags)
            // Tag routes
            .service(tags::list_tags)
            .service(tags::create_tag)
            .service(tags::delete_tag)
            // Favorite routes
            .service(favorites::list_favorites)
            .service(favorites::add_favorite)
            .service(favorites::remove_favorite)
            .service(favorites::favorite_status)
            // Search routes
            .service(search::search)
            .service(search::search_public)
            // Admin routes
            .service(admin::list_users)
            .service(admin::get_user)
            .service(admin::update_user)
            // Audit routes
            .service(audit::list_audit_logs)
            .service(audit::audit_stats)
    })
    .workers(1) // Single worker for 1 CPU constraint
    .bind((bind_host.as_str(), bind_port))?
    .run()
    .await
}
