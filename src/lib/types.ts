export type HostType = "github" | "gitlab" | "bitbucket" | "custom";

export interface Account {
  id: string;
  display_name: string;
  git_username: string;
  git_email: string;
  host: string;
  host_type: HostType;
  is_active: boolean;
  created_at: string;
  updated_at: string;
}

export interface CreateAccountInput {
  display_name: string;
  git_username: string;
  git_email: string;
  host: string;
  host_type: HostType;
  auth_token: string;
}

export interface UpdateAccountInput {
  id: string;
  display_name?: string;
  git_username?: string;
  git_email?: string;
  host?: string;
  host_type?: HostType;
  auth_token?: string;
}

export interface GitUser {
  name: string;
  email: string;
}

export interface SwitchResult {
  success: boolean;
  validation_result: "valid" | "invalid" | "skipped";
}

export interface ValidateResult {
  valid: boolean;
  message?: string;
}

export interface RepositoryBinding {
  id: string;
  repo_path: string;
  account_id: string;
  account_name?: string;
  created_at: string;
}

export interface AppSettings {
  theme: "light" | "dark" | "system";
  language: "ko" | "en";
  start_minimized: boolean;
  auto_start: boolean;
}
