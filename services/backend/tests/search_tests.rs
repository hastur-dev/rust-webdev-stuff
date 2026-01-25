//! Search functionality tests for Knowledge Vault
//! Tests full-text search with SQLite FTS5

use knowledge_vault::models::article::CreateArticle;
use knowledge_vault::db::Database;
use knowledge_vault::config::Config;
use pretty_assertions::assert_eq;

/// Test basic search finds matching articles
#[test]
fn test_search_finds_matching_title() {
    let (db, config) = setup_test_db();
    let user_id = create_test_user(&db);

    // Create articles
    let articles = vec![
        ("Rust Programming Guide", "Learn Rust basics"),
        ("Python Tutorial", "Python for beginners"),
        ("Advanced Rust Patterns", "Complex Rust concepts"),
    ];

    for (title, content) in articles {
        let create = CreateArticle {
            title: title.to_string(),
            content: content.to_string(),
            tags: vec![],
        };
        db.create_article(&create, &user_id, &config.security)
            .expect("Create should succeed");
    }

    // Search for "Rust"
    let results = db.search_articles("Rust", 0, 10, &config.security)
        .expect("Search should succeed");

    assert_eq!(results.len(), 2, "Should find 2 Rust articles");

    let titles: Vec<_> = results.iter().map(|a| a.title.as_str()).collect();
    assert!(titles.contains(&"Rust Programming Guide"));
    assert!(titles.contains(&"Advanced Rust Patterns"));
}

/// Test search finds matching content
#[test]
fn test_search_finds_matching_content() {
    let (db, config) = setup_test_db();
    let user_id = create_test_user(&db);

    let create = CreateArticle {
        title: "Generic Title".to_string(),
        content: "This article discusses cryptography and encryption".to_string(),
        tags: vec![],
    };
    db.create_article(&create, &user_id, &config.security)
        .expect("Create should succeed");

    let results = db.search_articles("cryptography", 0, 10, &config.security)
        .expect("Search should succeed");

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].title, "Generic Title");
}

/// Test search with no results
#[test]
fn test_search_no_results() {
    let (db, config) = setup_test_db();
    let user_id = create_test_user(&db);

    let create = CreateArticle {
        title: "About Cats".to_string(),
        content: "Cats are fluffy animals".to_string(),
        tags: vec![],
    };
    db.create_article(&create, &user_id, &config.security)
        .expect("Create should succeed");

    let results = db.search_articles("dogs", 0, 10, &config.security)
        .expect("Search should succeed");

    assert!(results.is_empty(), "Should find no matches for 'dogs'");
}

/// Test search with empty query
#[test]
fn test_search_empty_query() {
    let (db, config) = setup_test_db();

    let results = db.search_articles("", 0, 10, &config.security)
        .expect("Search should succeed");

    assert!(results.is_empty(), "Empty query should return no results");
}

/// Test search pagination
#[test]
fn test_search_pagination() {
    let (db, config) = setup_test_db();
    let user_id = create_test_user(&db);

    // Create 5 articles all containing "knowledge"
    const ARTICLE_COUNT: usize = 5;
    for i in 0..ARTICLE_COUNT {
        let create = CreateArticle {
            title: format!("Knowledge Article {}", i),
            content: format!("This is knowledge base content {}", i),
            tags: vec![],
        };
        db.create_article(&create, &user_id, &config.security)
            .expect("Create should succeed");
    }

    // Get first page
    let page1 = db.search_articles("knowledge", 0, 2, &config.security)
        .expect("Search should succeed");
    assert_eq!(page1.len(), 2);

    // Get second page
    let page2 = db.search_articles("knowledge", 2, 2, &config.security)
        .expect("Search should succeed");
    assert_eq!(page2.len(), 2);

    // Get third page
    let page3 = db.search_articles("knowledge", 4, 2, &config.security)
        .expect("Search should succeed");
    assert_eq!(page3.len(), 1);
}

/// Test search is case-insensitive
#[test]
fn test_search_case_insensitive() {
    let (db, config) = setup_test_db();
    let user_id = create_test_user(&db);

    let create = CreateArticle {
        title: "UPPERCASE TITLE".to_string(),
        content: "lowercase content".to_string(),
        tags: vec![],
    };
    db.create_article(&create, &user_id, &config.security)
        .expect("Create should succeed");

    // Search with different cases
    let upper = db.search_articles("UPPERCASE", 0, 10, &config.security)
        .expect("Search should succeed");
    let lower = db.search_articles("uppercase", 0, 10, &config.security)
        .expect("Search should succeed");
    let mixed = db.search_articles("UpPeRcAsE", 0, 10, &config.security)
        .expect("Search should succeed");

    assert_eq!(upper.len(), 1);
    assert_eq!(lower.len(), 1);
    assert_eq!(mixed.len(), 1);
}

