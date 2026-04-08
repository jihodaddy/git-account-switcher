use crate::git;
use crate::models::RepositoryBinding;
use crate::storage::Storage;
use tauri::State;

#[tauri::command]
pub fn get_bindings(storage: State<Storage>) -> Vec<RepositoryBinding> {
    storage.get_bindings()
}

#[tauri::command]
pub fn bind_repository(
    repo_path: String,
    account_id: String,
    storage: State<Storage>,
) -> Result<RepositoryBinding, String> {
    // Verify it's a git repo
    if !git::is_git_repo(&repo_path) {
        return Err(format!("{} is not a git repository", repo_path));
    }

    // Get account info
    let account = storage
        .get_account(&account_id)
        .ok_or_else(|| "Account not found".to_string())?;

    // Set local git config
    git::set_local_user(&repo_path, &account.git_username, &account.git_email)?;

    // Save binding
    let binding = RepositoryBinding::new(repo_path, account_id);
    Ok(storage.create_binding(binding))
}

#[tauri::command]
pub fn unbind_repository(id: String, storage: State<Storage>) -> Result<bool, String> {
    Ok(storage.delete_binding(&id))
}
