use crate::credential;
use crate::git;
use crate::models::*;
use crate::storage::Storage;
use crate::tray;
use crate::validation;
use tauri::State;

#[tauri::command]
pub fn get_accounts(storage: State<Storage>) -> Vec<Account> {
    storage.get_accounts()
}

#[tauri::command]
pub fn get_account(id: String, storage: State<Storage>) -> Result<Account, String> {
    storage
        .get_account_without_token(&id)
        .ok_or_else(|| "Account not found".to_string())
}

#[tauri::command]
pub fn create_account(input: CreateAccountInput, storage: State<Storage>) -> Result<Account, String> {
    let account = Account::new(
        input.display_name,
        input.git_username,
        input.git_email,
        input.host,
        input.host_type,
        input.auth_token,
    );
    Ok(storage.create_account(account))
}

#[tauri::command]
pub fn update_account(input: UpdateAccountInput, storage: State<Storage>) -> Result<Account, String> {
    let id = input.id.clone();
    storage
        .update_account(&id, input)
        .ok_or_else(|| "Account not found".to_string())
}

#[tauri::command]
pub fn delete_account(id: String, storage: State<Storage>) -> Result<bool, String> {
    // Get account info before deleting to clean up credentials
    if let Some(account) = storage.get_account(&id) {
        if account.is_active {
            let _ = credential::delete_credential(&account.host);
        }
    }
    Ok(storage.delete_account(&id))
}

#[tauri::command]
pub async fn switch_account(id: String, app_handle: tauri::AppHandle, storage: State<'_, Storage>) -> Result<SwitchResult, String> {
    let account = storage
        .get_account(&id)
        .ok_or_else(|| "Account not found".to_string())?;

    let token = account
        .auth_token
        .as_ref()
        .ok_or_else(|| "No token found".to_string())?;

    // Step 1: Validate token
    let validation = validation::validate_token(&account.host, &account.host_type, token).await;
    let validation_result = if validation.valid {
        if validation.message.as_deref() == Some("network_unavailable") {
            "skipped"
        } else {
            "valid"
        }
    } else {
        "invalid"
    };

    // Step 2: Delete existing credential for this host
    let _ = credential::delete_credential(&account.host);

    // Step 3: Write new credential
    credential::write_credential(&account.host, &account.git_username, token)?;

    // Step 4: Update git global config
    git::set_global_user(&account.git_username, &account.git_email)?;

    // Step 5: Update active status in storage
    storage.set_active(&id, &account.host);

    // Step 6: Update system tray menu
    let _ = tray::update_tray_menu(&app_handle);

    Ok(SwitchResult {
        success: true,
        validation_result: validation_result.to_string(),
    })
}

#[tauri::command]
pub async fn validate_token(id: String, storage: State<'_, Storage>) -> Result<ValidateResult, String> {
    let account = storage
        .get_account(&id)
        .ok_or_else(|| "Account not found".to_string())?;

    let token = account
        .auth_token
        .as_ref()
        .ok_or_else(|| "No token found".to_string())?;

    Ok(validation::validate_token(&account.host, &account.host_type, token).await)
}
