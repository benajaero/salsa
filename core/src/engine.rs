//! Deterministic trigger matcher.
//!
//! Uses a pair of reverse tries (exact + case-folded) to find the best matching
//! snippet for a typed buffer. Match is O(trigger length) in the worst case,
//! and O(1) average when no prefix matches.

use std::collections::HashMap;

use crate::model::{CaseMode, DelimiterMode, Snippet};

/// Per-keystroke context used during matching.
#[derive(Debug, Clone)]
pub struct MatchContext {
    pub app_bundle_id: String,
    pub window_title: Option<String>,
    pub is_secure: bool,
    pub ime_active: bool,
}

/// Result of a successful match: which snippet fired and what to expand to.
#[derive(Debug, Clone)]
pub struct MatchResult {
    pub snippet_id: uuid::Uuid,
    pub trigger: String,
    pub content: String,
}

/// Trie-backed matcher with priority ordering and scope checks.
pub struct Matcher {
    snippets: Vec<Snippet>,
    exact_trie: Vec<TrieNode>,
    folded_trie: Vec<TrieNode>,
    max_trigger_len: usize,
}

impl Matcher {
    /// Build a matcher from a list of snippets. Sorts by priority internally.
    pub fn from_snippets(mut snippets: Vec<Snippet>) -> Self {
        snippets.sort_by(|a, b| compare_snippets(a, b));

        let mut exact_trie = vec![TrieNode::default()];
        let mut folded_trie = vec![TrieNode::default()];
        let mut max_trigger_len = 0;
        for (idx, snippet) in snippets.iter().enumerate() {
            let trigger_len = snippet.trigger.chars().count();
            max_trigger_len = max_trigger_len.max(trigger_len);

            let (trie, trigger_chars): (&mut Vec<TrieNode>, Vec<char>) = if matches!(
                snippet.case_mode,
                CaseMode::Preserve
            ) {
                (&mut exact_trie, snippet.trigger.chars().collect())
            } else {
                (
                    &mut folded_trie,
                    snippet.trigger.to_lowercase().chars().collect(),
                )
            };

            let mut node_idx = 0;
            for ch in trigger_chars.into_iter().rev() {
                let next_idx = if let Some(&existing) = trie[node_idx].children.get(&ch) {
                    existing
                } else {
                    let new_idx = trie.len();
                    trie.push(TrieNode::default());
                    trie[node_idx].children.insert(ch, new_idx);
                    new_idx
                };
                node_idx = next_idx;
            }
            trie[node_idx].terminals.push(idx);
        }

        Self {
            snippets,
            exact_trie,
            folded_trie,
            max_trigger_len,
        }
    }

    /// Attempt to match the tail of `buffer` against loaded snippets.
    ///
    /// Returns `None` when the context is secure, IME is active, or no trigger matches.
    pub fn match_buffer(&self, buffer: &str, ctx: &MatchContext) -> Option<MatchResult> {
        if ctx.is_secure || ctx.ime_active {
            return None;
        }

        let buffer_chars: Vec<char> = buffer.chars().collect();
        let total_len = buffer_chars.len();
        let max_len = self.max_trigger_len.min(total_len);
        let buffer_lower: Vec<char> = buffer_chars
            .iter()
            .map(|ch| ch.to_ascii_lowercase())
            .collect();

        match_with_trie(
            &self.exact_trie,
            &self.snippets,
            &buffer_chars,
            &buffer_chars,
            max_len,
            ctx,
            true,
        )
        .or_else(|| {
            match_with_trie(
                &self.folded_trie,
                &self.snippets,
                &buffer_chars,
                &buffer_lower,
                max_len,
                ctx,
                false,
            )
        })
    }
}

#[derive(Default)]
struct TrieNode {
    children: HashMap<char, usize>,
    terminals: Vec<usize>,
}

fn compare_snippets(a: &Snippet, b: &Snippet) -> std::cmp::Ordering {
    b.priority
        .cmp(&a.priority)
        .then_with(|| b.updated_at.cmp(&a.updated_at))
        .then_with(|| a.id.cmp(&b.id))
}

fn matches_case_mode(suffix: &str, mode: &CaseMode, trigger: &str) -> bool {
    match mode {
        CaseMode::Smart => suffix.to_lowercase() == trigger.to_lowercase(),
        CaseMode::Upper => suffix == trigger.to_uppercase(),
        CaseMode::Lower => suffix == trigger.to_lowercase(),
        CaseMode::Preserve => suffix == trigger,
    }
}

fn matches_delimiter(delimiter_char: Option<char>, mode: &DelimiterMode) -> bool {
    match mode {
        DelimiterMode::Any => true,
        DelimiterMode::WordBoundary => {
            delimiter_char.map_or(true, |ch| !is_word_char(ch))
        }
        DelimiterMode::Custom(chars) => {
            delimiter_char.map_or(false, |ch| chars.contains(ch))
        }
    }
}

fn matches_scope(snippet: &Snippet, ctx: &MatchContext) -> bool {
    if snippet.scope.app_rules.is_empty() {
        return true;
    }

    snippet.scope.app_rules.iter().any(|rule| {
        if !rule.enabled || rule.bundle_id != ctx.app_bundle_id {
            return false;
        }

        match (&rule.window_title_pattern, &ctx.window_title) {
            (None, _) => true,
            (Some(pattern), Some(title)) => title.contains(pattern),
            _ => false,
        }
    })
}