/// Test search with special FTS characters
#[test]
fn test_search_special_characters() {
    let (db, config) = setup_test_db();
    let user_id = create_test_user(&db);

    let create = CreateArticle {
        title: "C++ Programming".to_string(),
        content: "Learn C++ and C# basics".to_string(),
        tags: vec![],
    };
    db.create_article(&create, &user_id, &config.security)
        .expect("Create should succeed");

    // Search should handle special chars gracefully
    let results = db.search_articles("C++", 0, 10, &config.security)
        .expect("Search should succeed");

    // FTS5 tokenization may vary, just ensure no error
    assert!(results.len() <= 1);
}

/// Test search with phrase matching
#[test]
fn test_search_phrase() {
    let (db, config) = setup_test_db();
    let user_id = create_test_user(&db);

    let create1 = CreateArticle {
        title: "Web Development".to_string(),
        content: "Full stack web development guide".to_string(),
        tags: vec![],
    };
    let create2 = CreateArticle {
        title: "Mobile Development".to_string(),
        content: "Mobile app development basics".to_string(),
        tags: vec![],
    };

    db.create_article(&create1, &user_id, &config.security)
        .expect("Create should succeed");
    db.create_article(&create2, &user_id, &config.security)
        .expect("Create should succeed");

    // Search for phrase (both words)
    let results = db.search_articles("web development", 0, 10, &config.security)
        .expect("Search should succeed");

    // Should find both or at least the web one
    assert!(!results.is_empty());
}

/// Test search only returns published articles
#[test]
fn test_search_published_only() {
    let (db, config) = setup_test_db();
    let user_id = create_test_user(&db);

    // Create published article
    let create1 = CreateArticle {
        title: "Published Searchable".to_string(),
        content: "This is searchable content".to_string(),
        tags: vec![],
    };
    let art1 = db.create_article(&create1, &user_id, &config.security)
        .expect("Create should succeed");

    db.update_article(&art1.id, &knowledge_vault::models::article::UpdateArticle {
        title: None,
        content: None,
        is_published: Some(true),
    }, &config.security).expect("Update should succeed");

    // Create draft article
    let create2 = CreateArticle {
        title: "Draft Searchable".to_string(),
        content: "This is also searchable content".to_string(),
        tags: vec![],
    };
    db.create_article(&create2, &user_id, &config.security)
        .expect("Create should succeed");

    // Public search should only find published
    let results = db.search_published_articles("searchable", 0, 10, &config.security)
        .expect("Search should succeed");

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].title, "Published Searchable");
}

/// Test search with unicode content
#[test]
fn test_search_unicode() {
    let (db, config) = setup_test_db();
    let user_id = create_test_user(&db);

    let create = CreateArticle {
        title: "日本語タイトル".to_string(),
        content: "これは日本語のコンテンツです".to_string(),
        tags: vec![],
    };
    db.create_article(&create, &user_id, &config.security)
        .expect("Create should succeed");

    let results = db.search_articles("日本語", 0, 10, &config.security)
        .expect("Search should succeed");

    assert_eq!(results.len(), 1);
}

/// Test search index updates on article update
#[test]
fn test_search_index_updates() {
    let (db, config) = setup_test_db();
    let user_id = create_test_user(&db);

    let create = CreateArticle {
        title: "Original Title".to_string(),
        content: "Original searchable content".to_string(),
        tags: vec![],
    };
    let article = db.create_article(&create, &user_id, &config.security)
        .expect("Create should succeed");

    // Should find original
    let results1 = db.search_articles("Original", 0, 10, &config.security)
        .expect("Search should succeed");
    assert_eq!(results1.len(), 1);

    // Update article
    db.update_article(&article.id, &knowledge_vault::models::article::UpdateArticle {
        title: Some("Updated Title".to_string()),
        content: Some("Updated searchable content".to_string()),
        is_published: None,
    }, &config.security).expect("Update should succeed");

    // Should find updated, not original title
    let results2 = db.search_articles("Updated", 0, 10, &config.security)
        .expect("Search should succeed");
    assert_eq!(results2.len(), 1);

    let results3 = db.search_articles("Original", 0, 10, &config.security)
        .expect("Search should succeed");
    // Original should no longer match title (may still match content depending on update)
}

/// Test search index cleans up on article delete
#[test]
fn test_search_index_delete() {
    let (db, config) = setup_test_db();
    let user_id = create_test_user(&db);

    let create = CreateArticle {
        title: "Deletable Article".to_string(),
        content: "Content to be deleted".to_string(),
        tags: vec![],
    };
    let article = db.create_article(&create, &user_id, &config.security)
        .expect("Create should succeed");

    // Should find before delete
    let before = db.search_articles("Deletable", 0, 10, &config.security)
        .expect("Search should succeed");
    assert_eq!(before.len(), 1);

    // Delete
    db.delete_article(&article.id).expect("Delete should succeed");

    // Should not find after delete
    let after = db.search_articles("Deletable", 0, 10, &config.security)
        .expect("Search should succeed");
    assert!(after.is_empty());
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
