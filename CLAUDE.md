# CLAUDE.md

## Code Structure

### Workspace Structure

This is a Rust workspace with the following main components:

- **app/**: Main Leptos application (SSR + hydration)
- **server/**: Axum server for SSR
- **rust_ui_internals/build_registry/**: Build tool that processes `.md` files from `public/docs/*` and generates registry files in `app/src/__registry__/` (located in the private `rust_ui_internals` repo)

### Crates

- **crates/leptos_ui/**: Core UI components library
- **crates/icons/**: Lucide icon components for Leptos
- **crates/tw_merge/**: Tailwind CSS class merging utility
- **crates/tw_merge_variants/**: Tailwind variants extension
- **crates/ui-cli/**: CLI tool for the UI system
- **crates/_starters/**: Template projects for different setups
- **crates/_markdown_config/**: Shared config/types (serde + strum) for markdown processing
- **crates/_markdown_crate/**: Markdown-to-Leptos renderer (pulldown-cmark + syntax highlighting)
- **crates/autoform/**: Derive macro for automatic form generation


## Instructions

### SEO & Canonical URLs

- **CRITICAL**: Always use canonical URL `https://rust-ui.com` (NO www). Never use `https://www.rust-ui.com` in code, configs, or static files.

### build_registry/ (in rust_ui_internals)

- **LOCATION**: `rust_ui_internals/build_registry/` — part of the private `rust_ui_internals` repo.
- **IMPORTANT**: `public/registry/*` are created automatically by `build_registry`. YOU **DO NOT** need to recreate them.
- **IMPORTANT**: Demo documentation uses `.md` files in `public/docs/*` to show the Demo components.
- When working in `build_registry/`, **ALWAYS** run `cargo run` from `rust_ui_internals/build_registry/` to make sure all good.


### CHANGELOG

- **IMPORTANT**: Update `public/docs/changelog.md` every time a new feature, fix, or change is added. Add a new section at the top.
- Only include **user-facing** changes — new features, components, or meaningful improvements. Never add internal refactors, styling tweaks, or build changes.


### Adding new demos Workflow

1. Check first if a similar demo exists to use the same pattern (may not exist).
2. Create the demo and add it in mod.rs
3. Run `cargo run` from `rust_ui_internals/build_registry/` to build the `__registry__` with the new demo.


