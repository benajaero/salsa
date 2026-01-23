BEGIN;

CREATE INDEX IF NOT EXISTS idx_snippets_trigger ON snippets(trigger);
CREATE INDEX IF NOT EXISTS idx_snippets_enabled ON snippets(enabled);
CREATE INDEX IF NOT EXISTS idx_app_rules_snippet_id ON app_rules(snippet_id);
CREATE INDEX IF NOT EXISTS idx_app_rules_bundle_id ON app_rules(bundle_id);
CREATE INDEX IF NOT EXISTS idx_history_snippet_id ON expansion_history(snippet_id);

COMMIT;
