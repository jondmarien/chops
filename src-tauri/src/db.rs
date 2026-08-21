pub mod models;

use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use anyhow::Result;

pub struct Db {
    pool: SqlitePool,
}

impl Db {
    pub async fn new(db_url: &str) -> Result<Self> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await?;
            
        let db = Self { pool };
        db.init_schema().await?;
        
        Ok(db)
    }

    async fn init_schema(&self) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS skills (
                id TEXT PRIMARY KEY,
                file_path TEXT NOT NULL,
                resolved_path TEXT UNIQUE NOT NULL,
                tool_source TEXT NOT NULL,
                is_directory BOOLEAN NOT NULL,
                is_global BOOLEAN NOT NULL,
                name TEXT NOT NULL,
                description TEXT,
                content TEXT,
                frontmatter JSON,
                installed_paths JSON,
                tool_sources JSON,
                file_modified_date DATETIME,
                file_size INTEGER,
                item_kind TEXT,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS collections (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS skill_collections (
                skill_id TEXT NOT NULL REFERENCES skills(id),
                collection_id TEXT NOT NULL REFERENCES collections(id),
                PRIMARY KEY (skill_id, collection_id)
            );

            CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );
            "#
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
