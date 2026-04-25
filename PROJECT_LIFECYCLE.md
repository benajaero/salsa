# Salsa Project Lifecycle

Weekly rotation for sustained, focused development.

| Day | Activity | Focus |
|-----|----------|-------|
| Monday | Planning & Architecture | Review spec, roadmap, and open questions. Update designs. |
| Tuesday | Core Engine | Matcher, templating, Sauce Variables, performance. |
| Wednesday | Store & Persistence | Migrations, backup/restore, query optimization. |
| Thursday | UI & UX | GPUI views, Salsa Bar, Test Kitchen, onboarding flows. |
| Friday | Agent & Platform | Event tap, injection, permissions, macOS bindings. |
| Saturday | Testing & QA | Unit tests, integration tests, lint rules, CI health. |
| Sunday | Review & Documentation | README updates, inline docs, spec drift check, cleanup. |

## Activity Guidelines

- **Stay in scope**: Don't drift into another day's domain unless blocked.
- **Leave a trail**: Commit message includes `[activity]` tag.
- **End with a summary**: What changed, what's next, any blockers.
- **Weekly reset**: Every Monday, confirm priorities are still correct.

## Current Status

See `README.md` for project overview and `spec.md` for design constraints.

- Core engine: scaffolded with trie matcher, basic tests.
- Store: SQLite with migrations, full CRUD for snippets.
- Agent: scaffolding only, event tap not wired.
- UI: GPUI shell with search and click placeholders.
- macOS: permission stubs, no real platform integration.
- IPC: minimal Ping/Pong over Unix socket.
