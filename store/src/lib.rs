use std::path::Path;

use anyhow::Context;
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, Row};
use salsa_core::model::{
    AppRule, Bundle, CaseMode, ContentType, DelimiterMode, ExpansionHistory, Profile, ScopeRule,
    Snippet,
};
use uuid::Uuid;

mod migrations;

pub struct Store {
    conn: Connection,
}

impl Store {
    pub fn open(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let conn = Connection::open(path)?;
        conn.pragma_update(None, "foreign_keys", "ON")?;
        apply_migrations(&conn)?;
        Ok(Self { conn })
    }

    pub fn connection(&self) -> &Connection {
        &self.conn
    }

    pub fn insert_snippet(&self, snippet: &Snippet) -> anyhow::Result<()> {
        let delimiter_mode = delimiter_mode_label(&snippet.delimiter_mode);
        let delimiter_custom = match &snippet.delimiter_mode {
            DelimiterMode::Custom(chars) => Some(chars.clone()),
            _ => None,
        };

        self.conn.execute(
            "INSERT INTO snippets (id, trigger, label, content, content_type, tags, enabled, case_mode, delimiter_mode, delimiter_custom, scope_profile_id, priority, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
            params![
                snippet.id.to_string(),
                snippet.trigger,
                snippet.label,
                snippet.content,
                content_type_label(&snippet.content_type),
                serde_json::to_string(&snippet.tags)?,
                snippet.enabled as i32,
                case_mode_label(&snippet.case_mode),
                delimiter_mode,
                delimiter_custom,
                snippet.scope.profile_id.map(|id| id.to_string()),
                snippet.priority,
                snippet.created_at.to_rfc3339(),
                snippet.updated_at.to_rfc3339(),
            ],
        )?;

        for rule in &snippet.scope.app_rules {
            self.insert_app_rule(snippet.id, rule)?;
        }

        Ok(())
    }

