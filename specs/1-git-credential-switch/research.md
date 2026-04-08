# Research: Git Account Switcher

## Decision 1: Application Framework

- **Decision**: Tauri v2 + React 19 + TypeScript
- **Rationale**: Tauri v2는 Rust 기반 백엔드로 Windows Credential Manager에 네이티브 접근이 가능하고, React 프론트엔드로 현대적인 UI 구현이 용이하다. Electron 대비 번들 크기가 작고(~10MB vs ~100MB+), 메모리 사용량이 적다. Stronghold 플러그인으로 자격증명 암호화 저장이 내장 지원된다.
- **Alternatives considered**:
  - Electron + React: 번들 크기가 크고 메모리 사용량이 높음
  - WPF/WinUI 3: .NET 런타임 의존, 웹 기술 대비 UI 커스터마이징 제한
  - Flutter: Windows 지원이 상대적으로 미성숙, Credential Manager 접근에 별도 플러그인 필요

## Decision 2: UI 프레임워크/스타일링

- **Decision**: Tailwind CSS + shadcn/ui 컴포넌트
- **Rationale**: 사용자가 요청한 "현대적이고 모던한 스타일"을 구현하기 위해 shadcn/ui의 세련된 컴포넌트를 활용한다. Tailwind CSS는 빌드 타임에 최적화되어 번들 크기를 최소화하고, 다크/라이트 테마 전환이 용이하다.
- **Alternatives considered**:
  - Material UI: Google 스타일로 범용적이나 차별화 부족
  - Ant Design: 엔터프라이즈 스타일로 개인 도구에는 과도함

## Decision 3: 자격증명 저장 방식

- **Decision**: Tauri Stronghold 플러그인 (암호화된 로컬 DB)
- **Rationale**: IOTA의 Stronghold 엔진은 AES-256 암호화로 시크릿을 저장하며, Tauri 공식 플러그인으로 통합이 용이하다. 앱 자체 계정 데이터(토큰, 비밀번호)를 안전하게 저장하는 데 적합하다.
- **Alternatives considered**:
  - Windows DPAPI 직접 사용: 로우레벨 구현이 필요하고 오류 가능성 높음
  - SQLCipher: 추가 바이너리 의존성 발생

## Decision 4: Windows Credential Manager 접근

- **Decision**: Rust FFI로 Windows Credential Manager API 직접 호출 (CredWriteW, CredReadW, CredDeleteW)
- **Rationale**: GCM이 `git:https://[hostname]` 형식으로 자격증명을 저장하므로, 동일 키에 대해 CredWrite로 덮어쓰면 즉시 반영된다. 별도 GCM 재시작이 불필요하다. `windows` crate를 사용하여 안전한 FFI 바인딩을 구현한다.
- **Alternatives considered**:
  - cmdkey.exe 호출: 서브프로세스 방식으로 느리고 파싱이 번거로움
  - 서드파티 credential crate: 유지보수 불확실

## Decision 5: Git 설정 변경

- **Decision**: tauri-plugin-shell을 통한 git config 명령 실행
- **Rationale**: `git config --global user.name/user.email` 명령을 서브프로세스로 실행하는 것이 가장 안정적이다. git2 crate는 config 변경에는 과도하고, 시스템에 설치된 Git의 설정 파일을 직접 수정하는 것이 호환성이 높다.
- **Alternatives considered**:
  - git2 crate: 라이브러리 링킹 복잡도 증가, libgit2 번들 필요
  - .gitconfig 파일 직접 편집: 파싱 오류 위험

## Decision 6: 시스템 트레이

- **Decision**: Tauri 내장 시스템 트레이 API
- **Rationale**: Tauri v2는 Windows 시스템 트레이를 네이티브 지원한다. .ico 파일 기반 아이콘, 컨텍스트 메뉴, 클릭 이벤트 처리가 모두 내장되어 있어 추가 라이브러리가 불필요하다.

## Decision 7: 토큰 유효성 검증

- **Decision**: 호스트별 API 엔드포인트에 인증 요청으로 검증
- **Rationale**: GitHub는 `GET /user` (api.github.com), GitLab는 `GET /api/v4/user`, Bitbucket는 `GET /2.0/user`로 토큰 유효성을 검증한다. Rust의 reqwest crate로 HTTP 요청을 보내고, 네트워크 불가 시 검증을 건너뛴다.
