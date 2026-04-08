# Implementation Plan: Git Account Switcher

## Technical Context

| Item                | Choice                                      |
|---------------------|----------------------------------------------|
| Framework           | Tauri v2 (Rust backend + WebView frontend)   |
| Frontend            | React 19 + TypeScript + Vite                 |
| UI Library          | Tailwind CSS + shadcn/ui                     |
| Credential Storage  | Tauri Stronghold (AES-256 암호화)             |
| Credential Manager  | Windows API (CredWriteW/CredReadW/CredDeleteW) via `windows` crate |
| Git Integration     | tauri-plugin-shell (subprocess)              |
| HTTP Client         | reqwest (토큰 검증)                           |
| System Tray         | Tauri 내장 tray API                          |
| Target OS           | Windows 10+                                  |

## Implementation Phases

### Phase 1: 프로젝트 초기 설정

**Goal**: Tauri v2 + React 프로젝트 스캐폴딩 및 빌드 확인

- [ ] `npm create tauri-app`으로 프로젝트 생성 (react-ts 템플릿)
- [ ] Tailwind CSS + shadcn/ui 초기 설정
- [ ] Tauri 플러그인 설치 (stronghold, shell, dialog, autostart)
- [ ] Rust 의존성 추가 (windows, uuid, serde, reqwest, chrono)
- [ ] 프로젝트 구조 생성 (commands/, models, credential, git, storage 모듈)
- [ ] `npm run tauri dev` 빌드 성공 확인

**Deliverable**: 빈 앱이 빌드/실행되는 상태

---

### Phase 2: 핵심 백엔드 - 자격증명 엔진

**Goal**: Rust 백엔드의 핵심 기능 구현 (UI 없이 동작 검증)

#### 2-1: 데이터 모델 및 Stronghold 저장소
- [ ] Account, RepositoryBinding, AppSettings 모델 정의 (models.rs)
- [ ] Stronghold 초기화 및 CRUD 함수 구현 (storage.rs)
  - 계정 저장/조회/수정/삭제
  - auth_token 암호화 저장/복호화 조회
- [ ] 단위 테스트: 저장/조회/수정/삭제 동작 확인

#### 2-2: Windows Credential Manager 통합
- [ ] `windows` crate로 Credential Manager API 래핑 (credential.rs)
  - `write_credential(host, username, token)`: `git:https://[host]` 키로 자격증명 저장
  - `read_credential(host)`: 현재 저장된 자격증명 조회
  - `delete_credential(host)`: 자격증명 삭제
- [ ] 단위 테스트: 실제 Credential Manager에 쓰기/읽기/삭제

#### 2-3: Git Config 관리
- [ ] tauri-plugin-shell 활용 git config 함수 구현 (git.rs)
  - `set_global_user(name, email)`
  - `get_global_user() -> (name, email)`
  - `set_local_user(repo_path, name, email)`
- [ ] 단위 테스트: git config 변경 확인

#### 2-4: 토큰 유효성 검증
- [ ] 호스트별 검증 로직 구현 (validation.rs)
  - GitHub: `GET https://api.github.com/user` (Authorization: Bearer)
  - GitLab: `GET https://[host]/api/v4/user` (PRIVATE-TOKEN)
  - Bitbucket: `GET https://api.bitbucket.org/2.0/user` (Authorization: Bearer)
  - Custom: 검증 건너뜀
- [ ] 네트워크 불가 시 graceful skip 구현
- [ ] 단위 테스트

#### 2-5: 계정 전환 오케스트레이션
- [ ] `switch_account` 통합 함수 구현
  1. 토큰 유효성 검증 (실패해도 계속)
  2. 기존 호스트 Credential Manager 항목 삭제
  3. 새 자격증명 쓰기
  4. Git global config 변경
  5. 이전 활성 계정 비활성화, 새 계정 활성화
- [ ] 통합 테스트: 전환 후 git config + Credential Manager 상태 확인

**Deliverable**: CLI에서도 동작 가능한 완전한 백엔드 엔진

---

### Phase 3: Tauri Command 레이어

**Goal**: Frontend-Backend IPC 바인딩

