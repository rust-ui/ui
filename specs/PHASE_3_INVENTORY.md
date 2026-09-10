# Phase 3 cleanup inventory

Baseline:

- Parent branch: `phase-3-remove-leptos-parent`
- Parent rollback commit: record before production cleanup commit
- First production `leptos-ui` commit: record from deployment log
- Current `leptos-ui` commit: `d1283eb`

## Delete: migrated website source

Exact copies verified against `leptos-ui`:

- `app/`
- `server/`
- `app_crates/`
- `crates/leptos_ui/`
- `crates/autoform/`
- `crates/_markdown_crate/`
- `style/`
- `e2e/`

## Delete: migrated website/deployment wiring

- Root Leptos `Cargo.toml` members, dependencies, and metadata.
- Root Docker build and production compose files.
- Root Node/Tailwind/Playwright manifests and package config.
- Parent Leptos production workflow.
- Root Tauri app, platform helpers, and desktop release workflow; replacement lives in `leptos-ui`.

## Retain: shared or separately owned

- `public/registry/`: shared generated registry input for retained `ui-cli` tooling.
- Root `Cargo.lock`: regenerated for retained shared workspace crates.
- `public/.well-known/security.txt`: repository security metadata.
- `nginx.conf`: shared Rust UI and Dioxus virtual-host infrastructure.
- `crates/tw_merge/`, `crates/ui-cli/`, `crates/_markdown_config/`, and starter submodules.
- `crates/icons/`: retained for now; nested repository has uncommitted user changes and needs separate ownership decision.
- `dioxus-ui`: retained as separate Dioxus source until dedicated Dioxus merge phase.
- Repository governance, specs, and shared automation after stale Leptos references are removed.

## Unknown / follow-up gates

- Validate SQLite persistence after container recreation.
- Run rollback drill against previous parent deployment.
- Validate Tauri targets from `leptos-ui`.
- Repair parent `.gitmodules` mapping for `crates/_starters/start-dioxus-fullstack`.
