use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub codebase_path: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub codebase_path: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct ProjectWithStats {
    #[serde(flatten)]
    pub project: Project,
    pub spec_count: i64,
    pub coverage_percent: Option<f64>,
    pub last_run_at: Option<String>,
}
