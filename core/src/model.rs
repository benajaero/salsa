use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snippet {
    pub id: Uuid,
    pub trigger: String,
    pub label: String,
    pub content: String,
    pub content_type: ContentType,
    pub tags: Vec<String>,
    pub enabled: bool,
    pub case_mode: CaseMode,
    pub delimiter_mode: DelimiterMode,
    pub scope: ScopeRule,
    pub priority: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentType {
    PlainText,
    Markdown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CaseMode {
    Smart,
    Upper,
    Lower,
    Preserve,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DelimiterMode {
    WordBoundary,
    Any,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeRule {
    pub app_rules: Vec<AppRule>,
    pub profile_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppRule {
    pub bundle_id: String,
    pub window_title_pattern: Option<String>,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub id: Uuid,
    pub name: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bundle {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpansionHistory {
    pub id: Uuid,
    pub snippet_id: Uuid,
    pub app_bundle_id: String,
    pub timestamp: DateTime<Utc>,
    pub retained_content: Option<String>,
}
