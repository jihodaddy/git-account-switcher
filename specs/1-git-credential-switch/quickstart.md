# Quickstart: Git Account Switcher

## Prerequisites

- Windows 10 이상
- Node.js 18+
- Rust 1.75+ (rustup)
- Git
- Visual Studio Build Tools (C++ 빌드 도구)

## Project Setup

```bash
# Tauri v2 + React 프로젝트 생성
npm create tauri-app@latest git-account-switcher -- --template react-ts

# 프로젝트 디렉토리로 이동
cd git-account-switcher

# Frontend 의존성 설치
npm install

# UI 라이브러리 설치
npx shadcn@latest init
npm install tailwindcss @tailwindcss/vite
npm install lucide-react

# Tauri 플러그인 설치 (Cargo.toml에 추가)
# tauri-plugin-stronghold  - 암호화 저장
# tauri-plugin-shell       - git 명령 실행
# tauri-plugin-dialog       - 폴더 선택
# tauri-plugin-autostart    - 자동 시작

# Rust 의존성 (Cargo.toml)
# windows crate            - Windows Credential Manager API
# uuid                     - 고유 ID 생성
# serde / serde_json       - 직렬화
# reqwest                  - HTTP (토큰 검증)
# chrono                   - 날짜/시간
```

## Project Structure

```
git-account-switcher/
├── src/                        # React Frontend
│   ├── components/
│   │   ├── ui/                 # shadcn/ui 컴포넌트
│   │   ├── AccountList.tsx     # 계정 목록
│   │   ├── AccountForm.tsx     # 계정 등록/수정 폼
│   │   ├── AccountCard.tsx     # 개별 계정 카드
│   │   ├── RepoBindings.tsx    # 저장소 연결 관리
│   │   ├── StatusBar.tsx       # 현재 활성 계정 표시
│   │   └── Settings.tsx        # 설정 화면
│   ├── lib/
│   │   ├── commands.ts         # Tauri IPC invoke 래퍼
│   │   └── types.ts            # TypeScript 타입 정의
│   ├── App.tsx
│   └── main.tsx
├── src-tauri/                  # Rust Backend
│   ├── src/
│   │   ├── main.rs
│   │   ├── commands/           # Tauri command 핸들러
│   │   │   ├── account.rs
│   │   │   ├── binding.rs
│   │   │   ├── settings.rs
│   │   │   └── system.rs
│   │   ├── credential.rs       # Windows Credential Manager 통합
│   │   ├── git.rs              # Git config 명령 실행
│   │   ├── storage.rs          # Stronghold 암호화 저장
│   │   ├── validation.rs       # 토큰 유효성 검증
│   │   └── models.rs           # 데이터 모델
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   └── icons/
└── specs/                      # Feature specs
```

## Dev Commands

```bash
# 개발 서버 실행 (hot reload)
npm run tauri dev

# 프로덕션 빌드
npm run tauri build

# Rust 테스트
cd src-tauri && cargo test

# Frontend 테스트
npm test
```
