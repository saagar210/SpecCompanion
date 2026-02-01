use crate::services::git_service::{self, RepoInfo, ChangedFile};
use crate::errors::AppError;

#[tauri::command]
pub fn get_repo_info(path: String) -> Result<RepoInfo, AppError> {
    if path.trim().is_empty() {
        return Err(AppError::InvalidInput("Path cannot be empty".into()));
    }
    validate_git_path(&path)?;
    git_service::get_repo_info(&path)
}

#[tauri::command]
pub fn get_changed_files(path: String, since_commit: Option<String>) -> Result<Vec<ChangedFile>, AppError> {
    if path.trim().is_empty() {
        return Err(AppError::InvalidInput("Path cannot be empty".into()));
    }
    validate_git_path(&path)?;
    git_service::get_changed_files(&path, since_commit.as_deref())
}

fn validate_git_path(path: &str) -> Result<(), AppError> {
    let canonical = std::fs::canonicalize(path).map_err(AppError::Io)?;
    let home = crate::utils::home_dir()
        .ok_or_else(|| AppError::General("Cannot determine home directory".into()))?;
    if !canonical.starts_with(&home) {
        return Err(AppError::InvalidInput("Access denied: path is outside home directory".into()));
    }
    Ok(())
}
