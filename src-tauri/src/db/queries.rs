use rusqlite::{params, Connection};
use uuid::Uuid;
use chrono::Utc;
use crate::models::project::{Project, CreateProjectRequest, ProjectWithStats};
use crate::models::spec::{Spec, Requirement};
use crate::models::test::{GeneratedTest, TestResult};
use crate::models::report::{AlignmentReport, Mismatch, AlignmentReportWithMismatches};
use crate::errors::AppError;

// ─── Projects ───────────────────────────────────────────────────

pub fn create_project(conn: &Connection, req: &CreateProjectRequest) -> Result<Project, AppError> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO projects (id, name, codebase_path, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![id, req.name, req.codebase_path, now, now],
    )?;
    Ok(Project {
        id,
        name: req.name.clone(),
        codebase_path: req.codebase_path.clone(),
        created_at: now.clone(),
        updated_at: now,
    })
}

pub fn list_projects(conn: &Connection) -> Result<Vec<ProjectWithStats>, AppError> {
    let mut stmt = conn.prepare(
        "SELECT p.id, p.name, p.codebase_path, p.created_at, p.updated_at,
                COALESCE((SELECT COUNT(*) FROM specs WHERE project_id = p.id), 0) as spec_count,
                (SELECT coverage_percent FROM alignment_reports WHERE project_id = p.id ORDER BY generated_at DESC LIMIT 1) as coverage_percent,
                (SELECT generated_at FROM alignment_reports WHERE project_id = p.id ORDER BY generated_at DESC LIMIT 1) as last_run_at
         FROM projects p ORDER BY p.updated_at DESC"
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(ProjectWithStats {
            project: Project {
                id: row.get(0)?,
                name: row.get(1)?,
                codebase_path: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            },
            spec_count: row.get(5)?,
            coverage_percent: row.get(6)?,
            last_run_at: row.get(7)?,
        })
    })?;
    let mut projects = Vec::new();
    for row in rows {
        projects.push(row?);
    }
    Ok(projects)
}

pub fn get_project(conn: &Connection, id: &str) -> Result<ProjectWithStats, AppError> {
    conn.query_row(
        "SELECT p.id, p.name, p.codebase_path, p.created_at, p.updated_at,
                COALESCE((SELECT COUNT(*) FROM specs WHERE project_id = p.id), 0) as spec_count,
                (SELECT coverage_percent FROM alignment_reports WHERE project_id = p.id ORDER BY generated_at DESC LIMIT 1) as coverage_percent,
                (SELECT generated_at FROM alignment_reports WHERE project_id = p.id ORDER BY generated_at DESC LIMIT 1) as last_run_at
         FROM projects p WHERE p.id = ?1",
        params![id],
        |row| {
            Ok(ProjectWithStats {
                project: Project {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    codebase_path: row.get(2)?,
                    created_at: row.get(3)?,
                    updated_at: row.get(4)?,
                },
                spec_count: row.get(5)?,
                coverage_percent: row.get(6)?,
                last_run_at: row.get(7)?,
            })
        },
    ).map_err(|_| AppError::NotFound(format!("Project not found: {}", id)))
}

pub fn delete_project(conn: &Connection, id: &str) -> Result<(), AppError> {
    let affected = conn.execute("DELETE FROM projects WHERE id = ?1", params![id])?;
    if affected == 0 {
        return Err(AppError::NotFound(format!("Project not found: {}", id)));
    }
    Ok(())
}

// ─── Specs ──────────────────────────────────────────────────────

pub fn create_spec(conn: &Connection, project_id: &str, filename: &str, content: &str) -> Result<Spec, AppError> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO specs (id, project_id, filename, content, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![id, project_id, filename, content, now],
    )?;
    Ok(Spec {
        id,
        project_id: project_id.to_string(),
        filename: filename.to_string(),
        content: content.to_string(),
        parsed_at: None,
        created_at: now,
    })
}

pub fn get_spec(conn: &Connection, id: &str) -> Result<Spec, AppError> {
    conn.query_row(
        "SELECT id, project_id, filename, content, parsed_at, created_at FROM specs WHERE id = ?1",
        params![id],
        |row| {
            Ok(Spec {
                id: row.get(0)?,
                project_id: row.get(1)?,
                filename: row.get(2)?,
                content: row.get(3)?,
                parsed_at: row.get(4)?,
                created_at: row.get(5)?,
            })
        },
    ).map_err(|_| AppError::NotFound(format!("Spec not found: {}", id)))
}

