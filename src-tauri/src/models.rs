use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum HostType {
    Github,
    Gitlab,
    Bitbucket,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub display_name: String,
    pub git_username: String,
    pub git_email: String,
    pub host: String,
    pub host_type: HostType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<String>,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl Account {
    pub fn new(
        display_name: String,
        git_username: String,
        git_email: String,
        host: String,
        host_type: HostType,
        auth_token: String,
    ) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            id: Uuid::new_v4().to_string(),
            display_name,
            git_username,
            git_email,
            host,
            host_type,
            auth_token: Some(auth_token),
            is_active: false,
            created_at: now.clone(),
            updated_at: now,
        }
    }

    pub fn without_token(&self) -> Account {
        Account {
            auth_token: None,
            ..self.clone()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryBinding {
    pub id: String,
    pub repo_path: String,
    pub account_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    pub created_at: String,
}

impl RepositoryBinding {
    pub fn new(repo_path: String, account_id: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            repo_path,
            account_id,
            account_name: None,
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAccountInput {
    pub display_name: String,
    pub git_username: String,
    pub git_email: String,
    pub host: String,
    pub host_type: HostType,
    pub auth_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAccountInput {
    pub id: String,
    pub display_name: Option<String>,
    pub git_username: Option<String>,
    pub git_email: Option<String>,
    pub host: Option<String>,
    pub host_type: Option<HostType>,
    pub auth_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitUser {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwitchResult {
    pub success: bool,
    pub validation_result: String, // "valid", "invalid", "skipped"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateResult {
    pub valid: bool,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub theme: String,
    pub language: String,
    pub start_minimized: bool,
    pub auto_start: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: "dark".to_string(),
            language: "ko".to_string(),
            start_minimized: false,
            auto_start: false,
        }
    }
}
