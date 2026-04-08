# Tauri Command Contracts (IPC API)

Frontend(React)와 Backend(Rust) 간 통신을 위한 Tauri Command 정의.

## Account Commands

### `get_accounts`
- **Direction**: Frontend → Backend
- **Input**: 없음
- **Output**: `Account[]` (auth_token 제외)
- **Description**: 등록된 모든 계정 목록 반환

### `get_account`
- **Direction**: Frontend → Backend
- **Input**: `{ id: string }`
- **Output**: `Account` (auth_token 제외)
- **Description**: 특정 계정 상세 정보 반환

### `create_account`
- **Direction**: Frontend → Backend
- **Input**: `{ display_name, git_username, git_email, host, host_type, auth_token }`
- **Output**: `Account` (생성된 계정)
- **Error**: `DuplicateAccount` (동일 host+email 존재 시)
- **Description**: 새 계정 등록. Stronghold에 암호화 저장.

### `update_account`
- **Direction**: Frontend → Backend
- **Input**: `{ id, display_name?, git_username?, git_email?, host?, host_type?, auth_token? }`
- **Output**: `Account` (수정된 계정)
- **Error**: `AccountNotFound`, `DuplicateAccount`
- **Description**: 기존 계정 정보 수정

### `delete_account`
- **Direction**: Frontend → Backend
- **Input**: `{ id: string }`
- **Output**: `{ success: boolean }`
- **Side Effects**: 활성 계정이면 Windows Credential Manager에서도 제거, 연결된 RepositoryBinding 삭제
- **Description**: 계정 삭제

### `switch_account`
- **Direction**: Frontend → Backend
- **Input**: `{ id: string }`
- **Output**: `{ success: boolean, validation_result: "valid" | "invalid" | "skipped" }`
- **Side Effects**:
  1. 해당 호스트의 기존 Windows Credential Manager 항목 삭제
  2. 새 계정의 자격증명으로 교체 (`git:https://[host]`)
  3. `git config --global user.name` 변경
  4. `git config --global user.email` 변경
  5. 이전 활성 계정을 Inactive로 변경
- **Error**: `AccountNotFound`, `CredentialManagerError`
- **Description**: 전역 활성 계정 전환

### `validate_token`
- **Direction**: Frontend → Backend
- **Input**: `{ id: string }`
- **Output**: `{ valid: boolean, message?: string }`
- **Description**: 호스트 API로 토큰 유효성 검증. 네트워크 불가 시 `{ valid: true, message: "network_unavailable" }`

## Repository Binding Commands

### `get_bindings`
- **Direction**: Frontend → Backend
- **Input**: 없음
- **Output**: `RepositoryBinding[]` (Account 정보 포함)

### `bind_repository`
- **Direction**: Frontend → Backend
- **Input**: `{ repo_path: string, account_id: string }`
- **Output**: `RepositoryBinding`
- **Side Effects**: 해당 저장소의 로컬 git config (user.name, user.email) 변경
- **Error**: `InvalidRepository`, `AccountNotFound`

### `unbind_repository`
- **Direction**: Frontend → Backend
- **Input**: `{ id: string }`
- **Output**: `{ success: boolean }`

## Settings Commands

### `get_settings`
- **Output**: `AppSettings`

### `update_settings`
- **Input**: `AppSettings` (partial)
- **Output**: `AppSettings`

## System Commands

### `get_current_git_user`
- **Output**: `{ name: string, email: string }`
- **Description**: 현재 git config --global에 설정된 사용자 정보 반환

### `open_folder_dialog`
- **Output**: `string | null`
- **Description**: 폴더 선택 다이얼로그 열기 (저장소 경로 선택용)
