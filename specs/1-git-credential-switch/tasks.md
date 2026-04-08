# Tasks: Git Account Switcher

## Phase 1: Project Setup

- [x] T1.1: Tauri v2 + React + TypeScript 프로젝트 생성
- [x] T1.2: CSS 스타일 시스템 설정 (모던 다크/라이트 테마)
- [x] T1.3: Rust 의존성 추가 (windows, uuid, serde, reqwest, chrono, tokio)
- [x] T1.4: Tauri 플러그인 설치 (shell, dialog, autostart)
- [x] T1.5: 프로젝트 디렉토리 구조 생성
- [x] T1.6: .gitignore 생성
- [x] T1.7: Frontend 빌드 성공 확인 (TypeScript + Vite)

## Phase 2: Core Backend

- [x] T2.1: 데이터 모델 정의 (models.rs)
- [x] T2.2: JSON 파일 저장소 구현 (storage.rs) - Account CRUD
- [x] T2.3: Windows Credential Manager 통합 (credential.rs)
- [x] T2.4: Git Config 관리 (git.rs)
- [x] T2.5: 토큰 유효성 검증 (validation.rs)
- [x] T2.6: 계정 전환 오케스트레이션 (commands/account.rs switch_account)

## Phase 3: Tauri Command Layer

- [x] T3.1: Account commands 구현
- [x] T3.2: Repository binding commands 구현
- [x] T3.3: Settings & System commands 구현
- [x] T3.4: TypeScript 타입 정의 및 invoke 래퍼
- [x] T3.5: Tauri app builder에 commands 등록

## Phase 4: Frontend UI - Main

- [x] T4.1: 앱 레이아웃 및 테마 설정 (다크/라이트)
- [x] T4.2: 계정 목록 (AccountList + AccountCard)
- [x] T4.3: 계정 등록/수정 폼 (AccountForm)
- [x] T4.4: 계정 전환 UX (확인 다이얼로그, 로딩, 토스트)
- [x] T4.5: 현재 활성 계정 상태 표시 (StatusBar)

## Phase 5: Frontend UI - Additional

- [x] T5.1: 저장소 연결 관리 (RepoBindings)
- [x] T5.2: 설정 화면 (Settings)
- [ ] T5.3: 시스템 트레이 통합 (향후 구현)

## Phase 6: Build & Polish

- [x] T6.1: Rust + VS Build Tools 설치
- [x] T6.2: cargo check 성공
- [x] T6.3: 프로덕션 빌드 성공
- [x] T6.4: NSIS 인스톨러 생성 (3.1MB)
- [x] T6.5: MSI 인스톨러 생성 (4.6MB)
