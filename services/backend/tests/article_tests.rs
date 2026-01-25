//! Article CRUD tests for Knowledge Vault
//! Tests article creation, reading, updating, deletion, and encryption

use knowledge_vault::models::article::{Article, CreateArticle, UpdateArticle};
use knowledge_vault::security::encryption;
use knowledge_vault::db::Database;
use knowledge_vault::config::Config;
use pretty_assertions::assert_eq;
use tempfile::tempdir;

/// Test article creation with encryption
#[test]
fn test_create_article_encrypts_content() {
    let (db, config) = setup_test_db();
    let user_id = create_test_user(&db);

    let create = CreateArticle {
        title: "Test Article".to_string(),
        content: "This is the article content that should be encrypted".to_string(),
        tags: vec![],
    };

    let article = db.create_article(&create, &user_id, &config.security)
        .expect("Article creation should succeed");

    assert!(!article.id.is_empty(), "Article should have ID");
    assert_eq!(article.title, create.title);
    assert_eq!(article.author_id, user_id);

    // Verify content was encrypted (raw DB should have encrypted blob)
    let raw = db.get_article_raw(&article.id)
        .expect("Raw article should exist");

    assert_ne!(
        raw.content_encrypted.as_slice(),
        create.content.as_bytes(),
        "Stored content should be encrypted"
    );
}

/// Test article retrieval decrypts content
#[test]
fn test_get_article_decrypts_content() {
    let (db, config) = setup_test_db();
    let user_id = create_test_user(&db);

    let original_content = "Secret knowledge content";
    let create = CreateArticle {
        title: "Secret Article".to_string(),
        content: original_content.to_string(),
        tags: vec![],
    };

    let created = db.create_article(&create, &user_id, &config.security)
        .expect("Article creation should succeed");

    let retrieved = db.get_article(&created.id, &config.security)
        .expect("Article retrieval should succeed")
        .expect("Article should exist");

    assert_eq!(retrieved.content, original_content, "Content should be decrypted");
}

/// Test article update re-encrypts content
#[test]
fn test_update_article_reencrypts() {
    let (db, config) = setup_test_db();
    let user_id = create_test_user(&db);

    let create = CreateArticle {
        title: "Original Title".to_string(),
        content: "Original content".to_string(),
        tags: vec![],
    };

    let article = db.create_article(&create, &user_id, &config.security)
        .expect("Create should succeed");

    let original_raw = db.get_article_raw(&article.id)
        .expect("Raw should exist");

    let update = UpdateArticle {
        title: Some("Updated Title".to_string()),
        content: Some("Updated content".to_string()),
        is_published: Some(true),
    };

    let updated = db.update_article(&article.id, &update, &config.security)
        .expect("Update should succeed");

    assert_eq!(updated.title, "Updated Title");

    let updated_raw = db.get_article_raw(&article.id)
        .expect("Updated raw should exist");

    // Nonce should be different (re-encrypted)
    assert_ne!(
        original_raw.content_nonce,
        updated_raw.content_nonce,
        "Nonce should change on re-encryption"
    );
}

/// Test article deletion
#[test]
fn test_delete_article() {
    let (db, config) = setup_test_db();
    let user_id = create_test_user(&db);

    let create = CreateArticle {
        title: "To Delete".to_string(),
        content: "Will be deleted".to_string(),
        tags: vec![],
    };

    let article = db.create_article(&create, &user_id, &config.security)
        .expect("Create should succeed");

    db.delete_article(&article.id)
        .expect("Delete should succeed");

    let result = db.get_article(&article.id, &config.security)
        .expect("Get should not error");

    assert!(result.is_none(), "Deleted article should not exist");
}

/// Test listing articles with pagination
#[test]
fn test_list_articles_pagination() {
    let (db, config) = setup_test_db();
    let user_id = create_test_user(&db);

    // Create 5 articles
    const ARTICLE_COUNT: usize = 5;
    for i in 0..ARTICLE_COUNT {
        let create = CreateArticle {
            title: format!("Article {}", i),
            content: format!("Content {}", i),
            tags: vec![],
        };
        db.create_article(&create, &user_id, &config.security)
            .expect("Create should succeed");
    }

    // Get first page
    let page1 = db.list_articles(0, 2, &config.security)
        .expect("List should succeed");
    assert_eq!(page1.len(), 2, "First page should have 2 articles");

    // Get second page
    let page2 = db.list_articles(2, 2, &config.security)
        .expect("List should succeed");
    assert_eq!(page2.len(), 2, "Second page should have 2 articles");

    // Get last page
    let page3 = db.list_articles(4, 2, &config.security)
        .expect("List should succeed");
    assert_eq!(page3.len(), 1, "Last page should have 1 article");

    // Verify no overlap
    let all_ids: Vec<_> = page1.iter()
        .chain(page2.iter())
        .chain(page3.iter())
        .map(|a| &a.id)
        .collect();

    let unique_count = {
        let mut ids = all_ids.clone();
        ids.sort();
        ids.dedup();
        ids.len()
    };

    assert_eq!(unique_count, ARTICLE_COUNT, "All articles should be unique");
}

