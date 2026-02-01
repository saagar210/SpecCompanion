use rusqlite::Connection;

const CURRENT_VERSION: i32 = 1;

pub fn run_migrations(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS schema_version (
            version INTEGER NOT NULL
        );"
    )?;

    let version: i32 = conn
        .query_row("SELECT COALESCE(MAX(version), 0) FROM schema_version", [], |row| row.get(0))
        .unwrap_or(0);

    if version < CURRENT_VERSION {
        let tx = conn.unchecked_transaction()?;
        migrate_v1(&tx)?;
        tx.execute("DELETE FROM schema_version", [])?;
        tx.execute("INSERT INTO schema_version (version) VALUES (?1)", [CURRENT_VERSION])?;
        tx.commit()?;
    }

    Ok(())
}

fn migrate_v1(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS projects (
            id TEXT PRIMARY KEY NOT NULL,
            name TEXT NOT NULL,
            codebase_path TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS specs (
            id TEXT PRIMARY KEY NOT NULL,
            project_id TEXT NOT NULL,
            filename TEXT NOT NULL,
            content TEXT NOT NULL,
            parsed_at TEXT,
            created_at TEXT NOT NULL,
            FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS requirements (
            id TEXT PRIMARY KEY NOT NULL,
            spec_id TEXT NOT NULL,
            section TEXT NOT NULL,
            description TEXT NOT NULL,
            req_type TEXT NOT NULL DEFAULT 'functional',
            priority TEXT NOT NULL DEFAULT 'medium',
            FOREIGN KEY (spec_id) REFERENCES specs(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS generated_tests (
            id TEXT PRIMARY KEY NOT NULL,
            requirement_id TEXT NOT NULL,
            framework TEXT NOT NULL,
            code TEXT NOT NULL,
            generation_mode TEXT NOT NULL,
            file_path TEXT,
            created_at TEXT NOT NULL,
            FOREIGN KEY (requirement_id) REFERENCES requirements(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS test_results (
            id TEXT PRIMARY KEY NOT NULL,
            generated_test_id TEXT NOT NULL,
            status TEXT NOT NULL,
            execution_time_ms INTEGER NOT NULL DEFAULT 0,
            stdout TEXT NOT NULL DEFAULT '',
            stderr TEXT NOT NULL DEFAULT '',
            executed_at TEXT NOT NULL,
            FOREIGN KEY (generated_test_id) REFERENCES generated_tests(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS alignment_reports (
            id TEXT PRIMARY KEY NOT NULL,
            project_id TEXT NOT NULL,
            coverage_percent REAL NOT NULL DEFAULT 0.0,
            total_requirements INTEGER NOT NULL DEFAULT 0,
            covered_requirements INTEGER NOT NULL DEFAULT 0,
            generated_at TEXT NOT NULL,
            FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS alignment_mismatches (
            id TEXT PRIMARY KEY NOT NULL,
            report_id TEXT NOT NULL,
            requirement_id TEXT NOT NULL,
            spec_section TEXT NOT NULL,
            code_element TEXT,
            mismatch_type TEXT NOT NULL,
            details TEXT NOT NULL DEFAULT '',
            FOREIGN KEY (report_id) REFERENCES alignment_reports(id) ON DELETE CASCADE,
            FOREIGN KEY (requirement_id) REFERENCES requirements(id) ON DELETE CASCADE
        );

        CREATE INDEX IF NOT EXISTS idx_specs_project_id ON specs(project_id);
        CREATE INDEX IF NOT EXISTS idx_requirements_spec_id ON requirements(spec_id);
        CREATE INDEX IF NOT EXISTS idx_generated_tests_requirement_id ON generated_tests(requirement_id);
        CREATE INDEX IF NOT EXISTS idx_test_results_generated_test_id ON test_results(generated_test_id);
        CREATE INDEX IF NOT EXISTS idx_alignment_reports_project_id ON alignment_reports(project_id);
        CREATE INDEX IF NOT EXISTS idx_alignment_mismatches_report_id ON alignment_mismatches(report_id);"
    )?;

    Ok(())
}
