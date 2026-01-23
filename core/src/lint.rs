use std::collections::HashMap;

use crate::model::{AppRule, Snippet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LintKind {
    DuplicateTrigger,
    ShadowedByPriority,
}

#[derive(Debug, Clone)]
pub struct LintIssue {
    pub kind: LintKind,
    pub trigger: String,
    pub snippet_ids: Vec<uuid::Uuid>,
}

pub fn lint_snippets(snippets: &[Snippet]) -> Vec<LintIssue> {
    let mut issues = Vec::new();

    let mut trigger_map: HashMap<String, Vec<&Snippet>> = HashMap::new();
    for snippet in snippets {
        trigger_map
            .entry(snippet.trigger.clone())
            .or_default()
            .push(snippet);
    }

    for (trigger, group) in trigger_map {
        if group.len() <= 1 {
            continue;
        }

        let mut ids: Vec<uuid::Uuid> = group.iter().map(|s| s.id).collect();
        ids.sort();
        issues.push(LintIssue {
            kind: LintKind::DuplicateTrigger,
            trigger: trigger.clone(),
            snippet_ids: ids.clone(),
        });

        let mut sorted = group.clone();
        sorted.sort_by(|a, b| b.priority.cmp(&a.priority));
        if scopes_overlap(sorted[0], sorted[1]) {
            let ids = vec![sorted[0].id, sorted[1].id];
            issues.push(LintIssue {
                kind: LintKind::ShadowedByPriority,
                trigger,
                snippet_ids: ids,
            });
        }
    }

    issues
}

fn scopes_overlap(a: &Snippet, b: &Snippet) -> bool {
    if a.scope.app_rules.is_empty() || b.scope.app_rules.is_empty() {
        return true;
    }

    a.scope
        .app_rules
        .iter()
        .any(|rule| app_rule_overlaps(rule, &b.scope.app_rules))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{CaseMode, ContentType, DelimiterMode, ScopeRule};
    use chrono::Utc;
    use uuid::Uuid;

    fn snippet(trigger: &str, priority: i32) -> Snippet {
        Snippet {
            id: Uuid::new_v4(),
            trigger: trigger.to_string(),
            label: "label".to_string(),
            content: "content".to_string(),
            content_type: ContentType::PlainText,
            tags: vec![],
            enabled: true,
            case_mode: CaseMode::Preserve,
            delimiter_mode: DelimiterMode::Any,
            scope: ScopeRule {
                app_rules: vec![],
                profile_id: None,
            },
            priority,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn detects_duplicate_trigger() {
        let a = snippet(";sig", 0);
        let b = snippet(";sig", 1);
        let issues = lint_snippets(&[a, b]);
        assert!(issues.iter().any(|issue| issue.kind == LintKind::DuplicateTrigger));
    }
}

fn app_rule_overlaps(rule: &AppRule, others: &[AppRule]) -> bool {
    if !rule.enabled {
        return false;
    }

    others.iter().any(|other| {
        if !other.enabled || other.bundle_id != rule.bundle_id {
            return false;
        }

        match (&rule.window_title_pattern, &other.window_title_pattern) {
            (None, _) | (_, None) => true,
            (Some(a), Some(b)) => a == b,
        }
    })
}