    pub fn list_snippets(&self) -> anyhow::Result<Vec<Snippet>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, trigger, label, content, content_type, tags, enabled, case_mode, delimiter_mode, delimiter_custom, scope_profile_id, priority, created_at, updated_at
             FROM snippets",
        )?;

        let mut snippets = Vec::new();
        let rows = stmt.query_map([], |row| map_snippet_row(row))?;
        for row in rows {
            let mut snippet = row?;
            snippet.scope.app_rules = self.list_app_rules(snippet.id)?;
            snippets.push(snippet);
        }

        Ok(snippets)
    }

    pub fn update_snippet(&self, snippet: &Snippet) -> anyhow::Result<()> {
        let delimiter_mode = delimiter_mode_label(&snippet.delimiter_mode);
        let delimiter_custom = match &snippet.delimiter_mode {
            DelimiterMode::Custom(chars) => Some(chars.clone()),
            _ => None,
        };

        self.conn.execute(
            "UPDATE snippets
             SET trigger = ?1,
                 label = ?2,
                 content = ?3,
                 content_type = ?4,
                 tags = ?5,
                 enabled = ?6,
                 case_mode = ?7,
                 delimiter_mode = ?8,
                 delimiter_custom = ?9,
                 scope_profile_id = ?10,
                 priority = ?11,
                 updated_at = ?12
             WHERE id = ?13",
            params![
                snippet.trigger,
                snippet.label,
                snippet.content,
                content_type_label(&snippet.content_type),
                serde_json::to_string(&snippet.tags)?,
                snippet.enabled as i32,
                case_mode_label(&snippet.case_mode),
                delimiter_mode,
                delimiter_custom,
                snippet.scope.profile_id.map(|id| id.to_string()),
                snippet.priority,
                snippet.updated_at.to_rfc3339(),
                snippet.id.to_string(),
            ],
        )?;

        self.conn.execute(
            "DELETE FROM app_rules WHERE snippet_id = ?1",
            params![snippet.id.to_string()],
        )?;
        for rule in &snippet.scope.app_rules {
            self.insert_app_rule(snippet.id, rule)?;
        }

        Ok(())
    }

    pub fn delete_snippet(&self, snippet_id: Uuid) -> anyhow::Result<()> {
        self.conn.execute(
            "DELETE FROM snippets WHERE id = ?1",
            params![snippet_id.to_string()],
        )?;
        Ok(())
    }

    pub fn list_profiles(&self) -> anyhow::Result<Vec<Profile>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, enabled FROM profiles")?;
        let rows = stmt.query_map([], |row| {
            Ok(Profile {
                id: Uuid::parse_str(row.get::<_, String>(0)?.as_str())
                    .unwrap_or_else(|_| Uuid::new_v4()),
                name: row.get(1)?,
                enabled: row.get::<_, i64>(2)? != 0,
            })
        })?;

        let mut profiles = Vec::new();
        for row in rows {
            profiles.push(row?);
        }
        Ok(profiles)
    }

    pub fn list_bundles(&self) -> anyhow::Result<Vec<Bundle>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, description, enabled FROM bundles")?;
        let rows = stmt.query_map([], |row| {
            Ok(Bundle {
                id: Uuid::parse_str(row.get::<_, String>(0)?.as_str())
                    .unwrap_or_else(|_| Uuid::new_v4()),
                name: row.get(1)?,
                description: row.get(2)?,
                enabled: row.get::<_, i64>(3)? != 0,
            })
        })?;

        let mut bundles = Vec::new();
        for row in rows {
            bundles.push(row?);
        }
        Ok(bundles)
    }

    pub fn insert_profile(&self, profile: &Profile) -> anyhow::Result<()> {
        self.conn.execute(
            "INSERT INTO profiles (id, name, enabled) VALUES (?1, ?2, ?3)",
            params![profile.id.to_string(), profile.name, profile.enabled as i32],
        )?;
        Ok(())
    }

    pub fn insert_bundle(&self, bundle: &Bundle) -> anyhow::Result<()> {
        self.conn.execute(
            "INSERT INTO bundles (id, name, description, enabled) VALUES (?1, ?2, ?3, ?4)",
            params![
                bundle.id.to_string(),
                bundle.name,
                bundle.description,
                bundle.enabled as i32
            ],
        )?;
        Ok(())
    }

    pub fn insert_history(&self, history: &ExpansionHistory) -> anyhow::Result<()> {
        self.conn.execute(
            "INSERT INTO expansion_history (id, snippet_id, app_bundle_id, timestamp, retained_content)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                history.id.to_string(),
                history.snippet_id.to_string(),
                history.app_bundle_id,
                history.timestamp.to_rfc3339(),
                history.retained_content,
            ],
        )?;
        Ok(())
    }

    fn insert_app_rule(&self, snippet_id: Uuid, rule: &AppRule) -> anyhow::Result<()> {
        let rule_id = Uuid::new_v4();
        self.conn.execute(
            "INSERT INTO app_rules (id, snippet_id, bundle_id, window_title_pattern, enabled)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                rule_id.to_string(),
                snippet_id.to_string(),
                rule.bundle_id,
                rule.window_title_pattern,
                rule.enabled as i32,
            ],
        )?;
        Ok(())
    }

    fn list_app_rules(&self, snippet_id: Uuid) -> anyhow::Result<Vec<AppRule>> {
        let mut stmt = self.conn.prepare(
            "SELECT bundle_id, window_title_pattern, enabled FROM app_rules WHERE snippet_id = ?1",
        )?;
        let rows = stmt.query_map([snippet_id.to_string()], |row| {
            Ok(AppRule {
                bundle_id: row.get(0)?,
                window_title_pattern: row.get(1)?,
                enabled: row.get::<_, i64>(2)? != 0,
            })
        })?;

        let mut rules = Vec::new();
        for row in rows {
            rules.push(row?);
        }
        Ok(rules)
    }
}

