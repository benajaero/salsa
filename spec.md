# Salsa Product + Technical Specification

## 1. Overview

Salsa is a fast, native text expander and snippet manager for macOS. It turns short triggers (like `;addr`) into longer text anywhere you type. The product aims to be the most reliable and usable expander on the market while staying strictly local-first.

**Who it’s for**
- People who reuse the same phrases, addresses, and templates every day.
- Support, sales, and ops teams working across many apps.
- Developers and writers who want quick insertions without switching contexts.

**What success looks like**
- Expansions feel instant and predictable.
- Users understand and trust permissions.
- Snippets are easy to find, test, and maintain.

**“Works everywhere” scope**
- In scope: standard macOS text inputs across native apps, browsers, and Electron.
- Best-effort: editors and terminals that accept synthetic input.
- Out of scope by default: secure fields, apps that enable secure input or block injection, and some custom canvas-based inputs. Salsa detects and avoids these safely.

## 2. Goals and Non-goals

**Goals (v1)**
- System-wide expansion with deterministic matching and low latency.
- A fast GPUI UI with a command palette (Salsa Bar).
- Local-first privacy: no telemetry by default, no content logging.
- Clear, staged permissions onboarding with a dashboard.
- Reliable local storage with migrations and backups.
- Open source code for auditability and community contribution.

**Non-goals (v1)**
- Cloud sync or multi-device accounts.
- Full scripting or a programming language inside snippets.
- iOS/Windows support.
- Collaboration features.
- AI content generation.

## 3. User Stories
1. As a new user, I can grant permissions step-by-step and understand why Salsa needs them.
2. As a user, I can create a snippet and see it expand instantly in any app.
3. As a user, I can test a snippet safely and see why it matched.
4. As a user, I can scope snippets to specific apps or window titles.
5. As a user, I can use Sauce Variables like date/time/clipboard.
6. As a user, I can search and insert snippets from the Salsa Bar.
7. As a user, I can undo the last expansion with a hotkey.
8. As a user, I can resolve duplicate triggers with clear priority rules.
9. As a user, I can organize snippets into Menus (profiles) and Jars (bundles).
10. As a user, I can see what I get with the official build vs a community build.
11. As a user, I can export and import my snippets locally.
12. As a user, I can trust Salsa never stores my raw typing.

## 4. MVP Feature Set

**Core engine**
- System-wide expansion via event tap + synthetic input.
- Deterministic matching engine (O(1)–O(log n) per keystroke).
- Delimiter-aware triggers, word boundary controls, and case modes.
- Anti-accidental protections (debounce, min trigger length, linger window).

**Killer differentiators (v1)**
1. **Test Kitchen (Live Playground)**
   - Shows matched trigger, rule path, resolved output, and undo/redo preview.
   - Toggles for secure field, app/profile selection, IME mode.
2. **Context-aware snippets**
   - App bundle ID scoping and optional window title pattern.
   - Per-app enable/disable and per-app output variants.
3. **Sauce Variables + Mini-Templates**
   - `{{date}}`, `{{time}}`, `{{clipboard}}`, `{{cursor}}`
   - `{{choice:A|B|C}}`
   - `{{fill:FieldName}}`
   - `{{calc:...}}`
   - `{{rand}}`
   - `{{case:smart}}`
4. **Salsa Bar (Command Palette)**
   - Fuzzy search triggers/content/tags.
   - Quick insert without triggers.
   - Pinned favorites and recents.
5. **Smart Expansion Engine**
   - Linger window for focus jitter.
   - Per-app fallback strategy.
   - Safe behavior in secure fields.

**Power-user depth (v1 + v1.1+)**
- Snippet types: plain text (v1), rich text/Markdown with downgrade (v2).
- Menus (profiles) and Jars (bundles) with export/import.
- Governance: linting, conflict inspector (Heat Map).
- History metadata with optional content retention (opt-in).
- Transforms toolkit (v1.1): case, trim, slugify, URL encode/decode.

## 5. UX Flows

**First-run onboarding**
1. Welcome screen: “Make typing feel like a dance.”
2. Explain privacy and permissions in plain language.
3. Request Accessibility permission.
4. Request Input Monitoring permission (if needed for capture method).
5. Show Permissions Dashboard with “Test input” and “Fix now” actions.

**Create snippet**
1. “New Snippet” form: trigger, label, content, tags, scope.
2. Optional Sauce Variables with inline help.
3. Test in Test Kitchen.
4. Save and enable.

