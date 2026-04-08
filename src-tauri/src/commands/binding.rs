use crate::credential;
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

    let token = account
        .auth_token
        .as_ref()
        .ok_or_else(|| "Account has no token".to_string())?;

    // Set local git config (user.name, user.email)
    git::set_local_user(&repo_path, &account.git_username, &account.git_email)?;

    // Set up per-directory credential auto-switching (includeIf + useHttpPath)
    git::setup_per_directory_credential(
        &repo_path,
        &account.git_username,
        &account.git_email,
        &account.host,
    )?;

    // Write a per-user credential to Windows Credential Manager
    // This uses the format git:https://username@host so it doesn't conflict with the global credential
    credential::write_credential_for_user(&account.host, &account.git_username, token)?;

    // Save binding
    let binding = RepositoryBinding::new(repo_path, account_id);
    Ok(storage.create_binding(binding))
}

#[tauri::command]
pub fn unbind_repository(id: String, storage: State<Storage>) -> Result<bool, String> {
    // Get binding info before deleting to clean up
    if let Some(binding) = storage.get_binding(&id) {
        // Remove per-directory credential config
        let _ = git::remove_per_directory_credential(&binding.repo_path);

        // Remove the per-user credential if we know the account
        if let Some(account) = storage.get_account(&binding.account_id) {
            let _ = credential::delete_credential_for_user(&account.host, &account.git_username);
        }
    }

    Ok(storage.delete_binding(&id))
}
