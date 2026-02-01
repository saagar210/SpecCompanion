pub mod schema;
pub mod queries;

use rusqlite::Connection;
use std::sync::Mutex;

pub struct Database {
    pub conn: Mutex<Connection>,
}

impl Database {
    pub fn new(app_data_dir: &std::path::Path) -> Result<Self, rusqlite::Error> {
        std::fs::create_dir_all(app_data_dir)
            .map_err(|e| rusqlite::Error::InvalidPath(app_data_dir.join(e.to_string())))?;
        let db_path = app_data_dir.join("spec_companion.db");
        let conn = Connection::open(&db_path)?;

        // Enable WAL mode for better concurrent read performance
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;

        // Run migrations
        schema::run_migrations(&conn)?;

        Ok(Database {
            conn: Mutex::new(conn),
        })
    }
}
