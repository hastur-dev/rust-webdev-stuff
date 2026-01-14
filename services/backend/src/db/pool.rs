//! Database connection pool and operations
//!
//! Provides all database CRUD operations with encryption support.

use chrono::Utc;
use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use uuid::Uuid;

use crate::config::{Config, SecurityConfig};
use crate::error::{AppError, AppResult};
use crate::models::article::{Article, ArticleRaw, CreateArticle, UpdateArticle};
use crate::models::audit::{
    AuditAction, AuditEntry, AuditResourceType, CreateAuditEntry,
};
use crate::models::favorite::Favorite;
use crate::models::tag::Tag;
use crate::models::user::{CreateUser, UpdateUser, User, UserRole};
use crate::security::encryption;

/// Database wrapper with connection pool
pub struct Database {
    pool: Pool<SqliteConnectionManager>,
}

impl Database {
    /// Create a new database connection pool
    pub fn new(config: &Config) -> AppResult<Self> {
        let manager = SqliteConnectionManager::file(&config.database.path);
        let pool = Pool::builder()
            .max_size(config.database.max_connections)
            .build(manager)
            .map_err(|e| AppError::Database(format!("Failed to create pool: {}", e)))?;

        Ok(Database { pool })
    }

    /// Create an in-memory database (for testing)
    pub fn new_in_memory(_config: &Config) -> AppResult<Self> {
        let manager = SqliteConnectionManager::memory();
        let pool = Pool::builder()
            .max_size(1)
            .build(manager)
            .map_err(|e| AppError::Database(format!("Failed to create pool: {}", e)))?;

        Ok(Database { pool })
    }

    /// Get a connection from the pool
    fn conn(&self) -> AppResult<PooledConnection<SqliteConnectionManager>> {
        self.pool
            .get()
            .map_err(|e| AppError::Database(format!("Failed to get connection: {}", e)))
    }

    /// Run database migrations
    pub fn run_migrations(&self) -> AppResult<()> {
        let conn = self.conn()?;
        super::migrations::run_migrations(&conn)
    }

    // === User Operations ===

    /// Create a new user
    pub fn create_user(&self, user: &CreateUser) -> AppResult<User> {
        let conn = self.conn()?;
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO users (id, username, email, password_hash, role, is_active, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, 1, ?6, ?6)",
            params![id, user.username, user.email, user.password_hash, user.role.as_str(), now],
        )?;

