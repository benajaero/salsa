# Changelog

All notable changes to the Salsa project.

## [Unreleased]

### Added
- Core deterministic matcher with reverse trie (exact + folded).
- SQLite store with versioned migrations and indexed schema.
- GPUI shell UI: search input, snippet list, click interactions.
- CLI: `add`, `list`, `delete`, `lint`, `ping`, and `ui` commands.
- Agent scaffolding with Unix-domain IPC (Ping/Pong).
- macOS permission stubs.
- Lint rules: duplicate-trigger and priority-shadow detection.
- Model types: Snippet, Profile, Bundle, ExpansionHistory with full serde support.
- Unit tests for matcher correctness (basic, secure, word-boundary, case-mode, app-scope).

### Changed
- README updated with nightly toolchain, current status, and CLI examples.

## [0.1.0] - Project Inception
- Initial scaffold: workspace crates, CI/CD, spec, license.
