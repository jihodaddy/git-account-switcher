use crate::models::{Account, AppSettings, RepositoryBinding};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AppData {
    pub accounts: Vec<Account>,
    pub bindings: Vec<RepositoryBinding>,
    pub settings: AppSettings,
}

pub struct Storage {
    data: Mutex<AppData>,
    path: PathBuf,
}

impl Storage {
    pub fn new(app_dir: PathBuf) -> Self {
        let path = app_dir.join("data.json");
        let data = if path.exists() {
            match fs::read_to_string(&path) {
                Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
                Err(_) => AppData::default(),
            }
        } else {
            AppData::default()
        };

        Self {
            data: Mutex::new(data),
            path,
        }
    }

    fn save(&self, data: &AppData) {
        if let Some(parent) = self.path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(json) = serde_json::to_string_pretty(data) {
            let _ = fs::write(&self.path, json);
        }
    }

    // Account operations
    pub fn get_accounts(&self) -> Vec<Account> {
        let data = self.data.lock().unwrap();
        data.accounts.iter().map(|a| a.without_token()).collect()
    }

    pub fn get_account(&self, id: &str) -> Option<Account> {
        let data = self.data.lock().unwrap();
        data.accounts.iter().find(|a| a.id == id).cloned()
    }

    pub fn get_account_without_token(&self, id: &str) -> Option<Account> {
        self.get_account(id).map(|a| a.without_token())
    }

    pub fn create_account(&self, account: Account) -> Account {
        let mut data = self.data.lock().unwrap();
        data.accounts.push(account.clone());
        self.save(&data);
        account.without_token()
    }

    pub fn update_account(&self, id: &str, update: crate::models::UpdateAccountInput) -> Option<Account> {
        let mut data = self.data.lock().unwrap();
        if let Some(account) = data.accounts.iter_mut().find(|a| a.id == id) {
            if let Some(name) = update.display_name {
                account.display_name = name;
            }
            if let Some(username) = update.git_username {
                account.git_username = username;
            }
            if let Some(email) = update.git_email {
                account.git_email = email;
            }
            if let Some(host) = update.host {
                account.host = host;
            }
            if let Some(host_type) = update.host_type {
                account.host_type = host_type;
            }
            if let Some(token) = update.auth_token {
                account.auth_token = Some(token);
            }
            account.updated_at = chrono::Utc::now().to_rfc3339();
            let result = account.without_token();
            self.save(&data);
            Some(result)
        } else {
            None
        }
    }

    pub fn delete_account(&self, id: &str) -> bool {
        let mut data = self.data.lock().unwrap();
        let len_before = data.accounts.len();
        data.accounts.retain(|a| a.id != id);
        data.bindings.retain(|b| b.account_id != id);
        let removed = data.accounts.len() < len_before;
        if removed {
            self.save(&data);
        }
        removed
    }

    pub fn set_active(&self, id: &str, host: &str) {
        let mut data = self.data.lock().unwrap();
        for account in &mut data.accounts {
            if account.host == host {
                account.is_active = account.id == id;
            }
        }
        self.save(&data);
    }

    pub fn deactivate_for_host(&self, host: &str) {
        let mut data = self.data.lock().unwrap();
        for account in &mut data.accounts {
            if account.host == host {
                account.is_active = false;
            }
        }
        self.save(&data);
    }

    // Binding operations
    pub fn get_bindings(&self) -> Vec<RepositoryBinding> {
        let data = self.data.lock().unwrap();
        data.bindings
            .iter()
            .map(|b| {
                let mut binding = b.clone();
                binding.account_name = data
                    .accounts
                    .iter()
                    .find(|a| a.id == b.account_id)
                    .map(|a| a.display_name.clone());
                binding
            })
            .collect()
    }

    pub fn create_binding(&self, binding: RepositoryBinding) -> RepositoryBinding {
        let mut data = self.data.lock().unwrap();
        // Remove existing binding for same path
        data.bindings.retain(|b| b.repo_path != binding.repo_path);
        data.bindings.push(binding.clone());
        self.save(&data);
        binding
    }

    pub fn delete_binding(&self, id: &str) -> bool {
        let mut data = self.data.lock().unwrap();
        let len_before = data.bindings.len();
        data.bindings.retain(|b| b.id != id);
        let removed = data.bindings.len() < len_before;
        if removed {
            self.save(&data);
        }
        removed
    }

    // Settings
    pub fn get_settings(&self) -> AppSettings {
        let data = self.data.lock().unwrap();
        data.settings.clone()
    }

    pub fn update_settings(&self, settings: AppSettings) -> AppSettings {
        let mut data = self.data.lock().unwrap();
        data.settings = settings.clone();
        self.save(&data);
        settings
    }
}