        self.get_user_by_id(&id)?.ok_or_else(|| {
            AppError::Database("Failed to retrieve created user".to_string())
        })
    }

    /// Get user by ID
    pub fn get_user_by_id(&self, id: &str) -> AppResult<Option<User>> {
        let conn = self.conn()?;

        let result = conn.query_row(
            "SELECT id, username, email, password_hash, role, is_active, created_at, updated_at
             FROM users WHERE id = ?1",
            [id],
            |row| {
                Ok(User {
                    id: row.get(0)?,
                    username: row.get(1)?,
                    email: row.get(2)?,
                    password_hash: row.get(3)?,
                    role: UserRole::from_str(&row.get::<_, String>(4)?).unwrap_or(UserRole::Viewer),
                    is_active: row.get::<_, i32>(5)? == 1,
                    created_at: row.get(6)?,
                    updated_at: row.get(7)?,
                })
            },
        );

        match result {
            Ok(user) => Ok(Some(user)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(AppError::Database(e.to_string())),
        }
    }

    /// Get user by username
    pub fn get_user_by_username(&self, username: &str) -> AppResult<Option<User>> {
        let conn = self.conn()?;

        let result = conn.query_row(
            "SELECT id, username, email, password_hash, role, is_active, created_at, updated_at
             FROM users WHERE username = ?1",
            [username],
            |row| {
                Ok(User {
                    id: row.get(0)?,
                    username: row.get(1)?,
                    email: row.get(2)?,
                    password_hash: row.get(3)?,
                    role: UserRole::from_str(&row.get::<_, String>(4)?).unwrap_or(UserRole::Viewer),
                    is_active: row.get::<_, i32>(5)? == 1,
                    created_at: row.get(6)?,
                    updated_at: row.get(7)?,
                })
            },
        );

        match result {
            Ok(user) => Ok(Some(user)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(AppError::Database(e.to_string())),
        }
    }

    /// List all users
    pub fn list_users(&self, offset: i64, limit: i64) -> AppResult<Vec<User>> {
        let conn = self.conn()?;

        let mut stmt = conn.prepare(
            "SELECT id, username, email, password_hash, role, is_active, created_at, updated_at
             FROM users ORDER BY created_at DESC LIMIT ?1 OFFSET ?2",
        )?;

        let users = stmt
            .query_map([limit, offset], |row| {
                Ok(User {
                    id: row.get(0)?,
                    username: row.get(1)?,
                    email: row.get(2)?,
                    password_hash: row.get(3)?,
                    role: UserRole::from_str(&row.get::<_, String>(4)?).unwrap_or(UserRole::Viewer),
                    is_active: row.get::<_, i32>(5)? == 1,
                    created_at: row.get(6)?,
                    updated_at: row.get(7)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(users)
    }

    /// Update user
    pub fn update_user(&self, id: &str, update: &UpdateUser) -> AppResult<User> {
        let conn = self.conn()?;
        let now = Utc::now().to_rfc3339();

        if let Some(ref username) = update.username {
            conn.execute(
                "UPDATE users SET username = ?1, updated_at = ?2 WHERE id = ?3",
                params![username, now, id],
            )?;
        }
        if let Some(ref email) = update.email {
            conn.execute(
                "UPDATE users SET email = ?1, updated_at = ?2 WHERE id = ?3",
                params![email, now, id],
            )?;
        }
        if let Some(ref role) = update.role {
            conn.execute(
                "UPDATE users SET role = ?1, updated_at = ?2 WHERE id = ?3",
                params![role.as_str(), now, id],
            )?;
        }
        if let Some(is_active) = update.is_active {
            conn.execute(
                "UPDATE users SET is_active = ?1, updated_at = ?2 WHERE id = ?3",
                params![if is_active { 1 } else { 0 }, now, id],
            )?;
        }

        self.get_user_by_id(id)?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))
    }

    // === Article Operations ===

    /// Create a new article with encryption
    pub fn create_article(
        &self,
        article: &CreateArticle,
        author_id: &str,
        security: &SecurityConfig,
    ) -> AppResult<Article> {
        let conn = self.conn()?;
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        // Encrypt content
        let (encrypted, nonce) = encryption::encrypt_string(&article.content, security)?;

        conn.execute(
            "INSERT INTO articles (id, title, content_encrypted, content_nonce, author_id, is_published, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, 0, ?6, ?6)",
            params![id, article.title, encrypted, nonce, author_id, now],
        )?;

        // Update FTS index
        conn.execute(
            "INSERT INTO articles_fts (article_id, title, content) VALUES (?1, ?2, ?3)",
            params![id, article.title, article.content],
        )?;

        // Add tags
        for tag_id in &article.tags {
            let _ = conn.execute(
                "INSERT OR IGNORE INTO article_tags (article_id, tag_id) VALUES (?1, ?2)",
                params![id, tag_id],
            );
        }

        Ok(Article {
            id,
            title: article.title.clone(),
            content: article.content.clone(),
            author_id: author_id.to_string(),
            is_published: false,
            created_at: now.clone(),
            updated_at: now,
        })
    }

    /// Get article by ID (decrypted)
    pub fn get_article(&self, id: &str, security: &SecurityConfig) -> AppResult<Option<Article>> {
        let raw = self.get_article_raw(id)?;

        match raw {
            Some(raw) => {
                let content =
                    encryption::decrypt_string(&raw.content_encrypted, &raw.content_nonce, security)?;

                Ok(Some(Article {
                    id: raw.id,
                    title: raw.title,
                    content,
                    author_id: raw.author_id,
                    is_published: raw.is_published,
                    created_at: raw.created_at,
                    updated_at: raw.updated_at,
                }))
            }
            None => Ok(None),
        }
    }

    /// Get raw article (encrypted)
    pub fn get_article_raw(&self, id: &str) -> AppResult<Option<ArticleRaw>> {
        let conn = self.conn()?;

        let result = conn.query_row(
            "SELECT id, title, content_encrypted, content_nonce, author_id, is_published, created_at, updated_at
             FROM articles WHERE id = ?1",
            [id],
            |row| {
                Ok(ArticleRaw {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    content_encrypted: row.get(2)?,
                    content_nonce: row.get(3)?,
                    author_id: row.get(4)?,
                    is_published: row.get::<_, i32>(5)? == 1,
                    created_at: row.get(6)?,
                    updated_at: row.get(7)?,
                })
            },
        );

        match result {
            Ok(article) => Ok(Some(article)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(AppError::Database(e.to_string())),
        }
    }

    /// List articles (decrypted)
    pub fn list_articles(
        &self,
        offset: i64,
        limit: i64,
        security: &SecurityConfig,
    ) -> AppResult<Vec<Article>> {
        let conn = self.conn()?;

        let mut stmt = conn.prepare(
            "SELECT id, title, content_encrypted, content_nonce, author_id, is_published, created_at, updated_at
             FROM articles ORDER BY created_at DESC LIMIT ?1 OFFSET ?2",
        )?;

        let raws: Vec<ArticleRaw> = stmt
            .query_map([limit, offset], |row| {
                Ok(ArticleRaw {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    content_encrypted: row.get(2)?,
                    content_nonce: row.get(3)?,
                    author_id: row.get(4)?,
                    is_published: row.get::<_, i32>(5)? == 1,
                    created_at: row.get(6)?,
                    updated_at: row.get(7)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        let mut articles = Vec::with_capacity(raws.len());
        for raw in raws {
            let content =
                encryption::decrypt_string(&raw.content_encrypted, &raw.content_nonce, security)?;
            articles.push(Article {
                id: raw.id,
                title: raw.title,
                content,
                author_id: raw.author_id,
                is_published: raw.is_published,
                created_at: raw.created_at,
                updated_at: raw.updated_at,
            });
        }

        Ok(articles)
    }

    /// List published articles only
    pub fn list_published_articles(
        &self,
        offset: i64,
        limit: i64,
        security: &SecurityConfig,
    ) -> AppResult<Vec<Article>> {
        let conn = self.conn()?;

        let mut stmt = conn.prepare(
            "SELECT id, title, content_encrypted, content_nonce, author_id, is_published, created_at, updated_at
             FROM articles WHERE is_published = 1 ORDER BY created_at DESC LIMIT ?1 OFFSET ?2",
        )?;

        let raws: Vec<ArticleRaw> = stmt
            .query_map([limit, offset], |row| {
                Ok(ArticleRaw {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    content_encrypted: row.get(2)?,
                    content_nonce: row.get(3)?,
                    author_id: row.get(4)?,
                    is_published: row.get::<_, i32>(5)? == 1,
                    created_at: row.get(6)?,
                    updated_at: row.get(7)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        let mut articles = Vec::with_capacity(raws.len());
        for raw in raws {
            let content =
                encryption::decrypt_string(&raw.content_encrypted, &raw.content_nonce, security)?;
            articles.push(Article {
                id: raw.id,
                title: raw.title,
                content,
                author_id: raw.author_id,
                is_published: raw.is_published,
                created_at: raw.created_at,
                updated_at: raw.updated_at,
            });
        }

        Ok(articles)
    }

    /// Update article with re-encryption
    pub fn update_article(
        &self,
        id: &str,
        update: &UpdateArticle,
        security: &SecurityConfig,
    ) -> AppResult<Article> {
        let conn = self.conn()?;
        let now = Utc::now().to_rfc3339();

        let current = self
            .get_article(id, security)?
            .ok_or_else(|| AppError::NotFound("Article not found".to_string()))?;

        let new_title = update.title.as_ref().unwrap_or(&current.title);
        let new_content = update.content.as_ref().unwrap_or(&current.content);
        let new_published = update.is_published.unwrap_or(current.is_published);

        // Re-encrypt content
        let (encrypted, nonce) = encryption::encrypt_string(new_content, security)?;

        conn.execute(
            "UPDATE articles SET title = ?1, content_encrypted = ?2, content_nonce = ?3, is_published = ?4, updated_at = ?5 WHERE id = ?6",
            params![new_title, encrypted, nonce, if new_published { 1 } else { 0 }, now, id],
        )?;

        // Update FTS index
        conn.execute(
            "DELETE FROM articles_fts WHERE article_id = ?1",
            params![id],
        )?;
        conn.execute(
            "INSERT INTO articles_fts (article_id, title, content) VALUES (?1, ?2, ?3)",
            params![id, new_title, new_content],
        )?;

        self.get_article(id, security)?
            .ok_or_else(|| AppError::NotFound("Article not found".to_string()))
    }

    /// Delete article
    pub fn delete_article(&self, id: &str) -> AppResult<()> {
        let conn = self.conn()?;

        // Delete from FTS
        conn.execute(
            "DELETE FROM articles_fts WHERE article_id = ?1",
            params![id],
        )?;

        // Delete article (cascade deletes tags and favorites)
        conn.execute("DELETE FROM articles WHERE id = ?1", params![id])?;

        Ok(())
    }

    // === Search Operations ===

    /// Search articles using FTS5
    pub fn search_articles(
        &self,
        query: &str,
        offset: i64,
        limit: i64,
        security: &SecurityConfig,
    ) -> AppResult<Vec<Article>> {
        if query.trim().is_empty() {
            return Ok(vec![]);
        }

        let conn = self.conn()?;

        // Escape FTS special characters
        let safe_query = query.replace('"', "\"\"");

        let mut stmt = conn.prepare(
            "SELECT a.id, a.title, a.content_encrypted, a.content_nonce, a.author_id, a.is_published, a.created_at, a.updated_at
             FROM articles a
             JOIN articles_fts f ON a.id = f.article_id
             WHERE articles_fts MATCH ?1
             ORDER BY rank
             LIMIT ?2 OFFSET ?3",
        )?;

        let raws: Vec<ArticleRaw> = stmt
            .query_map(params![safe_query, limit, offset], |row| {
                Ok(ArticleRaw {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    content_encrypted: row.get(2)?,
                    content_nonce: row.get(3)?,
                    author_id: row.get(4)?,
                    is_published: row.get::<_, i32>(5)? == 1,
                    created_at: row.get(6)?,
                    updated_at: row.get(7)?,
                })
            })?
            .filter_map(|r| r.ok())
            .collect();

        let mut articles = Vec::with_capacity(raws.len());
        for raw in raws {
            if let Ok(content) =
                encryption::decrypt_string(&raw.content_encrypted, &raw.content_nonce, security)
            {
                articles.push(Article {
                    id: raw.id,
                    title: raw.title,
                    content,
                    author_id: raw.author_id,
                    is_published: raw.is_published,
                    created_at: raw.created_at,
                    updated_at: raw.updated_at,
                });
            }
        }

        Ok(articles)
    }

    /// Search published articles only
    pub fn search_published_articles(
        &self,
        query: &str,
        offset: i64,
        limit: i64,
        security: &SecurityConfig,
    ) -> AppResult<Vec<Article>> {
        if query.trim().is_empty() {
            return Ok(vec![]);
        }

        let conn = self.conn()?;
        let safe_query = query.replace('"', "\"\"");

        let mut stmt = conn.prepare(
            "SELECT a.id, a.title, a.content_encrypted, a.content_nonce, a.author_id, a.is_published, a.created_at, a.updated_at
             FROM articles a
             JOIN articles_fts f ON a.id = f.article_id
             WHERE articles_fts MATCH ?1 AND a.is_published = 1
             ORDER BY rank
             LIMIT ?2 OFFSET ?3",
        )?;

        let raws: Vec<ArticleRaw> = stmt
            .query_map(params![safe_query, limit, offset], |row| {
                Ok(ArticleRaw {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    content_encrypted: row.get(2)?,
                    content_nonce: row.get(3)?,
                    author_id: row.get(4)?,
                    is_published: row.get::<_, i32>(5)? == 1,
                    created_at: row.get(6)?,
                    updated_at: row.get(7)?,
                })
            })?
            .filter_map(|r| r.ok())
            .collect();

        let mut articles = Vec::with_capacity(raws.len());
        for raw in raws {
            if let Ok(content) =
                encryption::decrypt_string(&raw.content_encrypted, &raw.content_nonce, security)
            {
                articles.push(Article {
                    id: raw.id,
                    title: raw.title,
                    content,
                    author_id: raw.author_id,
                    is_published: raw.is_published,
                    created_at: raw.created_at,
                    updated_at: raw.updated_at,
                });
            }
        }

        Ok(articles)
    }

    // === Tag Operations ===

    /// Create a new tag
    pub fn create_tag(&self, name: &str) -> AppResult<Tag> {
        let conn = self.conn()?;
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO tags (id, name, created_at) VALUES (?1, ?2, ?3)",
            params![id, name, now],
        )?;

        Ok(Tag {
            id,
            name: name.to_string(),
            created_at: now,
        })
    }

    /// Get tags for an article
    pub fn get_article_tags(&self, article_id: &str) -> AppResult<Vec<Tag>> {
        let conn = self.conn()?;

        let mut stmt = conn.prepare(
            "SELECT t.id, t.name, t.created_at
             FROM tags t
             JOIN article_tags at ON t.id = at.tag_id
             WHERE at.article_id = ?1",
        )?;

        let tags = stmt
            .query_map([article_id], |row| {
                Ok(Tag {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    created_at: row.get(2)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(tags)
    }

    /// List all tags
    pub fn list_tags(&self) -> AppResult<Vec<Tag>> {
        let conn = self.conn()?;

        let mut stmt = conn.prepare(
            "SELECT id, name, created_at FROM tags ORDER BY name",
        )?;

        let tags = stmt
            .query_map([], |row| {
                Ok(Tag {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    created_at: row.get(2)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(tags)
    }

    /// Delete a tag
    pub fn delete_tag(&self, id: &str) -> AppResult<()> {
        let conn = self.conn()?;
        conn.execute("DELETE FROM tags WHERE id = ?1", params![id])?;
        Ok(())
    }

    /// Add tag to article
    pub fn add_tag_to_article(&self, article_id: &str, tag_id: &str) -> AppResult<()> {
        let conn = self.conn()?;
        conn.execute(
            "INSERT OR IGNORE INTO article_tags (article_id, tag_id) VALUES (?1, ?2)",
            params![article_id, tag_id],
        )?;
        Ok(())
    }

    // === Favorite Operations ===

    /// Add favorite
    pub fn add_favorite(&self, user_id: &str, article_id: &str) -> AppResult<Favorite> {
        let conn = self.conn()?;
        let now = Utc::now().to_rfc3339();

        conn.execute(
            "INSERT OR IGNORE INTO favorites (user_id, article_id, created_at) VALUES (?1, ?2, ?3)",
            params![user_id, article_id, now],
        )?;

        Ok(Favorite {
            user_id: user_id.to_string(),
            article_id: article_id.to_string(),
            created_at: now,
        })
    }

    /// Remove favorite
    pub fn remove_favorite(&self, user_id: &str, article_id: &str) -> AppResult<()> {
        let conn = self.conn()?;
        conn.execute(
            "DELETE FROM favorites WHERE user_id = ?1 AND article_id = ?2",
            params![user_id, article_id],
        )?;
        Ok(())
    }

    /// Get user's favorites
    pub fn get_user_favorites(&self, user_id: &str) -> AppResult<Vec<Favorite>> {
        let conn = self.conn()?;

        let mut stmt = conn.prepare(
            "SELECT user_id, article_id, created_at FROM favorites WHERE user_id = ?1 ORDER BY created_at DESC",
        )?;

        let favorites = stmt
            .query_map([user_id], |row| {
                Ok(Favorite {
                    user_id: row.get(0)?,
                    article_id: row.get(1)?,
                    created_at: row.get(2)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(favorites)
    }

    /// Check if article is favorited
    pub fn is_favorited(&self, user_id: &str, article_id: &str) -> AppResult<bool> {
        let conn = self.conn()?;

        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM favorites WHERE user_id = ?1 AND article_id = ?2",
            params![user_id, article_id],
            |row| row.get(0),
        )?;

        Ok(count > 0)
    }

    // === Audit Operations ===

    /// Create audit entry
    pub fn create_audit_entry(&self, entry: &CreateAuditEntry) -> AppResult<AuditEntry> {
        let conn = self.conn()?;
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO audit_log (id, user_id, action, resource_type, resource_id, details, ip_address, user_agent, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                id,
                entry.user_id,
                entry.action.as_str(),
                entry.resource_type.as_str(),
                entry.resource_id,
                entry.details,
                entry.ip_address,
                entry.user_agent,
                now
            ],
        )?;

        Ok(AuditEntry {
            id,
            user_id: entry.user_id.clone(),
            action: entry.action,
            resource_type: entry.resource_type,
            resource_id: entry.resource_id.clone(),
            details: entry.details.clone(),
            ip_address: entry.ip_address.clone(),
            user_agent: entry.user_agent.clone(),
            created_at: now,
        })
    }

    /// List audit entries with filtering
    pub fn list_audit_entries(
        &self,
        offset: i64,
        limit: i64,
        user_id: Option<&str>,
        action: Option<AuditAction>,
        resource_type: Option<AuditResourceType>,
    ) -> AppResult<Vec<AuditEntry>> {
        let conn = self.conn()?;

        let mut sql = String::from(
            "SELECT id, user_id, action, resource_type, resource_id, details, ip_address, user_agent, created_at
             FROM audit_log WHERE 1=1",
        );
        let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = vec![];

        if let Some(uid) = user_id {
            sql.push_str(" AND user_id = ?");
            params_vec.push(Box::new(uid.to_string()));
        }
        if let Some(act) = action {
            sql.push_str(" AND action = ?");
            params_vec.push(Box::new(act.as_str().to_string()));
        }
        if let Some(rt) = resource_type {
            sql.push_str(" AND resource_type = ?");
            params_vec.push(Box::new(rt.as_str().to_string()));
        }

        sql.push_str(" ORDER BY created_at DESC LIMIT ? OFFSET ?");
        params_vec.push(Box::new(limit));
        params_vec.push(Box::new(offset));

        let mut stmt = conn.prepare(&sql)?;
        let params_refs: Vec<&dyn rusqlite::ToSql> = params_vec.iter().map(|p| p.as_ref()).collect();

        let entries = stmt
            .query_map(params_refs.as_slice(), |row| {
                Ok(AuditEntry {
                    id: row.get(0)?,
                    user_id: row.get(1)?,
                    action: AuditAction::from_str(&row.get::<_, String>(2)?).unwrap_or(AuditAction::Read),
                    resource_type: AuditResourceType::from_str(&row.get::<_, String>(3)?).unwrap_or(AuditResourceType::System),
                    resource_id: row.get(4)?,
                    details: row.get(5)?,
                    ip_address: row.get(6)?,
                    user_agent: row.get(7)?,
                    created_at: row.get(8)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(entries)
    }

    /// Count audit entries
    pub fn count_audit_entries(
        &self,
        user_id: Option<&str>,
        action: Option<AuditAction>,
        resource_type: Option<AuditResourceType>,
    ) -> AppResult<i64> {
        let conn = self.conn()?;

        let mut sql = String::from("SELECT COUNT(*) FROM audit_log WHERE 1=1");
        let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = vec![];

        if let Some(uid) = user_id {
            sql.push_str(" AND user_id = ?");
            params_vec.push(Box::new(uid.to_string()));
        }
        if let Some(act) = action {
            sql.push_str(" AND action = ?");
            params_vec.push(Box::new(act.as_str().to_string()));
        }
        if let Some(rt) = resource_type {
            sql.push_str(" AND resource_type = ?");
            params_vec.push(Box::new(rt.as_str().to_string()));
        }

        let params_refs: Vec<&dyn rusqlite::ToSql> = params_vec.iter().map(|p| p.as_ref()).collect();
        let count: i64 = conn.query_row(&sql, params_refs.as_slice(), |row| row.get(0))?;

        Ok(count)
    }

    /// Count users
    pub fn count_users(&self) -> AppResult<i64> {
        let conn = self.conn()?;
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM users", [], |row| row.get(0))?;
        Ok(count)
    }
}
