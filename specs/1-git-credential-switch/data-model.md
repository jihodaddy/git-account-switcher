# Data Model: Git Account Switcher

## Entities

### Account

앱에 등록된 Git 계정 정보. Stronghold에 암호화 저장.

| Field        | Type     | Required | Description                              |
|--------------|----------|----------|------------------------------------------|
| id           | UUID     | Yes      | 고유 식별자                               |
| display_name | String   | Yes      | 사용자가 지정한 표시 이름 (예: "회사", "개인") |
| git_username | String   | Yes      | Git user.name 값                         |
| git_email    | String   | Yes      | Git user.email 값                        |
| host         | String   | Yes      | Git 호스트 URL (예: github.com)           |
| host_type    | Enum     | Yes      | GitHub, GitLab, Bitbucket, Custom        |
| auth_token   | String   | Yes      | Personal Access Token 또는 비밀번호 (암호화) |
| is_active    | Boolean  | Yes      | 현재 활성 상태 여부                        |
| created_at   | DateTime | Yes      | 생성 시각                                 |
| updated_at   | DateTime | Yes      | 마지막 수정 시각                           |

**Uniqueness**: (host, git_email) 조합은 고유해야 한다.

**Validation Rules**:
- display_name: 1~50자
- git_email: 이메일 형식
- host: 유효한 도메인 형식
- auth_token: 비어 있을 수 없음

### RepositoryBinding

특정 로컬 저장소에 연결된 계정 매핑.

| Field          | Type   | Required | Description                     |
|----------------|--------|----------|---------------------------------|
| id             | UUID   | Yes      | 고유 식별자                      |
| repo_path      | String | Yes      | 로컬 저장소 절대 경로            |
| account_id     | UUID   | Yes      | 연결된 Account의 ID (FK)        |
| created_at     | DateTime | Yes    | 생성 시각                        |

**Uniqueness**: repo_path는 고유해야 한다 (하나의 저장소에 하나의 계정만 연결).

**Validation Rules**:
- repo_path: 유효한 디렉토리 경로이며 .git 폴더가 존재해야 함

### AppSettings

앱 전역 설정.

| Field       | Type    | Required | Description                  |
|-------------|---------|----------|------------------------------|
| theme       | Enum    | Yes      | Light, Dark, System          |
| language    | Enum    | Yes      | ko, en                      |
| start_minimized | Boolean | Yes  | 시작 시 트레이로 최소화 여부   |
| auto_start  | Boolean | Yes      | Windows 시작 시 자동 실행     |

## State Transitions

### Account Activation Flow

```
Inactive ──[사용자가 "전환" 클릭]──> Validating
Validating ──[토큰 유효]──> Active (이전 Active 계정은 Inactive로)
Validating ──[토큰 무효]──> Active (경고 표시, 전환은 허용)
Validating ──[네트워크 불가]──> Active (검증 건너뜀)
```

**불변 조건**: 동일 호스트에 대해 최대 1개의 Active 계정만 존재 가능.

## Relationships

```
Account 1──N RepositoryBinding
  (하나의 Account에 여러 Repository가 연결 가능)
```
