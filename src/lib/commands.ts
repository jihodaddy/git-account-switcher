import { invoke } from "@tauri-apps/api/core";
import type {
  Account,
  CreateAccountInput,
  UpdateAccountInput,
  GitUser,
  SwitchResult,
  ValidateResult,
  RepositoryBinding,
  AppSettings,
} from "./types";

export async function getAccounts(): Promise<Account[]> {
  return invoke("get_accounts");
}

export async function getAccount(id: string): Promise<Account> {
  return invoke("get_account", { id });
}

export async function createAccount(input: CreateAccountInput): Promise<Account> {
  return invoke("create_account", { input });
}

export async function updateAccount(input: UpdateAccountInput): Promise<Account> {
  return invoke("update_account", { input });
}

export async function deleteAccount(id: string): Promise<void> {
  return invoke("delete_account", { id });
}

export async function switchAccount(id: string): Promise<SwitchResult> {
  return invoke("switch_account", { id });
}

export async function validateToken(id: string): Promise<ValidateResult> {
  return invoke("validate_token", { id });
}

export async function getCurrentGitUser(): Promise<GitUser> {
  return invoke("get_current_git_user");
}

export async function getBindings(): Promise<RepositoryBinding[]> {
  return invoke("get_bindings");
}

export async function bindRepository(
  repoPath: string,
  accountId: string,
): Promise<RepositoryBinding> {
  return invoke("bind_repository", { repoPath, accountId });
}

export async function unbindRepository(id: string): Promise<void> {
  return invoke("unbind_repository", { id });
}

export async function getSettings(): Promise<AppSettings> {
  return invoke("get_settings");
}

export async function updateSettings(settings: Partial<AppSettings>): Promise<AppSettings> {
  return invoke("update_settings", { settings });
}