**Manage snippets**
- List view with filters (tags, Menus, Jars, app scope).
- Conflict indicator and lint suggestions.
- Bulk enable/disable by profile or bundle.

**Conflicts**
- Duplicate triggers show priority order and a resolver.
- Heat Map view previews which snippet will fire per app/profile.

**Per-app rules**
- Add scope by selecting a running app or entering bundle ID.
- Optional window title match (regex-lite).

**Official build purchase flow**
- Settings: “Get the Official Build” panel explains benefits clearly.
- Supporter tier is framed as sustaining development.
- Community build disclaimer: “Community builds are not signed/notarised.”

## 6. Branding System

**Voice rules**
- Punchy, rhythmic, confident.
- Playful with light myth nods, never dense or cheesy.
- Clear, respectful, and transparent in permissions and privacy.

**Do**
- “Stir faster. Type less.”
- “Your snippets, your kitchen.”

**Don’t**
- Overuse Greek references.
- Use guilt-based upgrade copy.

**Visual direction**
- Warm, appetizing palette with bright accents; avoid sterile gray UI.
- Bold type pairing: one expressive display face + a crisp UI face.
- Iconography: rounded forms with subtle motion cues (stir, pour, spark).
- Myth motif used sparingly: a laurel or constellation detail in illustrations.

**Example strings**
- Onboarding: “Welcome to Salsa. Let’s make typing sing.”
- Permissions: “We only listen for short triggers, never your full sentence.”
- Empty state: “No snippets yet. Let’s add a fresh batch.”
- Tooltip: “Scopes this snippet to the current app.”
- Error: “That trigger is already in the jar.”
- Official build prompt: “Get the official build for signed, notarised goodness.”
- Undo: “Expansion undone. Back to simmer.”

## 7. Data Model

**Storage choice**: SQLite (via `rusqlite`). SQLite gives predictable performance, indexed queries, and safe migrations. A file-based JSON store is simpler but weaker for conflict checks and history queries.

**Schema (simplified)**

```rust
struct Snippet {
    id: Uuid,
    trigger: String,
    label: String,
    content: String,
    content_type: ContentType, // PlainText, Markdown
    tags: Vec<String>,
    enabled: bool,
    case_mode: CaseMode, // Smart, Upper, Lower, Preserve
    delimiter_mode: DelimiterMode, // WordBoundary, Any, Custom
    scope: ScopeRule,
    priority: i32,
    created_at: DateTime,
    updated_at: DateTime,
}

struct ScopeRule {
    app_rules: Vec<AppRule>,
    profile_id: Option<Uuid>,
}

struct AppRule {
    bundle_id: String,
    window_title_pattern: Option<String>,
    enabled: bool,
}

struct Profile {
    id: Uuid,
    name: String,
    enabled: bool,
}

struct Bundle {
    id: Uuid,
    name: String,
    description: Option<String>,
    enabled: bool,
}

struct ExpansionHistory {
    id: Uuid,
    snippet_id: Uuid,
    app_bundle_id: String,
    timestamp: DateTime,
    retained_content: Option<String>, // Opt-in only
}
```

**Migrations**
- Versioned SQL migrations with checksums.
- Backup on each schema bump to `~/Library/Application Support/Salsa/backups/`.
- Automatic rollback if migration fails (restore last known good DB).

## 8. System Architecture

**High-level**
- **Agent**: background daemon that captures keystrokes, matches triggers, and injects expansions.
- **GPUI app**: settings, snippet manager, Test Kitchen, Salsa Bar.
- **Core engine**: deterministic matcher, independent of macOS APIs.

**Modules / crates**
- `salsa-core`: matching engine, template resolver, linting, conflict detection.
- `salsa-store`: SQLite access, migrations, backup/restore.
- `salsa-agent`: macOS event tap, injection, permissions watchdog.
- `salsa-ui`: GPUI frontend and IPC client.
- `salsa-macos`: platform bindings, secure input detection, app metadata.

**IPC**
- Local IPC via Unix domain socket or XPC-like abstraction.
- UI communicates with agent for status, Test Kitchen, and expansion simulation.

**Privacy constraints**
- Event tap buffers only what’s needed to detect triggers (rolling window).
- Typed streams are never persisted.
- No raw input logging.

## 9. macOS Integration Details

