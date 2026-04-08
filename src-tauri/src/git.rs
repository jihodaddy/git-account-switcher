use std::process::Command;

fn run_git(args: &[&str]) -> Result<String, String> {
    let output = Command::new("git")
        .args(args)
        .output()
        .map_err(|e| format!("Failed to run git: {}", e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        Err(format!("git error: {}", stderr))
    }
}

pub fn get_global_user() -> Result<(String, String), String> {
    let name = run_git(&["config", "--global", "user.name"]).unwrap_or_default();
    let email = run_git(&["config", "--global", "user.email"]).unwrap_or_default();
    Ok((name, email))
}

pub fn set_global_user(name: &str, email: &str) -> Result<(), String> {
    run_git(&["config", "--global", "user.name", name])?;
    run_git(&["config", "--global", "user.email", email])?;
    Ok(())
}

pub fn set_local_user(repo_path: &str, name: &str, email: &str) -> Result<(), String> {
    let output = Command::new("git")
        .args(["config", "user.name", name])
        .current_dir(repo_path)
        .output()
        .map_err(|e| format!("Failed to run git: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "git config user.name failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let output = Command::new("git")
        .args(["config", "user.email", email])
        .current_dir(repo_path)
        .output()
        .map_err(|e| format!("Failed to run git: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "git config user.email failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(())
}

pub fn is_git_repo(path: &str) -> bool {
    Command::new("git")
        .args(["rev-parse", "--git-dir"])
        .current_dir(path)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Configure per-directory credential auto-switching using gitconfig includeIf.
/// This writes a directory-specific gitconfig file and adds an includeIf directive
/// to the global gitconfig so that the correct credential is used automatically.
pub fn setup_per_directory_credential(
    repo_path: &str,
    username: &str,
    email: &str,
    host: &str,
) -> Result<String, String> {
    // Normalize path to use forward slashes and ensure trailing slash
    let normalized = repo_path.replace('\\', "/");
    let gitdir_pattern = if normalized.ends_with('/') {
        normalized.clone()
    } else {
        format!("{}/", normalized)
    };

    // Create a per-repo gitconfig file inside the repo's .git directory
    let config_path = format!("{}/.git/config.gas", repo_path);

    // Write the per-repo config with credential settings
    let credential_url = format!("https://{}@{}", username, host);
    let config_content = format!(
        "[user]\n\tname = {}\n\temail = {}\n[credential \"https://{}\"]\n\tusername = {}\n\tuseHttpPath = true\n",
        username, email, host, username
    );

    std::fs::write(&config_path, &config_content)
        .map_err(|e| format!("Failed to write per-repo config: {}", e))?;

    // Add includeIf to global gitconfig
    let include_key = format!("includeIf.gitdir:{}.path", gitdir_pattern);
    let _ = run_git(&["config", "--global", &include_key, &config_path]);

    // Also enable useHttpPath globally for this host so GCM stores credentials per-user
    let _ = run_git(&[
        "config",
        "--global",
        &format!("credential.https://{}.useHttpPath", host),
        "true",
    ]);

    Ok(credential_url)
}

/// Remove per-directory credential configuration
pub fn remove_per_directory_credential(repo_path: &str) -> Result<(), String> {
    let normalized = repo_path.replace('\\', "/");
    let gitdir_pattern = if normalized.ends_with('/') {
        normalized
    } else {
        format!("{}/", normalized)
    };

    // Remove includeIf from global gitconfig
    let include_key = format!("includeIf.gitdir:{}.path", gitdir_pattern);
    let _ = run_git(&["config", "--global", "--unset", &include_key]);

    // Remove the per-repo config file
    let config_path = format!("{}/.git/config.gas", repo_path);
    let _ = std::fs::remove_file(&config_path);

    Ok(())
}