- [ ] Tauri command 핸들러 구현 (commands/*.rs)
  - account: get_accounts, get_account, create_account, update_account, delete_account, switch_account, validate_token
  - binding: get_bindings, bind_repository, unbind_repository
  - settings: get_settings, update_settings
  - system: get_current_git_user, open_folder_dialog
- [ ] 에러 타입 정의 및 Frontend-friendly 에러 응답 포맷
- [ ] Tauri app builder에 command 등록
- [ ] TypeScript 타입 정의 (types.ts) 및 invoke 래퍼 (commands.ts)

**Deliverable**: Frontend에서 IPC로 모든 백엔드 기능 호출 가능

---

### Phase 4: Frontend UI - 메인 화면

**Goal**: 현대적이고 모던한 GUI 구현

#### 4-1: 레이아웃 및 테마
- [ ] 앱 전체 레이아웃 (사이드바 네비게이션 + 메인 콘텐츠)
- [ ] 다크/라이트 테마 지원 (shadcn/ui theme provider)
- [ ] 글로벌 스타일 설정 (폰트, 색상, 간격)
- [ ] 디자인 컨셉: 모던 미니멀, 부드러운 라운드 코너, 미묘한 그림자, 부드러운 애니메이션

#### 4-2: 계정 목록 및 상태 표시
- [ ] AccountList: 등록된 계정 카드 목록
  - 활성 계정 강조 표시 (초록색 배지 / 글로우 효과)
  - 호스트별 아이콘 (GitHub, GitLab, Bitbucket 로고)
  - 빠른 전환 버튼 (1-click switch)
- [ ] StatusBar: 현재 활성 계정 상단 표시
  - 호스트 아이콘 + 이름 + 이메일
  - 마지막 전환 시간

#### 4-3: 계정 등록/수정 폼
- [ ] AccountForm: 모달 또는 슬라이드 패널
  - 호스트 타입 선택 (드롭다운 + 아이콘)
  - 입력 필드: 표시 이름, user.name, user.email, 호스트 URL, 토큰
  - 토큰 입력 시 마스킹 처리 (보기/숨기기 토글)
  - 실시간 유효성 검사 (이메일 형식 등)
  - 토큰 검증 버튼 (연결 테스트)

#### 4-4: 계정 전환 UX
- [ ] 전환 클릭 시 확인 다이얼로그 (현재 → 새 계정)
- [ ] 전환 중 로딩 상태 표시
- [ ] 전환 완료/실패 토스트 알림
- [ ] 토큰 무효 경고 시 경고 배너 + 재입력 유도

**Deliverable**: 계정 CRUD + 전환이 가능한 메인 UI

---

### Phase 5: Frontend UI - 부가 기능

#### 5-1: 저장소 연결 관리
- [ ] RepoBindings: 저장소-계정 매핑 목록
- [ ] 폴더 선택 다이얼로그로 저장소 경로 추가
- [ ] 연결된 계정 표시 및 변경/해제

#### 5-2: 설정 화면
- [ ] Settings: 테마, 언어, 시작 옵션
- [ ] 자동 시작 설정 (tauri-plugin-autostart)

#### 5-3: 시스템 트레이
- [ ] 트레이 아이콘 등록 (현재 활성 계정 표시)
- [ ] 트레이 컨텍스트 메뉴: 계정 목록 + 빠른 전환
- [ ] 창 닫기 시 트레이로 최소화 옵션

**Deliverable**: 완전한 기능의 데스크톱 앱

---

### Phase 6: 품질 보증 및 빌드

- [ ] E2E 테스트: 계정 등록 → 전환 → git push 시나리오
- [ ] 에러 시나리오 테스트: 잘못된 토큰, 네트워크 불가, Credential Manager 접근 불가
- [ ] 앱 아이콘 및 스플래시 화면 설정
- [ ] Windows 인스톨러 빌드 (NSIS / MSI)
- [ ] 번들 크기 최적화
- [ ] README.md 작성

**Deliverable**: 배포 가능한 Windows 설치 파일

## UI Design Direction

사용자 요청: **"현대적이고 모던한 스타일"**

- **Color Palette**: 다크 테마 기본, 딥 네이비/차콜 배경 + 액센트 컬러 (바이올렛 또는 블루)
- **Typography**: Inter 또는 Pretendard (한글 지원) 폰트
- **Components**: shadcn/ui 기반 - 유리형 카드(glassmorphism 터치), 부드러운 hover 전환
- **Layout**: 사이드바 네비게이션 (아이콘 + 라벨), 넓은 메인 콘텐츠 영역
- **Animation**: Framer Motion - 페이지 전환, 카드 등장, 전환 성공 피드백
- **Account Cards**: 호스트별 브랜드 컬러 악센트, 활성 계정은 글로우 보더
- **System Tray**: 모노크롬 아이콘, 다크/라이트 자동 감지

## Risk Mitigations

| Risk                    | Mitigation                                            |
|------------------------|-------------------------------------------------------|
| GCM 캐시 충돌           | CredWrite 후 GCM 캐시 무효화 확인, 필요 시 GCM 설정 조정 가이드 |
| Stronghold 비밀번호 분실 | 앱 최초 실행 시 마스터 비밀번호 설정, 복구 불가 경고 표시       |
| 관리자 권한 필요         | Credential Manager는 일반 사용자 권한으로 접근 가능하므로 불필요 |
| Windows 버전 호환성      | Windows 10 1903+ 타겟, CI에서 다중 버전 테스트              |

## Dependencies Graph

```
Phase 1 (프로젝트 설정)
  └── Phase 2 (백엔드 엔진)
        ├── Phase 3 (Tauri Commands)
        │     └── Phase 4 (메인 UI)
        │           └── Phase 5 (부가 기능)
        │                 └── Phase 6 (빌드/배포)
        └── Phase 2 내부: 2-1 → 2-2, 2-3, 2-4 (병렬) → 2-5
```
