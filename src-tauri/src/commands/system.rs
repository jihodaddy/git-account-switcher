use crate::git;
use crate::models::{AppSettings, GitUser};
use crate::storage::Storage;
use tauri::State;

#[tauri::command]
pub fn get_current_git_user() -> Result<GitUser, String> {
    let (name, email) = git::get_global_user()?;
    Ok(GitUser { name, email })
}

#[tauri::command]
pub fn get_settings(storage: State<Storage>) -> AppSettings {
    storage.get_settings()
}

#[tauri::command]
pub fn update_settings(
    settings: AppSettings,
    storage: State<Storage>,
) -> AppSettings {
    storage.update_settings(settings)
}
