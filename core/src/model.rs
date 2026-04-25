//! Salsa data model — snippets, profiles, bundles, and expansion history.
//!
//! All identifiers use UUID v4. Timestamps are UTC. Serialised as JSON over IPC
//! and stored as SQLite rows via `salsa-store`.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A text-expansion snippet: trigger → expanded content.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snippet {
    pub id: Uuid,
    /// Short trigger text, e.g. `;addr`.
    pub trigger: String,
    /// Human-readable label for UI lists.
    pub label: String,
    /// Expanded output. May contain Sauce Variables like `{{date}}`.
    pub content: String,
    pub content_type: ContentType,
    /// Free-form tags for filtering and search.
    pub tags: Vec<String>,
    /// If `false`, the snippet is ignored by the matcher.
    pub enabled: bool,
    pub case_mode: CaseMode,
    pub delimiter_mode: DelimiterMode,
    /// App-scope and profile rules.
    pub scope: ScopeRule,
    /// Higher priority wins when triggers collide. Tie-break: newest updated_at, then UUID.
    pub priority: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Plain text (v1) or Markdown (v2+) snippet output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentType {
    PlainText,
    Markdown,
}

/// How trigger case is interpreted during matching.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CaseMode {
    /// Case-insensitive match.
    Smart,
    /// Trigger must be typed in UPPERCASE.
    Upper,
    /// Trigger must be typed in lowercase.
    Lower,
    /// Trigger must match character-for-character.
    Preserve,
}

/// What must precede the trigger for it to fire.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DelimiterMode {
    /// Preceding char must be a non-word character (or start of input).
    WordBoundary,
    /// No delimiter requirement.
    Any,
    /// One of the listed characters must precede the trigger.
    Custom(String),
}

/// Scope constraints for a snippet.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeRule {
    pub app_rules: Vec<AppRule>,
    /// If `Some`, snippet is only active in this profile.
    pub profile_id: Option<Uuid>,
}

/// Per-app scope rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppRule {
    /// macOS bundle identifier, e.g. `com.apple.mail`.
    pub bundle_id: String,
    /// Optional substring match on window title.
    pub window_title_pattern: Option<String>,
    pub enabled: bool,
}

/// A user-defined profile (Menu) that groups snippets.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub id: Uuid,
    pub name: String,
    pub enabled: bool,
}

/// A bundle (Jar) of snippets for import/export.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bundle {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub enabled: bool,
}

/// Record of an expansion event. `retained_content` is opt-in only.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpansionHistory {
    pub id: Uuid,
    pub snippet_id: Uuid,
    pub app_bundle_id: String,
    pub timestamp: DateTime<Utc>,
    /// Only populated when the user opts in to content retention.
    pub retained_content: Option<String>,
}
