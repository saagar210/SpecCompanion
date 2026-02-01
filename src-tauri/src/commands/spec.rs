use tauri::State;
use crate::db::Database;
use crate::db::queries;
use crate::models::spec::{Spec, Requirement, ParsedSpec};
use crate::services::spec_parser;
use crate::errors::AppError;

#[tauri::command]
pub fn upload_spec(
    state: State<'_, Database>,
    project_id: String,
    filename: String,
    content: String,
) -> Result<ParsedSpec, AppError> {
    if project_id.trim().is_empty() {
        return Err(AppError::InvalidInput("Project ID cannot be empty".into()));
    }
    if filename.trim().is_empty() {
        return Err(AppError::InvalidInput("Filename cannot be empty".into()));
    }
    if content.trim().is_empty() {
        return Err(AppError::InvalidInput("Spec content cannot be empty".into()));
    }
    // Sanitize filename — strip path components
    let safe_filename = std::path::Path::new(&filename)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(&filename)
        .to_string();

    let conn = state.conn.lock().map_err(|e| AppError::General(e.to_string()))?;

    queries::get_project(&conn, &project_id)?;

    let spec = queries::create_spec(&conn, &project_id, &safe_filename, &content)?;
    let requirements = spec_parser::parse_spec(&spec.id, &content);

    if !requirements.is_empty() {
        queries::insert_requirements(&conn, &requirements)?;
    }

    queries::update_spec_parsed_at(&conn, &spec.id)?;
    let updated_spec = queries::get_spec(&conn, &spec.id)?;

    Ok(ParsedSpec {
        spec: updated_spec,
        requirements,
    })
}

#[tauri::command]
pub fn get_spec(state: State<'_, Database>, id: String) -> Result<ParsedSpec, AppError> {
    if id.trim().is_empty() {
        return Err(AppError::InvalidInput("Spec ID cannot be empty".into()));
    }
    let conn = state.conn.lock().map_err(|e| AppError::General(e.to_string()))?;
    let spec = queries::get_spec(&conn, &id)?;
    let requirements = queries::get_requirements_for_spec(&conn, &id)?;
    Ok(ParsedSpec { spec, requirements })
}

#[tauri::command]
pub fn list_specs(state: State<'_, Database>, project_id: String) -> Result<Vec<Spec>, AppError> {
    if project_id.trim().is_empty() {
        return Err(AppError::InvalidInput("Project ID cannot be empty".into()));
    }
    let conn = state.conn.lock().map_err(|e| AppError::General(e.to_string()))?;
    queries::list_specs(&conn, &project_id)
}

#[tauri::command]
pub fn delete_spec(state: State<'_, Database>, id: String) -> Result<(), AppError> {
    if id.trim().is_empty() {
        return Err(AppError::InvalidInput("Spec ID cannot be empty".into()));
    }
    let conn = state.conn.lock().map_err(|e| AppError::General(e.to_string()))?;
    queries::delete_spec(&conn, &id)
}

#[tauri::command]
pub fn reparse_spec(state: State<'_, Database>, id: String) -> Result<Vec<Requirement>, AppError> {
    if id.trim().is_empty() {
        return Err(AppError::InvalidInput("Spec ID cannot be empty".into()));
    }
    let conn = state.conn.lock().map_err(|e| AppError::General(e.to_string()))?;
    let spec = queries::get_spec(&conn, &id)?;

    queries::delete_requirements_for_spec(&conn, &id)?;

    let requirements = spec_parser::parse_spec(&id, &spec.content);

    if !requirements.is_empty() {
        queries::insert_requirements(&conn, &requirements)?;
    }

    queries::update_spec_parsed_at(&conn, &id)?;

    Ok(requirements)
}

#[tauri::command]
pub fn read_file_content(path: String) -> Result<String, AppError> {
    if path.trim().is_empty() {
        return Err(AppError::InvalidInput("File path cannot be empty".into()));
    }
    let canonical = std::fs::canonicalize(&path).map_err(AppError::Io)?;
    // Block paths outside user home directory
    if let Some(home) = dirs_next_home() {
        if !canonical.starts_with(&home) {
            return Err(AppError::InvalidInput("Access denied: path is outside home directory".into()));
        }
    }
    std::fs::read_to_string(&canonical).map_err(AppError::Io)
}

fn dirs_next_home() -> Option<std::path::PathBuf> {
    #[cfg(target_os = "macos")]
    {
        std::env::var("HOME").ok().map(std::path::PathBuf::from)
    }
    #[cfg(not(target_os = "macos"))]
    {
        std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .ok()
            .map(std::path::PathBuf::from)
    }
}