**Permissions**
- Accessibility: required for event tap and injection.
- Input Monitoring: required when using low-level key events.

**Event capture options**
- CGEventTap for key down events.
- Secure input detection via IOHID/AX APIs.

**Injection options**
- CGEventPost for synthetic key events.
- Accessibility API for pasteboard-based insertion when typing injection fails.

**Sandbox strategy**
- Primary plan: non-sandboxed app for full event tap/injection reliability.
- Document sandboxed limitations (reduced access, extra prompts, limited injection).

## 10. Edge Cases
- Secure input enabled (passwords, system prompts).
- IME composition (no trigger mid-composition).
- Focus jitter (use linger window).
- Conflicting triggers across Menus/Jars.
- Host app blocks backspace or synthetic input.
- High-frequency typing bursts.

## 11. Quality Bar / Acceptance Criteria

**Performance**
- Expansion latency median ≤ 20ms, p95 ≤ 60ms.
- No perceptible typing lag; per-keystroke work O(1)–O(log n).
- CPU idle ≤ 0.3% avg; active typing ≤ 2% avg.
- Memory ≤ 150MB typical (≤10k snippets).
- Agent cold start ≤ 700ms; UI open ≤ 250ms warm.

**Reliability / safety**
- Expansion success rate ≥ 99.5% in QA matrix.
- Never expand in secure fields by default.
- Agent auto-recovers; UI crash must not kill agent (separate process).
- Watchdog for event tap failures with user remediation.
- Undo last expansion hotkey; fail gracefully if blocked.

**Privacy / security**
- Local-first; no telemetry by default.
- Never log snippet content, typed content, or outputs.
- Crash reporting opt-in and redacts content.

## 12. Test Plan

**Unit tests**
- Matcher correctness across triggers, delimiters, case modes.
- Template resolver for Sauce variables.
- Conflict detection and lint rules.
- SQLite migrations and rollback.

**Integration tests**
- Agent to UI IPC.
- Expansion injection behavior in a mock target app.
- Permissions watchdog.

**Manual QA matrix (12–20 apps)**
- Safari, Chrome, Firefox
- Mail, Notes
- Slack, Discord
- VS Code, Xcode
- Terminal, iTerm2
- Microsoft Word, Google Docs
- Figma, Notion
- Password prompt (secure field) checks

## 13. Distribution & Release

**Community build path**
- Source available; users build via Cargo.
- Optional Homebrew formula that builds from source with clear disclaimers.
- Community builds are not signed/notarised.

**Official build path (paid)**
- Signed + notarised `.app` distributed via direct download.
- Optional auto-updater (Sparkle-style or equivalent) documented at a high level.
- Homebrew cask installs the official build for paying users via tokenized URL flow.

**Versioning and channels**
- SemVer with stable/beta channels.
- Release artifacts: DMG, ZIP, checksums, release notes.
- Rollback: last stable build preserved for recovery.

## 14. Business Model & Pricing

**Official build pricing**
- Official Build (one-time): A$39 / US$29
- Supporter (one-time): A$79 / US$59
- Optional future: paid encrypted sync (monthly), pricing TBD.

**What’s free vs paid**
- **Open source build (Community)**: full source, self-build, community distribution possible.
- **Official build (Paid)**: signed + notarised, one-click install, auto-updates, priority support, polished template/bundle packs.

**Purchase experience**
- In-app “Get the Official Build” panel explains benefits.
- Supporter tier messaging emphasizes sustainability.
- Clear disclaimers: “Community builds are not signed/notarised; official builds are.”

## 15. Licence & Contribution Policy

**License**: Apache-2.0. Permissive, business-friendly, and compatible with paid official builds while supporting auditability and contributions.

**Trademark stance**
- “Salsa” name and logos are reserved trademarks. Third-party builds must not imply official endorsement.

**Contributions**
- Accept external contributions under DCO (Developer Certificate of Origin).
- All contributions include a statement of origin.
- Dependency licensing must remain clean to preserve the business model and official distribution.

## 16. Roadmap

**v1.0**
- Core expansion engine
- Test Kitchen
- Salsa Bar
- Context-aware snippets
- Sauce variables (basic)
- Menus and Jars
- Community and official build paths

**v1.1**
- Transforms toolkit
- Improved linting + Heat Map
- Create snippet from selection

**v2.0**
- Rich text/Markdown output
- Optional encrypted sync (if aligned with strategy)
- Plugin system for templates