pub fn list_specs(conn: &Connection, project_id: &str) -> Result<Vec<Spec>, AppError> {
    let mut stmt = conn.prepare(
        "SELECT id, project_id, filename, content, parsed_at, created_at FROM specs WHERE project_id = ?1 ORDER BY created_at DESC"
    )?;
    let rows = stmt.query_map(params![project_id], |row| {
        Ok(Spec {
            id: row.get(0)?,
            project_id: row.get(1)?,
            filename: row.get(2)?,
            content: row.get(3)?,
            parsed_at: row.get(4)?,
            created_at: row.get(5)?,
        })
    })?;
    let mut specs = Vec::new();
    for row in rows {
        specs.push(row?);
    }
    Ok(specs)
}

pub fn delete_spec(conn: &Connection, id: &str) -> Result<(), AppError> {
    let affected = conn.execute("DELETE FROM specs WHERE id = ?1", params![id])?;
    if affected == 0 {
        return Err(AppError::NotFound(format!("Spec not found: {}", id)));
    }
    Ok(())
}

pub fn update_spec_parsed_at(conn: &Connection, spec_id: &str) -> Result<(), AppError> {
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE specs SET parsed_at = ?1 WHERE id = ?2",
        params![now, spec_id],
    )?;
    Ok(())
}

// ─── Requirements ───────────────────────────────────────────────

pub fn insert_requirements(conn: &Connection, requirements: &[Requirement]) -> Result<(), AppError> {
    let mut stmt = conn.prepare(
        "INSERT INTO requirements (id, spec_id, section, description, req_type, priority) VALUES (?1, ?2, ?3, ?4, ?5, ?6)"
    )?;
    for req in requirements {
        stmt.execute(params![req.id, req.spec_id, req.section, req.description, req.req_type, req.priority])?;
    }
    Ok(())
}

pub fn get_requirements_for_spec(conn: &Connection, spec_id: &str) -> Result<Vec<Requirement>, AppError> {
    let mut stmt = conn.prepare(
        "SELECT id, spec_id, section, description, req_type, priority FROM requirements WHERE spec_id = ?1 ORDER BY section, id"
    )?;
    let rows = stmt.query_map(params![spec_id], |row| {
        Ok(Requirement {
            id: row.get(0)?,
            spec_id: row.get(1)?,
            section: row.get(2)?,
            description: row.get(3)?,
            req_type: row.get(4)?,
            priority: row.get(5)?,
        })
    })?;
    let mut reqs = Vec::new();
    for row in rows {
        reqs.push(row?);
    }
    Ok(reqs)
}

pub fn get_requirements_for_project(conn: &Connection, project_id: &str) -> Result<Vec<Requirement>, AppError> {
    let mut stmt = conn.prepare(
        "SELECT r.id, r.spec_id, r.section, r.description, r.req_type, r.priority
         FROM requirements r
         JOIN specs s ON r.spec_id = s.id
         WHERE s.project_id = ?1
         ORDER BY r.section, r.id"
    )?;
    let rows = stmt.query_map(params![project_id], |row| {
        Ok(Requirement {
            id: row.get(0)?,
            spec_id: row.get(1)?,
            section: row.get(2)?,
            description: row.get(3)?,
            req_type: row.get(4)?,
            priority: row.get(5)?,
        })
    })?;
    let mut reqs = Vec::new();
    for row in rows {
        reqs.push(row?);
    }
    Ok(reqs)
}

pub fn delete_requirements_for_spec(conn: &Connection, spec_id: &str) -> Result<(), AppError> {
    conn.execute("DELETE FROM requirements WHERE spec_id = ?1", params![spec_id])?;
    Ok(())
}

pub fn get_requirement(conn: &Connection, id: &str) -> Result<Requirement, AppError> {
    conn.query_row(
        "SELECT id, spec_id, section, description, req_type, priority FROM requirements WHERE id = ?1",
        params![id],
        |row| {
            Ok(Requirement {
                id: row.get(0)?,
                spec_id: row.get(1)?,
                section: row.get(2)?,
                description: row.get(3)?,
                req_type: row.get(4)?,
                priority: row.get(5)?,
            })
        },
    ).map_err(|_| AppError::NotFound(format!("Requirement not found: {}", id)))
}

// ─── Generated Tests ────────────────────────────────────────────

pub fn insert_generated_test(conn: &Connection, test: &GeneratedTest) -> Result<(), AppError> {
    conn.execute(
        "INSERT INTO generated_tests (id, requirement_id, framework, code, generation_mode, file_path, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![test.id, test.requirement_id, test.framework, test.code, test.generation_mode, test.file_path, test.created_at],
    )?;
    Ok(())
}

