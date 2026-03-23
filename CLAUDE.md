# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**killports** is a desktop application built with Tauri v2 + SvelteKit + TypeScript. The app uses Rust for the backend (native OS integration) and SvelteKit in SPA mode as the frontend, compiled to static files.

The package manager is **bun** (configured in `tauri.conf.json` via `beforeDevCommand`/`beforeBuildCommand`).

## Commands

```bash
# Frontend only (Vite dev server on port 1420)
bun run dev

# Full Tauri app (Rust backend + frontend, recommended for development)
bunx tauri dev

# Type-check the frontend
bun run check
bun run check:watch   # watch mode

# Build production desktop bundle
bunx tauri build

# Build frontend only
bun run build
```

## Architecture

The repo has two distinct layers:

### Frontend — `src/`
SvelteKit configured as a **pure SPA** (`adapter-static` with `fallback: "index.html"`). There is no SSR — Tauri has no Node server. The frontend communicates with Rust via Tauri's `invoke()` API.

### Backend — `src-tauri/`
Rust crate (`tauri_app_lib`) that exposes commands to the frontend via `#[tauri::command]`. Commands are registered in `src-tauri/src/lib.rs` inside `invoke_handler!`. `main.rs` is a thin entry point that calls `tauri_app_lib::run()`.

Key files:
- `src-tauri/src/lib.rs` — Tauri commands and app builder (start here when adding Rust functionality)
- `src-tauri/tauri.conf.json` — app metadata, window config, build hooks, bundle targets
- `src-tauri/capabilities/default.json` — Tauri permission system (controls what the frontend can access)
- `src-tauri/Cargo.toml` — Rust dependencies

### Frontend ↔ Backend Communication
Frontend calls Rust via `@tauri-apps/api`:
```ts
import { invoke } from "@tauri-apps/api/core";
const result = await invoke("command_name", { arg: value });
```

New Rust commands must be: (1) annotated with `#[tauri::command]`, (2) added to `invoke_handler!(tauri::generate_handler![...])` in `lib.rs`, and (3) if they need new OS-level permissions, added to `src-tauri/capabilities/default.json`.

### Vite Configuration
Port **1420** is fixed (`strictPort: true`) — Tauri's dev mode expects this exact port. The `src-tauri/` directory is excluded from Vite's file watcher to avoid conflicts with Cargo.
