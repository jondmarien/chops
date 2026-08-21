use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use serde_json::Value as JsonValue;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: String,
    pub file_path: String,
    pub resolved_path: String,
    pub tool_source: String,
    pub is_directory: bool,
    pub is_global: bool,
    pub name: String,
    pub description: Option<String>,
    pub content: Option<String>,
    pub frontmatter: Option<JsonValue>,
    pub installed_paths: Option<JsonValue>,
    pub tool_sources: Option<JsonValue>,
    pub file_modified_date: Option<DateTime<Utc>>,
    pub file_size: Option<i64>,
    pub item_kind: Option<String>, // "skill" | "agent" | "rule"
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    pub id: String,
    pub name: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
