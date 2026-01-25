//! Audit logging tests for Knowledge Vault
//! Tests that all actions are properly logged and retrievable

use knowledge_vault::models::audit::{AuditAction, AuditEntry, AuditResourceType, CreateAuditEntry};
use knowledge_vault::db::Database;
use knowledge_vault::config::Config;
use pretty_assertions::assert_eq;

/// Test audit entry creation
#[test]
fn test_create_audit_entry() {
    let (db, _config) = setup_test_db();
    let user_id = create_test_user(&db);

    let entry = CreateAuditEntry {
        user_id: Some(user_id.clone()),
        action: AuditAction::Create,
        resource_type: AuditResourceType::Article,
        resource_id: Some("article-123".to_string()),
        details: Some(r#"{"title": "New Article"}"#.to_string()),
        ip_address: Some("192.168.1.1".to_string()),
        user_agent: Some("Mozilla/5.0".to_string()),
    };

    let audit = db.create_audit_entry(&entry)
        .expect("Audit creation should succeed");

    assert!(!audit.id.is_empty());
    assert_eq!(audit.user_id, Some(user_id));
    assert_eq!(audit.action, AuditAction::Create);
    assert_eq!(audit.resource_type, AuditResourceType::Article);
}

/// Test audit entry without user (system action)
#[test]
fn test_create_system_audit_entry() {
    let (db, _config) = setup_test_db();

    let entry = CreateAuditEntry {
        user_id: None,
        action: AuditAction::SystemStartup,
        resource_type: AuditResourceType::System,
        resource_id: None,
        details: Some(r#"{"version": "1.0.0"}"#.to_string()),
        ip_address: None,
        user_agent: None,
    };

    let audit = db.create_audit_entry(&entry)
        .expect("System audit should succeed");

    assert!(audit.user_id.is_none());
    assert_eq!(audit.action, AuditAction::SystemStartup);
}

/// Test listing audit entries with pagination
#[test]
fn test_list_audit_entries_pagination() {
    let (db, _config) = setup_test_db();
    let user_id = create_test_user(&db);

    // Create 5 audit entries
    const ENTRY_COUNT: usize = 5;
    for i in 0..ENTRY_COUNT {
        let entry = CreateAuditEntry {
            user_id: Some(user_id.clone()),
            action: AuditAction::Read,
            resource_type: AuditResourceType::Article,
            resource_id: Some(format!("article-{}", i)),
            details: None,
            ip_address: None,
            user_agent: None,
        };
        db.create_audit_entry(&entry).expect("Create should succeed");
    }

    // Get first page
    let page1 = db.list_audit_entries(0, 2, None, None, None)
        .expect("List should succeed");
    assert_eq!(page1.len(), 2);

    // Get second page
    let page2 = db.list_audit_entries(2, 2, None, None, None)
        .expect("List should succeed");
    assert_eq!(page2.len(), 2);
}

/// Test filtering audit by user
#[test]
fn test_filter_audit_by_user() {
    let (db, _config) = setup_test_db();
    let user1 = create_test_user(&db);
    let user2 = create_test_user(&db);

    // Create entries for user1
    for _ in 0..3 {
        db.create_audit_entry(&CreateAuditEntry {
            user_id: Some(user1.clone()),
            action: AuditAction::Read,
            resource_type: AuditResourceType::Article,
            resource_id: None,
            details: None,
            ip_address: None,
            user_agent: None,
        }).expect("Create should succeed");
    }

    // Create entries for user2
    for _ in 0..2 {
        db.create_audit_entry(&CreateAuditEntry {
            user_id: Some(user2.clone()),
            action: AuditAction::Update,
            resource_type: AuditResourceType::Article,
            resource_id: None,
            details: None,
            ip_address: None,
            user_agent: None,
        }).expect("Create should succeed");
    }

    // Filter by user1
    let user1_entries = db.list_audit_entries(0, 10, Some(&user1), None, None)
        .expect("List should succeed");
    assert_eq!(user1_entries.len(), 3);

    // Filter by user2
    let user2_entries = db.list_audit_entries(0, 10, Some(&user2), None, None)
        .expect("List should succeed");
    assert_eq!(user2_entries.len(), 2);
}

/// Test filtering audit by action type
#[test]
fn test_filter_audit_by_action() {
    let (db, _config) = setup_test_db();
    let user_id = create_test_user(&db);

    // Create different action types
    let actions = vec![
        AuditAction::Create,
        AuditAction::Read,
        AuditAction::Read,
        AuditAction::Update,
        AuditAction::Delete,
    ];

    for action in actions {
        db.create_audit_entry(&CreateAuditEntry {
            user_id: Some(user_id.clone()),
            action,
            resource_type: AuditResourceType::Article,
            resource_id: None,
            details: None,
            ip_address: None,
            user_agent: None,
        }).expect("Create should succeed");
    }

    // Filter by Read action
    let reads = db.list_audit_entries(0, 10, None, Some(AuditAction::Read), None)
        .expect("List should succeed");
    assert_eq!(reads.len(), 2);

    // Filter by Create action
    let creates = db.list_audit_entries(0, 10, None, Some(AuditAction::Create), None)
        .expect("List should succeed");
    assert_eq!(creates.len(), 1);
}

/// Test filtering audit by resource type
#[test]
fn test_filter_audit_by_resource_type() {
    let (db, _config) = setup_test_db();
    let user_id = create_test_user(&db);

    // Create entries for different resources
    let resources = vec![
        AuditResourceType::Article,
        AuditResourceType::Article,
        AuditResourceType::User,
        AuditResourceType::Tag,
        AuditResourceType::Auth,
    ];

    for resource in resources {
        db.create_audit_entry(&CreateAuditEntry {
            user_id: Some(user_id.clone()),
            action: AuditAction::Read,
            resource_type: resource,
            resource_id: None,
            details: None,
            ip_address: None,
            user_agent: None,
        }).expect("Create should succeed");
    }

    // Filter by Article resource
    let articles = db.list_audit_entries(0, 10, None, None, Some(AuditResourceType::Article))
        .expect("List should succeed");
    assert_eq!(articles.len(), 2);

    // Filter by Auth resource
    let auth = db.list_audit_entries(0, 10, None, None, Some(AuditResourceType::Auth))
        .expect("List should succeed");
    assert_eq!(auth.len(), 1);
}

/// Test audit entries are ordered by created_at descending
#[test]
fn test_audit_ordering() {
    let (db, _config) = setup_test_db();
    let user_id = create_test_user(&db);

    // Create entries with slight delays (in practice they'd have different timestamps)
    for i in 0..3 {
        db.create_audit_entry(&CreateAuditEntry {
            user_id: Some(user_id.clone()),
            action: AuditAction::Read,
            resource_type: AuditResourceType::Article,
            resource_id: Some(format!("article-{}", i)),
            details: None,
            ip_address: None,
            user_agent: None,
        }).expect("Create should succeed");
    }

    let entries = db.list_audit_entries(0, 10, None, None, None)
        .expect("List should succeed");

    // Verify descending order (newest first)
    for i in 0..entries.len() - 1 {
        assert!(
            entries[i].created_at >= entries[i + 1].created_at,
            "Entries should be in descending order"
        );
    }
}

/// Test audit log captures all required fields
#[test]
fn test_audit_captures_all_fields() {
    let (db, _config) = setup_test_db();
    let user_id = create_test_user(&db);

    let entry = CreateAuditEntry {
        user_id: Some(user_id.clone()),
        action: AuditAction::Login,
        resource_type: AuditResourceType::Auth,
        resource_id: Some(user_id.clone()),
        details: Some(r#"{"method": "password"}"#.to_string()),
        ip_address: Some("10.0.0.1".to_string()),
        user_agent: Some("TestAgent/1.0".to_string()),
    };

    let audit = db.create_audit_entry(&entry)
        .expect("Create should succeed");

    assert_eq!(audit.user_id, Some(user_id.clone()));
    assert_eq!(audit.action, AuditAction::Login);
    assert_eq!(audit.resource_type, AuditResourceType::Auth);
    assert_eq!(audit.resource_id, Some(user_id));
    assert!(audit.details.is_some());
    assert_eq!(audit.ip_address, Some("10.0.0.1".to_string()));
    assert_eq!(audit.user_agent, Some("TestAgent/1.0".to_string()));
    assert!(!audit.created_at.is_empty());
}

/// Test audit log count
#[test]
fn test_audit_count() {
    let (db, _config) = setup_test_db();
    let user_id = create_test_user(&db);

    // Create some entries
    const ENTRY_COUNT: usize = 7;
    for _ in 0..ENTRY_COUNT {
        db.create_audit_entry(&CreateAuditEntry {
            user_id: Some(user_id.clone()),
            action: AuditAction::Read,
            resource_type: AuditResourceType::Article,
            resource_id: None,
            details: None,
            ip_address: None,
            user_agent: None,
        }).expect("Create should succeed");
    }

    let count = db.count_audit_entries(None, None, None)
        .expect("Count should succeed");

    assert_eq!(count, ENTRY_COUNT as i64);
}

/// Test login/logout audit actions
#[test]
fn test_auth_audit_actions() {
    let (db, _config) = setup_test_db();
    let user_id = create_test_user(&db);

    // Log login
    db.create_audit_entry(&CreateAuditEntry {
        user_id: Some(user_id.clone()),
        action: AuditAction::Login,
        resource_type: AuditResourceType::Auth,
        resource_id: Some(user_id.clone()),
        details: None,
        ip_address: Some("192.168.1.100".to_string()),
        user_agent: None,
    }).expect("Login audit should succeed");

    // Log logout
    db.create_audit_entry(&CreateAuditEntry {
        user_id: Some(user_id.clone()),
        action: AuditAction::Logout,
        resource_type: AuditResourceType::Auth,
        resource_id: Some(user_id.clone()),
        details: None,
        ip_address: Some("192.168.1.100".to_string()),
        user_agent: None,
    }).expect("Logout audit should succeed");

    let auth_entries = db.list_audit_entries(0, 10, None, None, Some(AuditResourceType::Auth))
        .expect("List should succeed");

    assert_eq!(auth_entries.len(), 2);
}

/// Test failed login attempts are logged
#[test]
fn test_failed_login_audit() {
    let (db, _config) = setup_test_db();

    // Failed login (no user_id since auth failed)
    db.create_audit_entry(&CreateAuditEntry {
        user_id: None,
        action: AuditAction::LoginFailed,
        resource_type: AuditResourceType::Auth,
        resource_id: None,
        details: Some(r#"{"username": "unknown_user"}"#.to_string()),
        ip_address: Some("10.20.30.40".to_string()),
        user_agent: Some("SuspiciousBot/1.0".to_string()),
    }).expect("Failed login audit should succeed");

    let failed = db.list_audit_entries(0, 10, None, Some(AuditAction::LoginFailed), None)
        .expect("List should succeed");

    assert_eq!(failed.len(), 1);
    assert!(failed[0].user_id.is_none());
    assert!(failed[0].details.as_ref().unwrap().contains("unknown_user"));
}

/// Test audit entry for article CRUD operations
#[test]
fn test_article_crud_audit() {
    let (db, _config) = setup_test_db();
    let user_id = create_test_user(&db);
    let article_id = "article-uuid-123";

    // Create
    db.create_audit_entry(&CreateAuditEntry {
        user_id: Some(user_id.clone()),
        action: AuditAction::Create,
        resource_type: AuditResourceType::Article,
        resource_id: Some(article_id.to_string()),
        details: Some(r#"{"title": "New Article"}"#.to_string()),
        ip_address: None,
        user_agent: None,
    }).expect("Create audit should succeed");

    // Read
    db.create_audit_entry(&CreateAuditEntry {
        user_id: Some(user_id.clone()),
        action: AuditAction::Read,
        resource_type: AuditResourceType::Article,
        resource_id: Some(article_id.to_string()),
        details: None,
        ip_address: None,
        user_agent: None,
    }).expect("Read audit should succeed");

    // Update
    db.create_audit_entry(&CreateAuditEntry {
        user_id: Some(user_id.clone()),
        action: AuditAction::Update,
        resource_type: AuditResourceType::Article,
        resource_id: Some(article_id.to_string()),
        details: Some(r#"{"changed": ["title", "content"]}"#.to_string()),
        ip_address: None,
        user_agent: None,
    }).expect("Update audit should succeed");

    // Delete
    db.create_audit_entry(&CreateAuditEntry {
        user_id: Some(user_id.clone()),
        action: AuditAction::Delete,
        resource_type: AuditResourceType::Article,
        resource_id: Some(article_id.to_string()),
        details: None,
        ip_address: None,
        user_agent: None,
    }).expect("Delete audit should succeed");

    let article_audit = db.list_audit_entries(0, 10, None, None, Some(AuditResourceType::Article))
        .expect("List should succeed");

    assert_eq!(article_audit.len(), 4);
}

// === Test Setup Helpers ===

fn setup_test_db() -> (Database, Config) {
    let config = test_config();
    let db = Database::new_in_memory(&config)
        .expect("Test DB should initialize");
    db.run_migrations().expect("Migrations should succeed");
    (db, config)
}

fn create_test_user(db: &Database) -> String {
    use knowledge_vault::models::user::CreateUser;

    let user = db.create_user(&CreateUser {
        username: format!("testuser_{}", uuid::Uuid::new_v4()),
        email: format!("test_{}@example.com", uuid::Uuid::new_v4()),
        password_hash: "not_a_real_hash".to_string(),
        role: knowledge_vault::models::user::UserRole::Admin,
    }).expect("User creation should succeed");

    user.id
}

fn test_config() -> Config {
    Config {
        server: knowledge_vault::config::ServerConfig {
            host: "127.0.0.1".to_string(),
            port: 8080,
            workers: 1,
        },
        database: knowledge_vault::config::DatabaseConfig {
            path: ":memory:".to_string(),
            max_connections: 1,
        },
        security: knowledge_vault::config::SecurityConfig {
            jwt_secret: "test_secret_key_must_be_32_bytes_long".to_string(),
            jwt_expiry_hours: 24,
            encryption_key: "test_encryption_key_32_bytes_xx".to_string(),
            argon2: knowledge_vault::config::Argon2Config {
                memory_kib: 4096,
                iterations: 1,
                parallelism: 1,
            },
        },
        logging: knowledge_vault::config::LoggingConfig {
            level: "debug".to_string(),
            audit_retention_days: 90,
        },
        cors: knowledge_vault::config::CorsConfig {
            allowed_origins: vec!["http://localhost:5173".to_string()],
        },
    }
}
