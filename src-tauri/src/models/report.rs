use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AlignmentReport {
    pub id: String,
    pub project_id: String,
    pub coverage_percent: f64,
    pub total_requirements: i64,
    pub covered_requirements: i64,
    pub generated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mismatch {
    pub id: String,
    pub report_id: String,
    pub requirement_id: String,
    pub spec_section: String,
    pub code_element: Option<String>,
    pub mismatch_type: String,
    pub details: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct AlignmentReportWithMismatches {
    #[serde(flatten)]
    pub report: AlignmentReport,
    pub mismatches: Vec<Mismatch>,
}
