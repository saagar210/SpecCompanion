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
    queries::get_project(&conn, &project_id)?;
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
            // Add summary header
            let mut csv = format!(
                "# Spec Alignment Report\n# Generated: {}\n# Coverage: {:.1}% ({}/{} requirements)\n# Total Mismatches: {}\n\n",
                report.report.generated_at,
                report.report.coverage_percent,
                report.report.covered_requirements,
                report.report.total_requirements,
                report.mismatches.len(),
            );

            // Add column headers
            csv.push_str("requirement_id,spec_section,mismatch_type,code_element,details\n");

            // Add mismatches
            for m in &report.mismatches {
                csv.push_str(&format!(
                    "{},{},{},{},{}\n",
                    escape_csv(&m.requirement_id),
                    escape_csv(&m.spec_section),
                    escape_csv(&m.mismatch_type),
                    escape_csv(m.code_element.as_deref().unwrap_or("")),
                    escape_csv(&m.details),
                ));
            }
            Ok(csv)
        }
        "html" => {
            // Calculate summary statistics
            let mut mismatch_counts = std::collections::HashMap::new();
            for m in &report.mismatches {
                *mismatch_counts.entry(m.mismatch_type.as_str()).or_insert(0) += 1;
            }

            let total_mismatches = report.mismatches.len();
            let coverage = report.report.coverage_percent;
            let total_reqs = report.report.total_requirements;
            let covered_reqs = report.report.covered_requirements;

            let mut html = String::from(
                r#"<!DOCTYPE html><html><head><meta charset="utf-8"><title>Alignment Report</title>
<style>
* { box-sizing: border-box; }
body { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif; margin: 0; padding: 2em; background: #0f0f1e; color: #e4e4f0; }
.header { background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); padding: 2em; border-radius: 12px; margin-bottom: 2em; }
.header h1 { margin: 0 0 0.5em 0; font-size: 2em; }
.header p { margin: 0.25em 0; font-size: 1.1em; opacity: 0.95; }
.summary-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 1em; margin-bottom: 2em; }
.summary-card { background: #1e1e2e; border: 1px solid #333348; border-radius: 8px; padding: 1.5em; }
.summary-card h3 { margin: 0 0 0.5em 0; font-size: 0.9em; text-transform: uppercase; opacity: 0.7; font-weight: 500; }
.summary-card .value { font-size: 2em; font-weight: 600; margin: 0.25em 0; }
.summary-card .label { font-size: 0.85em; opacity: 0.8; }
.coverage-bar { height: 24px; background: #252538; border-radius: 12px; overflow: hidden; margin: 1em 0; }
.coverage-fill { height: 100%; background: linear-gradient(90deg, #10b981 0%, #059669 100%); transition: width 0.3s; }
table { border-collapse: collapse; width: 100%; background: #1e1e2e; border-radius: 8px; overflow: hidden; }
th, td { border-bottom: 1px solid #333348; padding: 12px 16px; text-align: left; }
th { background: #252538; font-weight: 600; font-size: 0.9em; text-transform: uppercase; letter-spacing: 0.5px; }
tr:last-child td { border-bottom: none; }
tr:hover { background: #252538; }
.badge { padding: 4px 12px; border-radius: 6px; font-size: 0.85em; font-weight: 500; display: inline-block; }
.no_test_generated { background: #eab308; color: #000; }
.test_failing { background: #ef4444; color: #fff; }
.not_implemented { background: #6366f1; color: #fff; }
.partial_coverage { background: #f97316; color: #fff; }
.section { background: #1e1e2e; padding: 1.5em; border-radius: 8px; margin-bottom: 2em; }
.section h2 { margin: 0 0 1em 0; font-size: 1.3em; }
.footer { text-align: center; margin-top: 3em; padding-top: 2em; border-top: 1px solid #333348; opacity: 0.6; font-size: 0.9em; }
.progress-text { display: flex; justify-content: space-between; margin-bottom: 0.5em; font-size: 0.9em; }
</style></head><body>"#,
            );

            // Header section
            html.push_str(&format!(
                r#"<div class="header">
                    <h1>📊 Spec Alignment Report</h1>
                    <p>Generated: {}</p>
                    <div class="progress-text">
                        <span>{} of {} requirements covered</span>
                        <span><strong>{:.1}%</strong></span>
                    </div>
                    <div class="coverage-bar">
                        <div class="coverage-fill" style="width: {:.1}%"></div>
                    </div>
                </div>"#,
                report.report.generated_at,
                covered_reqs,
                total_reqs,
                coverage,
                coverage,
            ));

            // Summary cards
            html.push_str("<div class=\"summary-grid\">");
            html.push_str(&format!(
                r#"<div class="summary-card">
                    <h3>Total Requirements</h3>
                    <div class="value">{}</div>
                </div>
                <div class="summary-card">
                    <h3>Covered</h3>
                    <div class="value" style="color: #10b981;">{}</div>
                </div>
                <div class="summary-card">
                    <h3>Mismatches</h3>
                    <div class="value" style="color: #f97316;">{}</div>
                </div>
                <div class="summary-card">
                    <h3>Coverage</h3>
                    <div class="value">{:.1}%</div>
                </div>"#,
                total_reqs, covered_reqs, total_mismatches, coverage,
            ));
            html.push_str("</div>");

            // Mismatch breakdown
            if !mismatch_counts.is_empty() {
                html.push_str("<div class=\"section\"><h2>Mismatch Breakdown</h2>");
                html.push_str("<table><thead><tr><th>Type</th><th>Count</th><th>Percentage</th></tr></thead><tbody>");
                let mut sorted_counts: Vec<_> = mismatch_counts.iter().collect();
                sorted_counts.sort_by_key(|(_, count)| std::cmp::Reverse(**count));
                for (mismatch_type, count) in sorted_counts {
                    let percentage = (*count as f64 / total_mismatches as f64) * 100.0;
                    html.push_str(&format!(
                        "<tr><td><span class=\"badge {}\">{}</span></td><td>{}</td><td>{:.1}%</td></tr>",
                        html_escape(mismatch_type),
                        html_escape(&mismatch_type.replace('_', " ")),
                        count,
                        percentage,
                    ));
                }
                html.push_str("</tbody></table></div>");
            }

            // Detailed mismatches
            if report.mismatches.is_empty() {
                html.push_str("<div class=\"section\"><p>✅ No mismatches found. All requirements are covered!</p></div>");
            } else {
                html.push_str("<div class=\"section\"><h2>Detailed Mismatches</h2>");
                html.push_str("<table><thead><tr><th>Section</th><th>Type</th><th>Details</th></tr></thead><tbody>");
                for m in &report.mismatches {
                    html.push_str(&format!(
                        "<tr><td>{}</td><td><span class=\"badge {}\">{}</span></td><td>{}</td></tr>",
                        html_escape(&m.spec_section),
                        html_escape(&m.mismatch_type),
                        html_escape(&m.mismatch_type.replace('_', " ")),
                        html_escape(&m.details),
                    ));
                }
                html.push_str("</tbody></table></div>");
            }

            html.push_str("<div class=\"footer\">Generated by SpecCompanion</div>");
            html.push_str("</body></html>");
            Ok(html)
        }
        _ => Err(AppError::InvalidInput(format!("Unsupported format: {}", format))),
    }
}

fn escape_csv(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') || s.contains('\r') {
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
