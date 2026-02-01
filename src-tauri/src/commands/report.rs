use tauri::State;
use crate::db::Database;
use crate::db::queries;
use crate::services::alignment;
use crate::models::report::{AlignmentReport, AlignmentReportWithMismatches};
use crate::errors::AppError;

#[tauri::command]
pub fn generate_alignment_report(
    state: State<'_, Database>,
    project_id: String,
) -> Result<AlignmentReportWithMismatches, AppError> {
    if project_id.trim().is_empty() {
        return Err(AppError::InvalidInput("Project ID cannot be empty".into()));
    }
    let conn = state.conn.lock().map_err(|e| AppError::General(e.to_string()))?;
    alignment::generate_report(&conn, &project_id)
}

#[tauri::command]
pub fn get_alignment_report(
    state: State<'_, Database>,
    id: String,
) -> Result<AlignmentReportWithMismatches, AppError> {
    if id.trim().is_empty() {
        return Err(AppError::InvalidInput("Report ID cannot be empty".into()));
    }
    let conn = state.conn.lock().map_err(|e| AppError::General(e.to_string()))?;
    queries::get_alignment_report(&conn, &id)
}

#[tauri::command]
pub fn list_reports(
    state: State<'_, Database>,
    project_id: String,
) -> Result<Vec<AlignmentReport>, AppError> {
    if project_id.trim().is_empty() {
        return Err(AppError::InvalidInput("Project ID cannot be empty".into()));
    }
    let conn = state.conn.lock().map_err(|e| AppError::General(e.to_string()))?;
    queries::list_reports(&conn, &project_id)
}

#[tauri::command]
pub fn export_report(
    state: State<'_, Database>,
    report_id: String,
    format: String,
) -> Result<String, AppError> {
    if report_id.trim().is_empty() {
        return Err(AppError::InvalidInput("Report ID cannot be empty".into()));
    }
    if !matches!(format.as_str(), "json" | "html" | "csv") {
        return Err(AppError::InvalidInput(format!("Unsupported format: {}", format)));
    }
    let conn = state.conn.lock().map_err(|e| AppError::General(e.to_string()))?;
    let report = queries::get_alignment_report(&conn, &report_id)?;

    match format.as_str() {
        "json" => {
            serde_json::to_string_pretty(&report).map_err(AppError::Serde)
        }
        "csv" => {
            let mut csv = String::from("requirement_id,spec_section,mismatch_type,code_element,details\n");
            for m in &report.mismatches {
                csv.push_str(&format!(
                    "{},{},{},{},{}\n",
                    m.requirement_id,
                    escape_csv(&m.spec_section),
                    m.mismatch_type,
                    m.code_element.as_deref().unwrap_or(""),
                    escape_csv(&m.details),
                ));
            }
            Ok(csv)
        }
        "html" => {
            let mut html = String::from(
                r#"<!DOCTYPE html><html><head><meta charset="utf-8"><title>Alignment Report</title>
<style>
body { font-family: -apple-system, sans-serif; margin: 2em; background: #1e1e2e; color: #e4e4f0; }
table { border-collapse: collapse; width: 100%; }
th, td { border: 1px solid #333348; padding: 8px 12px; text-align: left; }
th { background: #252538; }
.badge { padding: 2px 8px; border-radius: 4px; font-size: 0.85em; }
.no_test_generated { background: #eab308; color: #000; }
.test_failing { background: #ef4444; color: #fff; }
.not_implemented { background: #6366f1; color: #fff; }
.partial_coverage { background: #f97316; color: #fff; }
</style></head><body>"#,
            );
            html.push_str(&format!(
                "<h1>Alignment Report</h1><p>Coverage: <strong>{:.1}%</strong> ({}/{} requirements)</p>",
                report.report.coverage_percent,
                report.report.covered_requirements,
                report.report.total_requirements,
            ));

            if report.mismatches.is_empty() {
                html.push_str("<p>No mismatches found.</p>");
            } else {
                html.push_str("<table><thead><tr><th>Section</th><th>Type</th><th>Details</th></tr></thead><tbody>");
                for m in &report.mismatches {
                    html.push_str(&format!(
                        "<tr><td>{}</td><td><span class=\"badge {}\">{}</span></td><td>{}</td></tr>",
                        html_escape(&m.spec_section),
                        m.mismatch_type,
                        m.mismatch_type.replace('_', " "),
                        html_escape(&m.details),
                    ));
                }
                html.push_str("</tbody></table>");
            }

            html.push_str("</body></html>");
            Ok(html)
        }
        _ => Err(AppError::InvalidInput(format!("Unsupported format: {}", format))),
    }
}

fn escape_csv(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
