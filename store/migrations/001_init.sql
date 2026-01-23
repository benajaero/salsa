BEGIN;

CREATE TABLE IF NOT EXISTS schema_migrations (
    version INTEGER PRIMARY KEY,
    applied_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS snippets (
    id TEXT PRIMARY KEY,
    trigger TEXT NOT NULL,
    label TEXT NOT NULL,
    content TEXT NOT NULL,
    content_type TEXT NOT NULL,
    tags TEXT NOT NULL,
    enabled INTEGER NOT NULL,
    case_mode TEXT NOT NULL,
    delimiter_mode TEXT NOT NULL,
    delimiter_custom TEXT,
    scope_profile_id TEXT,
    priority INTEGER NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS app_rules (
    id TEXT PRIMARY KEY,
    snippet_id TEXT NOT NULL,
    bundle_id TEXT NOT NULL,
    window_title_pattern TEXT,
    enabled INTEGER NOT NULL,
    FOREIGN KEY (snippet_id) REFERENCES snippets (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS profiles (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    enabled INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS bundles (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    enabled INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS expansion_history (
    id TEXT PRIMARY KEY,
    snippet_id TEXT NOT NULL,
    app_bundle_id TEXT NOT NULL,
    timestamp TEXT NOT NULL,
    retained_content TEXT,
    FOREIGN KEY (snippet_id) REFERENCES snippets (id) ON DELETE CASCADE
);

COMMIT;
