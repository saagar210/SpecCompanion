use rusqlite::Connection;
use uuid::Uuid;
use chrono::Utc;
use crate::db::queries;
use crate::errors::AppError;
use crate::models::report::{AlignmentReport, Mismatch, AlignmentReportWithMismatches};

pub fn generate_report(conn: &Connection, project_id: &str) -> Result<AlignmentReportWithMismatches, AppError> {
    let requirements = queries::get_requirements_for_project(conn, project_id)?;
    let total = requirements.len() as i64;

    if total == 0 {
        let report = AlignmentReport {
            id: Uuid::new_v4().to_string(),
            project_id: project_id.to_string(),
            coverage_percent: 0.0,
            total_requirements: 0,
            covered_requirements: 0,
            generated_at: Utc::now().to_rfc3339(),
        };
        queries::insert_alignment_report(conn, &report)?;
        return Ok(AlignmentReportWithMismatches {
            report,
            mismatches: Vec::new(),
        });
    }

    let report_id = Uuid::new_v4().to_string();
    let mut covered = 0i64;
    let mut mismatches = Vec::new();

    for req in &requirements {
        let tests = queries::get_generated_tests_for_requirement(conn, &req.id)?;

        if tests.is_empty() {
            mismatches.push(Mismatch {
                id: Uuid::new_v4().to_string(),
                report_id: report_id.clone(),
                requirement_id: req.id.clone(),
                spec_section: req.section.clone(),
                code_element: None,
                mismatch_type: "no_test_generated".to_string(),
                details: format!("No test has been generated for: {}", req.description),
            });
            continue;
        }

        // Check if any test has been executed and passed
        let mut has_passing = false;
        let mut has_failing = false;
        let mut has_no_results = true;

        for test in &tests {
            if let Some(result) = queries::get_latest_test_result_for_test(conn, &test.id)? {
                has_no_results = false;
                match result.status.as_str() {
                    "passed" => has_passing = true,
                    "failed" | "error" => has_failing = true,
                    _ => {}
                }
            }
        }

        if has_no_results {
            mismatches.push(Mismatch {
                id: Uuid::new_v4().to_string(),
                report_id: report_id.clone(),
                requirement_id: req.id.clone(),
                spec_section: req.section.clone(),
                code_element: None,
                mismatch_type: "not_implemented".to_string(),
                details: format!("Tests generated but never executed for: {}", req.description),
            });
        } else if has_passing && !has_failing {
            covered += 1;
        } else if has_passing && has_failing {
            covered += 1;
            mismatches.push(Mismatch {
                id: Uuid::new_v4().to_string(),
                report_id: report_id.clone(),
                requirement_id: req.id.clone(),
                spec_section: req.section.clone(),
                code_element: None,
                mismatch_type: "partial_coverage".to_string(),
                details: format!("Some tests passing, some failing for: {}", req.description),
            });
        } else if has_failing {
            mismatches.push(Mismatch {
                id: Uuid::new_v4().to_string(),
                report_id: report_id.clone(),
                requirement_id: req.id.clone(),
                spec_section: req.section.clone(),
                code_element: None,
                mismatch_type: "test_failing".to_string(),
                details: format!("Test(s) failing for: {}", req.description),
            });
        }
    }

    let coverage_percent = if total > 0 {
        (covered as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    let report = AlignmentReport {
        id: report_id,
        project_id: project_id.to_string(),
        coverage_percent,
        total_requirements: total,
        covered_requirements: covered,
        generated_at: Utc::now().to_rfc3339(),
    };

    queries::insert_alignment_report(conn, &report)?;
    for mismatch in &mismatches {
        queries::insert_mismatch(conn, mismatch)?;
    }

    Ok(AlignmentReportWithMismatches { report, mismatches })
}
