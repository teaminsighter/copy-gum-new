// Database module for CopyGum
// Handles SQLite connection, initialization, and migrations
// Database operations are now handled directly in frontend via @tauri-apps/plugin-sql

use tauri_plugin_sql::{Migration, MigrationKind};

/// Initialize the database with schema and migrations
pub fn init_database() -> Vec<Migration> {
    vec![
        // Migration 1: Initial schema
        Migration {
            version: 1,
            description: "create_initial_schema",
            sql: include_str!("schema.sql"),
            kind: MigrationKind::Up,
        },
    ]
}

// Database connection will be accessed via tauri-plugin-sql
// Example usage in commands:
// use tauri_plugin_sql::Builder;
// let db = app.state::<Builder>().get("sqlite:copygum.db");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_database() {
        let migrations = init_database();
        assert_eq!(migrations.len(), 1);
        assert_eq!(migrations[0].version, 1);
    }
}
