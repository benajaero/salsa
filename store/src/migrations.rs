//! Versioned SQLite schema migrations.
//!
//! Each migration is a single SQL batch. The `schema_migrations` table tracks
//! what has already been applied. Migrations are applied in version order
//! inside a single `Connection::execute_batch` call.

pub struct Migration {
    pub version: i64,
    pub sql: &'static str,
}

pub const MIGRATIONS: &[Migration] = &[
    Migration {
        version: 1,
        sql: include_str!("../migrations/001_init.sql"),
    },
    Migration {
        version: 2,
        sql: include_str!("../migrations/002_indexes.sql"),
    },
];
