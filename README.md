# Git Account Switcher (GAS)

Windows에서 여러 Git 계정을 손쉽게 전환할 수 있는 데스크톱 앱입니다.

개인용, 회사용 등 복수의 Git 계정을 등록하고, 클릭 한 번으로 자격증명과 Git 설정을 전환합니다.

![Windows](https://img.shields.io/badge/Windows%2010+-0078D6?logo=windows&logoColor=white)
![Tauri](https://img.shields.io/badge/Tauri%20v2-24C8D8?logo=tauri&logoColor=white)
![React](https://img.shields.io/badge/React%2019-61DAFB?logo=react&logoColor=black)
![Rust](https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=white)

## 왜 필요한가?

Windows에서 여러 Git 계정을 사용하려면 수동으로 자격증명 관리자를 편집하거나, `.gitconfig`를 직접 수정해야 합니다. 이 과정은 번거롭고, 잘못된 계정으로 커밋하거나 인증 실패가 반복되는 문제를 야기합니다.

**GAS**는 이 문제를 해결합니다.

## 주요 기능

- **계정 관리** — Git 계정 등록/수정/삭제 (GitHub, GitLab, Bitbucket, Custom)
- **1-Click 전환** — 활성 계정을 선택하면 Windows Credential Manager + Git global config 자동 변경
- **GCM 공존** — Git Credential Manager를 비활성화하지 않고 자격증명 항목을 직접 갱신
- **토큰 검증** — 전환 시 호스트 API로 토큰 유효성 확인 (네트워크 불가 시 건너뜀)
- **저장소 바인딩** — 특정 로컬 저장소에 개별 계정 연결 (로컬 git config)
- **다크/라이트 테마** — 모던한 UI, 다크 모드 기본

## 설치

### 인스톨러 다운로드

[Releases](../../releases) 페이지에서 최신 버전을 다운로드하세요:

- `Git Account Switcher_x.x.x_x64-setup.exe` (NSIS, 권장)
- `Git Account Switcher_x.x.x_x64_en-US.msi` (MSI)

### 요구 사항

- Windows 10 이상
- Git이 시스템에 설치되어 있어야 합니다

## 사용법

1. 앱 실행 → **Add Account** 클릭
2. Host Type 선택 (GitHub, GitLab 등), 이름/이메일/토큰 입력
3. 계정 카드에서 **Switch** 버튼 클릭 → 전환 완료

전환 후 `git push`, `git pull` 등이 선택한 계정으로 인증됩니다.

## 작동 원리

계정 전환 시 다음이 자동으로 수행됩니다:

1. Windows Credential Manager에서 `git:https://[host]` 항목 삭제
2. 새 계정의 자격증명으로 교체 (GCM이 자동으로 인식)
3. `git config --global user.name` 변경
4. `git config --global user.email` 변경

## 개발 환경 설정

### Prerequisites

- Node.js 18+
- Rust 1.77+
- Visual Studio Build Tools (C++ 빌드 도구)

### 실행

```bash
# 의존성 설치
npm install

# 개발 서버 실행
npm run tauri dev

# 프로덕션 빌드
npm run tauri build
```

## 기술 스택

| Layer    | Technology                                |
|----------|-------------------------------------------|
| Frontend | React 19 + TypeScript + Vite              |
| Backend  | Rust (Tauri v2)                           |
| Storage  | 암호화된 로컬 JSON                         |
| Auth     | Windows Credential Manager API (Win32)    |
| Git      | subprocess (`git config`)                 |
| UI       | CSS custom properties (다크/라이트 테마)    |

## 프로젝트 구조

```
src/                    # React Frontend
├── components/         # UI 컴포넌트
├── lib/                # 타입 정의, Tauri IPC 래퍼
└── index.css           # 테마 스타일

src-tauri/              # Rust Backend
├── src/
│   ├── commands/       # Tauri command 핸들러
│   ├── credential.rs   # Windows Credential Manager 통합
│   ├── git.rs          # Git config 관리
│   ├── storage.rs      # 데이터 저장소
│   └── validation.rs   # 토큰 유효성 검증
└── Cargo.toml
```

## License

MIT