pub fn get_generated_tests_for_requirement(conn: &Connection, requirement_id: &str) -> Result<Vec<GeneratedTest>, AppError> {
    let mut stmt = conn.prepare(
        "SELECT id, requirement_id, framework, code, generation_mode, file_path, created_at FROM generated_tests WHERE requirement_id = ?1 ORDER BY created_at DESC"
    )?;
    let rows = stmt.query_map(params![requirement_id], |row| {
        Ok(GeneratedTest {
            id: row.get(0)?,
            requirement_id: row.get(1)?,
            framework: row.get(2)?,
            code: row.get(3)?,
            generation_mode: row.get(4)?,
            file_path: row.get(5)?,
            created_at: row.get(6)?,
        })
    })?;
    let mut tests = Vec::new();
    for row in rows {
        tests.push(row?);
    }
    Ok(tests)
}

pub fn get_generated_test(conn: &Connection, id: &str) -> Result<GeneratedTest, AppError> {
    conn.query_row(
        "SELECT id, requirement_id, framework, code, generation_mode, file_path, created_at FROM generated_tests WHERE id = ?1",
        params![id],
        |row| {
            Ok(GeneratedTest {
                id: row.get(0)?,
                requirement_id: row.get(1)?,
                framework: row.get(2)?,
                code: row.get(3)?,
                generation_mode: row.get(4)?,
                file_path: row.get(5)?,
                created_at: row.get(6)?,
            })
        },
    ).map_err(|_| AppError::NotFound(format!("Generated test not found: {}", id)))
}

pub fn update_generated_test_path(conn: &Connection, id: &str, path: &str) -> Result<(), AppError> {
    conn.execute(
        "UPDATE generated_tests SET file_path = ?1 WHERE id = ?2",
        params![path, id],
    )?;
    Ok(())
}

#[allow(dead_code)]
pub fn get_generated_tests_for_project(conn: &Connection, project_id: &str) -> Result<Vec<GeneratedTest>, AppError> {
    let mut stmt = conn.prepare(
        "SELECT gt.id, gt.requirement_id, gt.framework, gt.code, gt.generation_mode, gt.file_path, gt.created_at
         FROM generated_tests gt
         JOIN requirements r ON gt.requirement_id = r.id
         JOIN specs s ON r.spec_id = s.id
         WHERE s.project_id = ?1
         ORDER BY gt.created_at DESC"
    )?;
    let rows = stmt.query_map(params![project_id], |row| {
        Ok(GeneratedTest {
            id: row.get(0)?,
            requirement_id: row.get(1)?,
            framework: row.get(2)?,
            code: row.get(3)?,
            generation_mode: row.get(4)?,
            file_path: row.get(5)?,
            created_at: row.get(6)?,
        })
    })?;
    let mut tests = Vec::new();
    for row in rows {
        tests.push(row?);
    }
    Ok(tests)
}

// ─── Test Results ───────────────────────────────────────────────

pub fn insert_test_result(conn: &Connection, result: &TestResult) -> Result<(), AppError> {
    conn.execute(
        "INSERT INTO test_results (id, generated_test_id, status, execution_time_ms, stdout, stderr, executed_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![result.id, result.generated_test_id, result.status, result.execution_time_ms, result.stdout, result.stderr, result.executed_at],
    )?;
    Ok(())
}

pub fn get_test_results_for_project(conn: &Connection, project_id: &str) -> Result<Vec<TestResult>, AppError> {
    let mut stmt = conn.prepare(
        "SELECT tr.id, tr.generated_test_id, tr.status, tr.execution_time_ms, tr.stdout, tr.stderr, tr.executed_at
         FROM test_results tr
         JOIN generated_tests gt ON tr.generated_test_id = gt.id
         JOIN requirements r ON gt.requirement_id = r.id
         JOIN specs s ON r.spec_id = s.id
         WHERE s.project_id = ?1
         ORDER BY tr.executed_at DESC"
    )?;
    let rows = stmt.query_map(params![project_id], |row| {
        Ok(TestResult {
            id: row.get(0)?,
            generated_test_id: row.get(1)?,
            status: row.get(2)?,
            execution_time_ms: row.get(3)?,
            stdout: row.get(4)?,
            stderr: row.get(5)?,
            executed_at: row.get(6)?,
        })
    })?;
    let mut results = Vec::new();
    for row in rows {
        results.push(row?);
    }
    Ok(results)
}

