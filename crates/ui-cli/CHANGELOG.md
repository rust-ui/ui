# rust-ui CLI — Changelog

## Unreleased

---

## 0.3.15

### Fixed

- Removed incorrect nightly Rust warning. Leptos works on stable; nightly is only needed for the rust-ui dev workspace itself.

---

## 0.3.13

### Added

- `ui add` — warns when the project is not configured for nightly Rust (missing `rust-toolchain.toml` with `channel = "nightly"` or `leptos` without `features = ["nightly"]`); prompts to continue or abort

---

## 0.3.11

### Added

- `ui init` — interactive base color picker (Neutral / Stone / Zinc / Mauve / Olive / Mist / Taupe) and accent color picker (Default / Amber / Blue / … / Yellow); selections are saved to `ui_config.toml` and the generated `tailwind.css` uses the exact OKLCH values; `--yes` / `--force` skips prompts and uses defaults
- `ui add` — deprecated component warnings: warns and exits when a deprecated component is requested, pointing to the replacement (e.g. `toast` → use `sonner`)
- `ui init --yes` / `-y` and `--force` / `-f` — skip confirmation prompts and force overwrite existing files without prompting
- `ui init --reinstall` — re-download and overwrite all already-installed components after init; prompts automatically when existing components are detected on a re-run
- `ui init` — backup/restore on failure: `ui_config.toml` is backed up before writing and automatically restored if the init process fails (RAII `FileBackup` guard)

---

## 0.3.10

### Fixed

- MCP server now correctly advertises the `tools` capability in the initialize response, so Claude Code and other clients properly discover and register tools on session start

---

## 0.3.9

### Added

- `ui mcp` — starts a stdio MCP server exposing rust-ui registry tools to AI editors (Claude Code, Cursor, VS Code, OpenCode)
- `ui mcp init --client <claude|cursor|vscode|opencode>` — writes the editor config file so your AI editor auto-connects to the MCP server on startup
- MCP tools available: `list_components`, `search_components`, `view_component`, `get_add_command`, `get_audit_checklist`
- `ui view <name>` — prints a component's source from the registry without installing it; supports `--json`
- `ui add --path <dir>` — override the output directory for components, bypassing `base_path_components` from `ui_config.toml`
- `ui diff` — shows a line-by-line diff of installed components vs the registry; supports a single component (`ui diff button`) or all installed components at once; supports `--json` for machine-readable output

---

## 0.3.8

### Added

- `ui update` — checks all installed components against the registry; reports `up to date`, `outdated`, or `not in registry` per component; suggests the exact `ui add <name> -y` command to fix each; supports `--json`
- `ui search <query>` — filters the registry by name (case-insensitive); supports `--json` for scripted output
- `ui list --json` — machine-readable JSON output for `ui list` (`{ total, categories: { ... } }`)
- `ui list` — lists all available components from the registry grouped by category (grep-friendly, one component per line)
- `ui info --json` — machine-readable JSON output for `ui info`, useful for scripting and AI tooling
- `ui docs` — opens `https://rust-ui.com` in the system default browser (cross-platform: `open` / `xdg-open` / `start`)
- `ui add --dry-run` / `-n` — resolves all dependencies and previews which files would be written, overwritten, or skipped without touching the filesystem; output is sorted for determinism

---

## 0.3.7

### Added

- `ui info` — prints project config (`ui_config.toml`), base color, base path, workspace detection, and all installed components with count
- `ui add --yes` / `-y` — skips the overwrite prompt and forces all files to be written
- Overwrite prompt on `add` — when a component file already exists, the user is asked before overwriting (requires a TTY; bypassed with `--yes`)
- Explicit summary after `add`:
  - `✅ Added:` — newly written files
  - `⏭  Skipped:` — existing files the user chose not to overwrite
  - `📦 Dep already installed:` — auto-resolved dependency components already on disk (no silent skips)

### Changed

- Auto-resolved dependency components that are already installed no longer trigger the overwrite prompt; they are reported separately in the summary

---

## 0.3.6

### Added

- `ui add` reads tailwind input file path from `[package.metadata.leptos]` in `Cargo.toml`
- Workspace-aware Cargo dep injection: detects workspace root and uses `[workspace.dependencies]` when available

### Changed

- Removed deprecated starter templates
- Upgraded ratatui to 0.30

---

## 0.3.5

### Added

- JS file dependency support in `add`: downloads JS files to `public/` alongside Rust components

---

## 0.3.4

### Added

- Interactive TUI picker (ratatui) with tabs: Components, Hooks, Blocks, Icons, Demos, Settings
- Installed components highlighted in TUI list
- Dependency detail panel in TUI
- Footer keyboard shortcuts (`Ctrl+letter` to jump between tabs)
- Unit tests for TUI logic

---

## 0.1.5

### Added

- `ui starters` — choose and clone starter templates (Tauri, Tauri Fullstack)
- Registry-based component fetching (`add` reads from remote registry)
- Automatic `mod.rs` registration on `add`
- Automatic `pub mod components` registration in `lib.rs` / `main.rs`
- Cargo dependency injection on `add`

---

## 0.1.4 and earlier

- Initial `ui init` command: scaffolds `ui_config.toml`, installs Tailwind CSS config, wires Leptos dependencies
- Initial `ui add <components>` command: fetches components from registry by name with dependency resolution
- Workspace detection and multi-crate support