fn map_snippet_row(row: &Row) -> rusqlite::Result<Snippet> {
    let id: String = row.get(0)?;
    let trigger: String = row.get(1)?;
    let label: String = row.get(2)?;
    let content: String = row.get(3)?;
    let content_type: String = row.get(4)?;
    let tags: String = row.get(5)?;
    let enabled: i64 = row.get(6)?;
    let case_mode: String = row.get(7)?;
    let delimiter_mode: String = row.get(8)?;
    let delimiter_custom: Option<String> = row.get(9)?;
    let scope_profile_id: Option<String> = row.get(10)?;
    let priority: i64 = row.get(11)?;
    let created_at: String = row.get(12)?;
    let updated_at: String = row.get(13)?;

    let tags_vec = serde_json::from_str(&tags).unwrap_or_default();

    let scope = ScopeRule {
        app_rules: Vec::new(),
        profile_id: scope_profile_id.and_then(|id| Uuid::parse_str(&id).ok()),
    };

    Ok(Snippet {
        id: Uuid::parse_str(&id).unwrap_or_else(|_| Uuid::new_v4()),
        trigger,
        label,
        content,
        content_type: parse_content_type(&content_type),
        tags: tags_vec,
        enabled: enabled != 0,
        case_mode: parse_case_mode(&case_mode),
        delimiter_mode: parse_delimiter_mode(&delimiter_mode, delimiter_custom),
        scope,
        priority: priority as i32,
        created_at: parse_datetime(&created_at),
        updated_at: parse_datetime(&updated_at),
    })
}

fn parse_datetime(value: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(value)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now())
}

fn content_type_label(value: &ContentType) -> &'static str {
    match value {
        ContentType::PlainText => "plain",
        ContentType::Markdown => "markdown",
    }
}

fn parse_content_type(value: &str) -> ContentType {
    match value {
        "markdown" => ContentType::Markdown,
        _ => ContentType::PlainText,
    }
}

fn case_mode_label(value: &CaseMode) -> &'static str {
    match value {
        CaseMode::Smart => "smart",
        CaseMode::Upper => "upper",
        CaseMode::Lower => "lower",
        CaseMode::Preserve => "preserve",
    }
}

fn parse_case_mode(value: &str) -> CaseMode {
    match value {
        "upper" => CaseMode::Upper,
        "lower" => CaseMode::Lower,
        "preserve" => CaseMode::Preserve,
        _ => CaseMode::Smart,
    }
}

fn delimiter_mode_label(value: &DelimiterMode) -> &'static str {
    match value {
        DelimiterMode::Any => "any",
        DelimiterMode::WordBoundary => "word",
        DelimiterMode::Custom(_) => "custom",
    }
}

fn parse_delimiter_mode(value: &str, custom: Option<String>) -> DelimiterMode {
    match value {
        "word" => DelimiterMode::WordBoundary,
        "custom" => DelimiterMode::Custom(custom.unwrap_or_default()),
        _ => DelimiterMode::Any,
    }
}

fn apply_migrations(conn: &Connection) -> anyhow::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS schema_migrations (version INTEGER PRIMARY KEY, applied_at TEXT NOT NULL)",
        [],
    )?;

    let mut stmt = conn.prepare("SELECT version FROM schema_migrations")?;
    let applied: Vec<i64> = stmt
        .query_map([], |row| row.get(0))?
        .collect::<Result<Vec<i64>, _>>()?;

    for migration in migrations::MIGRATIONS {
        if applied.contains(&migration.version) {
            continue;
        }

        conn.execute_batch(migration.sql)
            .with_context(|| format!("migration {} failed", migration.version))?;
        conn.execute(
            "INSERT INTO schema_migrations (version, applied_at) VALUES (?1, ?2)",
            params![migration.version, Utc::now().to_rfc3339()],
        )?;
    }

    Ok(())
}
