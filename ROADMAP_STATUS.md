# Roadmap Status

Tracking what exists vs what the spec (v1.0) calls for.

## Core Engine (`salsa-core`)
| Feature | Status | Notes |
|---------|--------|-------|
| Trie matcher | ✅ Done | Reverse tries, exact + folded |
| Case modes | ✅ Done | Smart, Upper, Lower, Preserve |
| Delimiter modes | ✅ Done | Any, WordBoundary, Custom |
| App scoping | ✅ Done | Bundle ID + optional window title |
| Profile scoping | ✅ Partial | Model supports it; UI/CLI not wired |
| Priority tie-break | ✅ Done | priority → updated_at → UUID |
| Sauce Variables | ❌ Not started | `{{date}}`, `{{time}}`, etc. |
| Template resolver | ❌ Not started | Post-match content expansion |
| Anti-accident protection | ❌ Not started | Debounce, min length, linger window |
| Undo support | ❌ Not started | Hotkey + backspace injection |

## Store (`salsa-store`)
| Feature | Status | Notes |
|---------|--------|-------|
| SQLite schema | ✅ Done | Snippets, rules, profiles, bundles, history |
| Migrations | ✅ Done | Versioned with checksum + rollback plan |
| Indexes | ✅ Done | Trigger, enabled, bundle_id, snippet_id |
| CRUD snippets | ✅ Done | Insert, list, update, delete |
| CRUD profiles | ✅ Done | Insert + list |
| CRUD bundles | ✅ Done | Insert + list |
| History append | ✅ Done | Opt-in content retention field ready |
| Backup/restore | ❌ Not started | Spec calls for `~/Library/…/backups/` |
| Profile-scoped queries | ❌ Not started | Need filtered list by active profile |

## Agent (`salsa-agent`)
| Feature | Status | Notes |
|---------|--------|-------|
| Process scaffold | ✅ Done | `--serve` and `--run` entry points |
| IPC server | ✅ Partial | Ping/Pong only |
| Permission checks | ✅ Partial | Stubs return `Unknown` |
| Event tap capture | ❌ Not started | CGEventTap wiring needed |
| Synthetic injection | ❌ Not started | CGEventPost or Accessibility paste |
| Secure-input detection | ❌ Not started | IOHID/AX APIs |
| Watchdog / auto-recover | ❌ Not started | Restart event tap on failure |
| Keystroke buffer | ❌ Not started | Rolling window for trigger detection |

## UI (`salsa-app`)
| Feature | Status | Notes |
|---------|--------|-------|
| GPUI window | ✅ Done | 900×600, keybindings wired |
| Search field | ✅ Partial | Tracks focus, key-down wired, no real input |
| Snippet list | ✅ Partial | Renders rows, no selection or actions |
| New Snippet button | ✅ Partial | Click prints placeholder |
| Salsa Bar | ❌ Not started | Command palette overlay |
| Test Kitchen | ❌ Not started | Live playground with match reasoning |
| Snippet editor form | ❌ Not started | Trigger, label, content, tags, scope |
| Settings | ❌ Not started | Permissions dashboard, official build panel |
| Onboarding flow | ❌ Not started | Welcome → permissions → test input |

## macOS Platform (`salsa-macos`)
| Feature | Status | Notes |
|---------|--------|-------|
| Permission enums | ✅ Done | `PermissionStatus` scaffolded |
| Platform gate | ✅ Done | `is_supported_platform()` |
| Real accessibility check | ❌ Not started | Needs `AXIsProcessTrustedWithOptions` |
| Real input-monitoring check | ❌ Not started | Needs `CGEventSourceStateID` or IOKit |
| App metadata query | ❌ Not started | Bundle ID + window title from AX API |

## Build & Distribution
| Feature | Status | Notes |
|---------|--------|-------|
| Rust workspace | ✅ Done | 5 crates, nightly toolchain |
| CI/CD GitHub Actions | ✅ Done | Build + test matrix |
| Official build pipeline | ❌ Not started | Sign + notarise, DMG, sparkle |
| Homebrew formula | ❌ Not started | Source-build formula documented in spec |

## Spec Drift Notes
- Sauce Variables are the largest v1 gap. They need both a parser (mini-template language) and runtime resolution (date, clipboard, choice, fill, calc).
- The agent is still a skeleton. Until event tap and injection are wired, Salsa cannot actually expand text system-wide.
- UI interactions are placeholders (click prints, key-down appends to a string but doesn't use a real text input).
- No integration tests yet for agent ↔ UI IPC or injection behaviour.