fn is_word_char(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_'
}

fn match_with_trie(
    trie: &[TrieNode],
    snippets: &[Snippet],
    buffer_chars: &[char],
    match_chars: &[char],
    max_len: usize,
    ctx: &MatchContext,
    preserve_only: bool,
) -> Option<MatchResult> {
    let total_len = match_chars.len();
    if total_len == 0 {
        return None;
    }

    let mut node_idx = 0;
    for offset in 0..max_len {
        let ch = match_chars[total_len - 1 - offset];
        let next_idx = match trie[node_idx].children.get(&ch) {
            Some(idx) => *idx,
            None => break,
        };
        node_idx = next_idx;

        let trigger_len = offset + 1;
        if trie[node_idx].terminals.is_empty() {
            continue;
        }

        let start = buffer_chars.len().saturating_sub(trigger_len);
        let suffix: String = buffer_chars[start..].iter().collect();
        let delimiter_char = if start == 0 {
            None
        } else {
            buffer_chars.get(start - 1).copied()
        };

        for &idx in &trie[node_idx].terminals {
            let snippet = &snippets[idx];
            if !snippet.enabled {
                continue;
            }
            if preserve_only && !matches!(snippet.case_mode, CaseMode::Preserve) {
                continue;
            }
            if !preserve_only && matches!(snippet.case_mode, CaseMode::Preserve) {
                continue;
            }
            if snippet.trigger.chars().count() != trigger_len {
                continue;
            }
            if !matches_scope(snippet, ctx) {
                continue;
            }
            if !matches_delimiter(delimiter_char, &snippet.delimiter_mode) {
                continue;
            }
            if !matches_case_mode(&suffix, &snippet.case_mode, &snippet.trigger) {
                continue;
            }

            return Some(MatchResult {
                snippet_id: snippet.id,
                trigger: snippet.trigger.clone(),
                content: snippet.content.clone(),
            });
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{AppRule, ContentType, DelimiterMode, ScopeRule};
    use chrono::Utc;
    use uuid::Uuid;

    fn snippet(trigger: &str, content: &str) -> Snippet {
        Snippet {
            id: Uuid::new_v4(),
            trigger: trigger.to_string(),
            label: "label".to_string(),
            content: content.to_string(),
            content_type: ContentType::PlainText,
            tags: vec![],
            enabled: true,
            case_mode: CaseMode::Preserve,
            delimiter_mode: DelimiterMode::Any,
            scope: ScopeRule {
                app_rules: vec![],
                profile_id: None,
            },
            priority: 0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn matches_basic_trigger() {
        let s = snippet(";addr", "123 Main St");
        let matcher = Matcher::from_snippets(vec![s]);
        let ctx = MatchContext {
            app_bundle_id: "com.example".to_string(),
            window_title: None,
            is_secure: false,
            ime_active: false,
        };
        let result = matcher.match_buffer("please ;addr", &ctx);
        assert!(result.is_some());
    }

    #[test]
    fn skips_secure_context() {
        let s = snippet(";addr", "123 Main St");
        let matcher = Matcher::from_snippets(vec![s]);
        let ctx = MatchContext {
            app_bundle_id: "com.example".to_string(),
            window_title: None,
            is_secure: true,
            ime_active: false,
        };
        let result = matcher.match_buffer(";addr", &ctx);
        assert!(result.is_none());
    }

    #[test]
    fn respects_word_boundary() {
        let mut s = snippet("sig", "Best,");
        s.delimiter_mode = DelimiterMode::WordBoundary;
        let matcher = Matcher::from_snippets(vec![s]);
        let ctx = MatchContext {
            app_bundle_id: "com.example".to_string(),
            window_title: None,
            is_secure: false,
            ime_active: false,
        };

        assert!(matcher.match_buffer("signature", &ctx).is_none());
        assert!(matcher.match_buffer(" email sig", &ctx).is_some());
    }

    #[test]
    fn respects_case_mode() {
        let mut s = snippet("addr", "123 Main St");
        s.case_mode = CaseMode::Upper;
        let matcher = Matcher::from_snippets(vec![s]);
        let ctx = MatchContext {
            app_bundle_id: "com.example".to_string(),
            window_title: None,
            is_secure: false,
            ime_active: false,
        };

        assert!(matcher.match_buffer("addr", &ctx).is_none());
        assert!(matcher.match_buffer("ADDR", &ctx).is_some());
    }

    #[test]
    fn matches_scoped_app() {
        let mut s = snippet(";sig", "Best,");
        s.scope.app_rules.push(AppRule {
            bundle_id: "com.mail".to_string(),
            window_title_pattern: None,
            enabled: true,
        });

        let matcher = Matcher::from_snippets(vec![s]);
        let ctx = MatchContext {
            app_bundle_id: "com.mail".to_string(),
            window_title: None,
            is_secure: false,
            ime_active: false,
        };
        let result = matcher.match_buffer(";sig", &ctx);
        assert!(result.is_some());
    }
}
