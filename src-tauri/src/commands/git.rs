use crate::services::git_service::{self, RepoInfo, ChangedFile};
use crate::errors::AppError;

#[tauri::command]
pub fn get_repo_info(path: String) -> Result<RepoInfo, AppError> {
    git_service::get_repo_info(&path)
}

#[tauri::command]
pub fn get_changed_files(path: String, since_commit: Option<String>) -> Result<Vec<ChangedFile>, AppError> {
    git_service::get_changed_files(&path, since_commit.as_deref())
}
