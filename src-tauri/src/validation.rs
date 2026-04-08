use crate::models::{HostType, ValidateResult};

pub async fn validate_token(
    host: &str,
    host_type: &HostType,
    token: &str,
) -> ValidateResult {
    let url = match host_type {
        HostType::Github => "https://api.github.com/user".to_string(),
        HostType::Gitlab => format!("https://{}/api/v4/user", host),
        HostType::Bitbucket => "https://api.bitbucket.org/2.0/user".to_string(),
        HostType::Custom => {
            return ValidateResult {
                valid: true,
                message: Some("skipped_custom".to_string()),
            };
        }
    };

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .user_agent("git-account-switcher/0.1")
        .build();

    let client = match client {
        Ok(c) => c,
        Err(_) => {
            return ValidateResult {
                valid: true,
                message: Some("network_unavailable".to_string()),
            };
        }
    };

    let request = match host_type {
        HostType::Github | HostType::Bitbucket => {
            client.get(&url).header("Authorization", format!("Bearer {}", token))
        }
        HostType::Gitlab => {
            client.get(&url).header("PRIVATE-TOKEN", token)
        }
        HostType::Custom => unreachable!(),
    };

    match request.send().await {
        Ok(response) => {
            if response.status().is_success() {
                ValidateResult {
                    valid: true,
                    message: None,
                }
            } else {
                ValidateResult {
                    valid: false,
                    message: Some(format!("HTTP {}", response.status())),
                }
            }
        }
        Err(e) => {
            if e.is_timeout() || e.is_connect() {
                ValidateResult {
                    valid: true,
                    message: Some("network_unavailable".to_string()),
                }
            } else {
                ValidateResult {
                    valid: false,
                    message: Some(e.to_string()),
                }
            }
        }
    }
}