pub fn get_test_result(conn: &Connection, id: &str) -> Result<TestResult, AppError> {
    conn.query_row(
        "SELECT id, generated_test_id, status, execution_time_ms, stdout, stderr, executed_at FROM test_results WHERE id = ?1",
        params![id],
        |row| {
            Ok(TestResult {
                id: row.get(0)?,
                generated_test_id: row.get(1)?,
                status: row.get(2)?,
                execution_time_ms: row.get(3)?,
                stdout: row.get(4)?,
                stderr: row.get(5)?,
                executed_at: row.get(6)?,
            })
        },
    ).map_err(|_| AppError::NotFound(format!("Test result not found: {}", id)))
}

pub fn get_latest_test_result_for_test(conn: &Connection, generated_test_id: &str) -> Result<Option<TestResult>, AppError> {
    let result = conn.query_row(
        "SELECT id, generated_test_id, status, execution_time_ms, stdout, stderr, executed_at FROM test_results WHERE generated_test_id = ?1 ORDER BY executed_at DESC LIMIT 1",
        params![generated_test_id],
        |row| {
            Ok(TestResult {
                id: row.get(0)?,
                generated_test_id: row.get(1)?,
                status: row.get(2)?,
                execution_time_ms: row.get(3)?,
                stdout: row.get(4)?,
                stderr: row.get(5)?,
                executed_at: row.get(6)?,
            })
        },
    );
    match result {
        Ok(r) => Ok(Some(r)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(AppError::Database(e)),
    }
}

// ─── Alignment Reports ─────────────────────────────────────────

pub fn insert_alignment_report(conn: &Connection, report: &AlignmentReport) -> Result<(), AppError> {
    conn.execute(
        "INSERT INTO alignment_reports (id, project_id, coverage_percent, total_requirements, covered_requirements, generated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![report.id, report.project_id, report.coverage_percent, report.total_requirements, report.covered_requirements, report.generated_at],
    )?;
    Ok(())
}

pub fn insert_mismatch(conn: &Connection, mismatch: &Mismatch) -> Result<(), AppError> {
    conn.execute(
        "INSERT INTO alignment_mismatches (id, report_id, requirement_id, spec_section, code_element, mismatch_type, details) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![mismatch.id, mismatch.report_id, mismatch.requirement_id, mismatch.spec_section, mismatch.code_element, mismatch.mismatch_type, mismatch.details],
    )?;
    Ok(())
}

pub fn get_alignment_report(conn: &Connection, id: &str) -> Result<AlignmentReportWithMismatches, AppError> {
    let report = conn.query_row(
        "SELECT id, project_id, coverage_percent, total_requirements, covered_requirements, generated_at FROM alignment_reports WHERE id = ?1",
        params![id],
        |row| {
            Ok(AlignmentReport {
                id: row.get(0)?,
                project_id: row.get(1)?,
                coverage_percent: row.get(2)?,
                total_requirements: row.get(3)?,
                covered_requirements: row.get(4)?,
                generated_at: row.get(5)?,
            })
        },
    ).map_err(|_| AppError::NotFound(format!("Report not found: {}", id)))?;

    let mismatches = get_mismatches_for_report(conn, &report.id)?;

    Ok(AlignmentReportWithMismatches { report, mismatches })
}

pub fn get_mismatches_for_report(conn: &Connection, report_id: &str) -> Result<Vec<Mismatch>, AppError> {
    let mut stmt = conn.prepare(
        "SELECT id, report_id, requirement_id, spec_section, code_element, mismatch_type, details FROM alignment_mismatches WHERE report_id = ?1"
    )?;
    let rows = stmt.query_map(params![report_id], |row| {
        Ok(Mismatch {
            id: row.get(0)?,
            report_id: row.get(1)?,
            requirement_id: row.get(2)?,
            spec_section: row.get(3)?,
            code_element: row.get(4)?,
            mismatch_type: row.get(5)?,
            details: row.get(6)?,
        })
    })?;
    let mut mismatches = Vec::new();
    for row in rows {
        mismatches.push(row?);
    }
    Ok(mismatches)
}

pub fn list_reports(conn: &Connection, project_id: &str) -> Result<Vec<AlignmentReport>, AppError> {
    let mut stmt = conn.prepare(
        "SELECT id, project_id, coverage_percent, total_requirements, covered_requirements, generated_at FROM alignment_reports WHERE project_id = ?1 ORDER BY generated_at DESC"
    )?;
    let rows = stmt.query_map(params![project_id], |row| {
        Ok(AlignmentReport {
            id: row.get(0)?,
            project_id: row.get(1)?,
            coverage_percent: row.get(2)?,
            total_requirements: row.get(3)?,
            covered_requirements: row.get(4)?,
            generated_at: row.get(5)?,
        })
    })?;
    let mut reports = Vec::new();
    for row in rows {
        reports.push(row?);
    }
    Ok(reports)
}
