# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**killports** is a desktop application built with Tauri v2 + SvelteKit + TypeScript. The app monitors and displays all active network ports on the device, with advanced filtering capabilities and detailed port information.

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

**Component Structure:**
```
src/
├── routes/
│   └── +page.svelte              # Main page - manages state and orchestrates components
├── components/
│   ├── header/
│   │   └── Header.svelte         # Filter controls and search
│   ├── content/
│   │   └── ShowPorts.svelte      # Port list table with selection
│   └── aside/
│       └── Aside.svelte          # Detailed port information panel
├── interfaces/
│   └── Ports.ts                  # TypeScript interfaces (PortInfo, PortFilters)
└── assets/
    ├── eye.svg                   # View icon
    └── kill.svg                  # Kill process icon
```

**Key Features:**
- Real-time port monitoring (TCP/UDP)
- Advanced filtering (protocol, state, address type, port ranges)
- Search by port number, process name, or IP address
- Detailed port information view
- Process identification with PID
- Cross-platform support (Windows, macOS, Linux)

**State Management:**
- Parent component (`+page.svelte`) manages all filter state
- Two-way binding with child components using `bind:` and `$bindable()`
- Reactive filters using `$derived` for computed values
- Port selection state passed via callbacks

### Backend — `src-tauri/`

Rust crate (`tauri_app_lib`) that exposes commands to the frontend via `#[tauri::command]`.

**Structure:**
```
src-tauri/
├── src/
│   ├── lib.rs                    # Main entry point, registers commands
│   ├── main.rs                   # Thin wrapper, calls lib.rs
│   └── commands/
│       ├── mod.rs                # Exports commands
│       └── get_ports.rs          # Port monitoring implementation
├── Cargo.toml                    # Dependencies (netstat2, sysinfo)
├── tauri.conf.json               # App config
└── capabilities/
    └── default.json              # Permission system
```

**Key Dependencies:**
- `netstat2 = "0.11.2"` - Cross-platform network socket information
- `sysinfo = "0.30"` - Process information (names, PIDs)
- `tauri = "2"` - Desktop app framework
- `serde = "1"` - Serialization for Rust ↔ JS communication

**Available Commands:**
1. `get_active_ports(filters: PortFilters)` - Returns filtered list of active ports
2. `get_available_states()` - Returns list of TCP/UDP connection states

### Frontend ↔ Backend Communication

Frontend calls Rust via `@tauri-apps/api`:
```ts
import { invoke } from "@tauri-apps/api/core";

// Fetch ports with filters
const ports = await invoke<PortInfo[]>("get_active_ports", { filters });

// Get available states for dropdown
const states = await invoke<string[]>("get_available_states");
```

**Data Flow:**
1. User interacts with filters in `Header.svelte`
2. Filter state updates in `+page.svelte` via two-way binding
3. Filters are computed using `$derived` into `PortFilters` object
4. `ShowPorts.svelte` receives filters and calls Rust backend
5. Rust returns filtered port data
6. User clicks port → `Aside.svelte` shows detailed info

### Type Definitions

**PortInfo** (Rust & TypeScript):
```typescript
interface PortInfo {
  local_port: number;
  local_address: string;
  remote_port: number;
  remote_address: string;
  state: string;              // "Listen", "Established", "TimeWait", etc.
  protocol: string;           // "TCP" or "UDP"
  process_name: string | null;
  pid: number | null;
}
```

**PortFilters** (Rust & TypeScript):
```typescript
interface PortFilters {
  search_query: string | null;
  protocol_filter: string | null;      // "TCP", "UDP", or null
  state_filter: string | null;         // "Listen", "Established", etc.
  address_type: string | null;         // "all", "localhost", "network"
  hide_system_processes: boolean;
  hide_ephemeral_ports: boolean;       // Ports 49152+
  port_range_min: number | null;
  port_range_max: number | null;
}
```

### Rust Backend Implementation

New Rust commands must be:
1. Annotated with `#[tauri::command]`
2. Added to `invoke_handler!(tauri::generate_handler![...])` in `lib.rs`
3. If they need new OS-level permissions, added to `src-tauri/capabilities/default.json`

**Example:**
```rust
// In src-tauri/src/commands/my_command.rs
#[tauri::command]
pub fn my_command(arg: String) -> Result<String, String> {
    Ok(format!("Received: {}", arg))
}

// In src-tauri/src/commands/mod.rs
pub mod my_command;
pub use my_command::my_command;

// In src-tauri/src/lib.rs
mod commands;

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::get_ports::get_active_ports,
            commands::get_ports::get_available_states,
            commands::my_command  // Add here
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### Svelte 5 Patterns

This project uses **Svelte 5** with runes:
- `$state` for reactive state
- `$derived` for computed values
- `$effect` for side effects
- `$props()` for component props
- `$bindable()` for two-way binding props
- **No `createEventDispatcher`** - use callback props instead

**Component Props Pattern:**
```svelte
<script lang="ts">
  // Child component
  interface Props {
    value: string;
    onChange?: (value: string) => void;
  }
  let { value = $bindable(), onChange }: Props = $props();
</script>

<!-- Parent component -->
<ChildComponent bind:value onchange={handleChange} />
```

### Vite Configuration

Port **1420** is fixed (`strictPort: true`) — Tauri's dev mode expects this exact port. The `src-tauri/` directory is excluded from Vite's file watcher to avoid conflicts with Cargo.

## Development Notes

- **Cross-platform**: Code works on Windows, macOS, and Linux, but may require elevated privileges to see all process names
- **Performance**: `$effect` in `ShowPorts.svelte` automatically refetches data when filters change
- **Filtering**: All filtering happens in Rust backend for performance
- **Process Names**: Uses `sysinfo` to resolve PIDs to process names; may return `None` if insufficient permissions

## Future Features (Placeholders)

- Kill process functionality (button exists, not yet implemented)
- Auto-refresh toggle
- Export port list to CSV/JSON
- Port history tracking
- Notification when specific ports open/close
