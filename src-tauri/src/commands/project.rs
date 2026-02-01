use tauri::State;
use crate::db::Database;
use crate::db::queries;
use crate::models::project::{CreateProjectRequest, ProjectWithStats, Project};
use crate::errors::AppError;

#[tauri::command]
pub fn create_project(
    state: State<'_, Database>,
    request: CreateProjectRequest,
) -> Result<Project, AppError> {
    if request.name.trim().is_empty() {
        return Err(AppError::InvalidInput("Project name cannot be empty".into()));
    }
    if request.codebase_path.trim().is_empty() {
        return Err(AppError::InvalidInput("Codebase path cannot be empty".into()));
    }
    let path = std::path::Path::new(&request.codebase_path);
    if !path.exists() || !path.is_dir() {
        return Err(AppError::InvalidInput(format!(
            "Codebase path does not exist or is not a directory: {}",
            request.codebase_path
        )));
    }
    // Store canonicalized path to prevent symlink/.. traversal issues downstream
    let canonical = std::fs::canonicalize(path).map_err(AppError::Io)?;
    let mut canonical_request = request;
    canonical_request.codebase_path = canonical.to_string_lossy().to_string();
    let conn = state.conn.lock().map_err(|e| AppError::General(e.to_string()))?;
    queries::create_project(&conn, &canonical_request)
}

#[tauri::command]
pub fn list_projects(state: State<'_, Database>) -> Result<Vec<ProjectWithStats>, AppError> {
    let conn = state.conn.lock().map_err(|e| AppError::General(e.to_string()))?;
    queries::list_projects(&conn)
}

#[tauri::command]
pub fn get_project(state: State<'_, Database>, id: String) -> Result<ProjectWithStats, AppError> {
    if id.trim().is_empty() {
        return Err(AppError::InvalidInput("Project ID cannot be empty".into()));
    }
    let conn = state.conn.lock().map_err(|e| AppError::General(e.to_string()))?;
    queries::get_project(&conn, &id)
}

#[tauri::command]
pub fn delete_project(state: State<'_, Database>, id: String) -> Result<(), AppError> {
    if id.trim().is_empty() {
        return Err(AppError::InvalidInput("Project ID cannot be empty".into()));
    }
    let conn = state.conn.lock().map_err(|e| AppError::General(e.to_string()))?;
    queries::delete_project(&conn, &id)
}

#[tauri::command]
pub fn validate_path(path: String) -> Result<bool, AppError> {
    let p = std::path::Path::new(&path);
    Ok(p.exists() && p.is_dir())
}
