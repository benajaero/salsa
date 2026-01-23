# Salsa

Fast, local-first text expansion for macOS — open source code with paid, signed builds.

Salsa turns short triggers (like `;addr`) into longer text anywhere you type. It’s built in Rust + GPUI and designed to be fast, reliable, and respectful of privacy.

## Why Salsa
- **Works everywhere**: native apps, browsers, Electron — wherever standard text input exists.
- **Local-first**: no accounts, no required sync, no telemetry by default.
- **Deterministic engine**: consistent expansions with low latency.
- **Open source code**: auditability and community contributions.
- **Paid official builds**: signed + notarised macOS apps with auto-updates.

## Core Features
- System-wide expansion with predictable matching.
- Test Kitchen: live playground with match reasoning and undo preview.
- Context-aware snippets: scope by app and window title.
- Sauce Variables: `{{date}}`, `{{time}}`, `{{clipboard}}`, `{{choice:A|B|C}}`, `{{fill:Name}}`, `{{calc:...}}`.
- Salsa Bar: fast command palette for search and insert.
- Safe by default: never expands in secure fields.

## Quick Start (Community Build)

```bash
cargo build --release
cargo run -p salsa-app
```

You will need Accessibility (and possibly Input Monitoring) permissions for system-wide expansion.

## Official Builds

Official builds are signed + notarised and include one-click install, auto-updates, and priority support. These are paid to fund development.

- **Official Build (one-time)**: A$39 / US$29
- **Supporter (one-time)**: A$79 / US$59

Community builds are welcome, but they are not signed/notarised and should not be represented as official.

## Architecture (High-Level)

- `core`: deterministic matching engine + templating
- `agent`: background service for event capture + injection
- `app`: GPUI UI for settings, Salsa Bar, and Test Kitchen
- `store`: SQLite persistence + migrations
- `macos`: platform bindings and permission helpers

## Privacy

Salsa listens only for short triggers. It does not log raw typing, snippet content, or expanded output. Typed streams are never persisted.

## Contributing

We accept contributions under the DCO (Developer Certificate of Origin). See `spec.md` for the working agreement and design constraints. If you’re shipping a build, please respect the Salsa trademarks and do not imply official status.

## License

Apache-2.0. See `LICENSE`.