/// Test article with tags
#[test]
fn test_article_with_tags() {
    let (db, config) = setup_test_db();
    let user_id = create_test_user(&db);

    // Create tags first
    let tag1 = db.create_tag("rust").expect("Tag create should succeed");
    let tag2 = db.create_tag("security").expect("Tag create should succeed");

    let create = CreateArticle {
        title: "Tagged Article".to_string(),
        content: "Content with tags".to_string(),
        tags: vec![tag1.id.clone(), tag2.id.clone()],
    };

    let article = db.create_article(&create, &user_id, &config.security)
        .expect("Create should succeed");

    let tags = db.get_article_tags(&article.id)
        .expect("Get tags should succeed");

    assert_eq!(tags.len(), 2, "Article should have 2 tags");

    let tag_names: Vec<_> = tags.iter().map(|t| t.name.as_str()).collect();
    assert!(tag_names.contains(&"rust"));
    assert!(tag_names.contains(&"security"));
}

/// Test article content size limits
#[test]
fn test_article_large_content() {
    let (db, config) = setup_test_db();
    let user_id = create_test_user(&db);

    // Create article with large content (but reasonable)
    let large_content = "x".repeat(100_000); // 100KB

    let create = CreateArticle {
        title: "Large Article".to_string(),
        content: large_content.clone(),
        tags: vec![],
    };

    let article = db.create_article(&create, &user_id, &config.security)
        .expect("Large article should succeed");

    let retrieved = db.get_article(&article.id, &config.security)
        .expect("Get should succeed")
        .expect("Article should exist");

    assert_eq!(retrieved.content.len(), large_content.len());
}

/// Test article with special characters in content
#[test]
fn test_article_special_characters() {
    let (db, config) = setup_test_db();
    let user_id = create_test_user(&db);

    let special_content = r#"
        <script>alert('xss')</script>
        SELECT * FROM users;
        日本語テスト
        🎉🚀💻
        "quotes" and 'apostrophes'
        \n\t\r special chars
    "#;

    let create = CreateArticle {
        title: "Special <Characters>".to_string(),
        content: special_content.to_string(),
        tags: vec![],
    };

    let article = db.create_article(&create, &user_id, &config.security)
        .expect("Create should succeed");

    let retrieved = db.get_article(&article.id, &config.security)
        .expect("Get should succeed")
        .expect("Article should exist");

    assert_eq!(retrieved.content, special_content);
    assert_eq!(retrieved.title, "Special <Characters>");
}

/// Test encryption uses unique nonce per article
#[test]
fn test_encryption_unique_nonces() {
    let (db, config) = setup_test_db();
    let user_id = create_test_user(&db);

    let content = "Same content for all";
    let mut nonces = Vec::new();

    const ARTICLE_COUNT: usize = 5;
    for i in 0..ARTICLE_COUNT {
        let create = CreateArticle {
            title: format!("Article {}", i),
            content: content.to_string(),
            tags: vec![],
        };

        let article = db.create_article(&create, &user_id, &config.security)
            .expect("Create should succeed");

        let raw = db.get_article_raw(&article.id)
            .expect("Raw should exist");

        nonces.push(raw.content_nonce);
    }

    // All nonces should be unique
    let unique_count = {
        let mut n = nonces.clone();
        n.sort();
        n.dedup();
        n.len()
    };

    assert_eq!(unique_count, ARTICLE_COUNT, "All nonces should be unique");
}

/// Test only published articles are listed for non-admins
#[test]
fn test_list_published_only() {
    let (db, config) = setup_test_db();
    let user_id = create_test_user(&db);

    // Create published article
    let create1 = CreateArticle {
        title: "Published".to_string(),
        content: "Visible".to_string(),
        tags: vec![],
    };
    let art1 = db.create_article(&create1, &user_id, &config.security)
        .expect("Create should succeed");
    db.update_article(&art1.id, &UpdateArticle {
        title: None,
        content: None,
        is_published: Some(true),
    }, &config.security).expect("Update should succeed");

    // Create unpublished article
    let create2 = CreateArticle {
        title: "Draft".to_string(),
        content: "Hidden".to_string(),
        tags: vec![],
    };
    db.create_article(&create2, &user_id, &config.security)
        .expect("Create should succeed");

    // List published only
    let published = db.list_published_articles(0, 10, &config.security)
        .expect("List should succeed");

    assert_eq!(published.len(), 1);
    assert_eq!(published[0].title, "Published");
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
        role: knowledge_vault::models::user::UserRole::Editor,
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
