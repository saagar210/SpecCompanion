use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Spec {
    pub id: String,
    pub project_id: String,
    pub filename: String,
    pub content: String,
    pub parsed_at: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Requirement {
    pub id: String,
    pub spec_id: String,
    pub section: String,
    pub description: String,
    pub req_type: String,
    pub priority: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct ParsedSpec {
    pub spec: Spec,
    pub requirements: Vec<Requirement>,
}
