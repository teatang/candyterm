# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

- **Development (Frontend)**: `pnpm dev`
- **Development (Tauri Desktop App)**: `pnpm tauri dev`
- **Type Check & Build (Frontend)**: `pnpm build`
- **Build (Tauri Desktop App)**: `pnpm tauri build`
- **Run Tests**: `pnpm test` (or `pnpm test:watch` for watch mode)
- **Run a Single Test**: `pnpm test src/path/to/test.ts` (or `npx vitest run <test-file>`)
- **Rust Backend Tests**: `cargo test --manifest-path src-tauri/Cargo.toml`
- **Run a Single Rust Test**: `cargo test --manifest-path src-tauri/Cargo.toml <test_name>`

## Architecture & Structure

Candyterm is a high-performance, minimalist SSH and SFTP desktop client built with **Tauri 2.x** (Rust backend) and **Vue 3** (TypeScript + Naive UI + TailwindCSS + Xterm.js).

- **`src/` (Frontend - Vue 3 + Vite)**:
  - `components/`: Modular UI components organized by domain (`auth/`, `connection/`, `sftp/`, `sidebar/`, `terminal/`).
  - `stores/`: Pinia state management for workspace state, SFTP file transfers, themes, and localization (`workspace.ts`, `sftp.ts`, etc.).
  - `locales/`: Internationalization data (`en-US.json`, `zh-CN.json`).
  - `views/`: Primary views (`MainWorkspace.vue`).
- **`src-tauri/` (Backend - Rust)**:
  - `src/ssh/`: SSH protocol handling via `russh`, session management, jump hosts (`session.rs`, `config.rs`, `jump.rs`).
  - `src/sftp/`: SFTP file transfer operations (`client.rs`).
  - `src/crypto/` & `src/vault/`: Secure master password derivation (Argon2), encryption/decryption (AES-GCM), and credential storage (`cipher.rs`, `kdf.rs`, `storage.rs`).
  - `src/commands.rs`: Tauri commands bridging frontend invocations to Rust backend services.
