# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

### Building and Testing
```bash
# Run tests (preferred)
cargo nextest run

# Run tests (fallback if nextest not installed)
cargo test

# For manual end-to-end testing, use the _TMP/ directory (gitignored) as a throwaway project.

# Run with specific commands (examples from main.rs)
cargo run --bin ui init
cargo run --bin ui add button demo_button demo_button_variants demo_button_sizes
cargo run --bin ui add demo_use_floating_placement
cargo run --bin ui starters
```


### Project Structure
```
crates/ui-cli/
├── src/
│   ├── command_add/          # Component installation logic
│   ├── command_diff/         # ui diff command
│   ├── command_docs/         # ui docs command
│   ├── command_info/         # ui info command
│   ├── command_init/         # Project initialization
│   ├── command_list/         # ui list command
│   ├── command_mcp/          # ui mcp server + mcp init
│   ├── command_search/       # ui search command
│   ├── command_starters/     # Starter template cloning
│   ├── command_update/       # ui update command
│   ├── command_view/         # ui view command
│   └── shared/               # Shared utilities
└── Cargo.toml               # Binary configuration
```

## Workflow Rules

- **CHANGELOG.md**: Update `CHANGELOG.md` every time a new feature, fix, or change is added. New work goes under `## Unreleased`. Keep entries concise and user-facing.
- **Version**: When bumping the crate version in `Cargo.toml`, move `## Unreleased` entries to the new version section in `CHANGELOG.md` at the same time.
  - **DEFAULT**: Always bump the **minor** version only (e.g. `0.3.7` → `0.3.8`). Never bump major or minor segment (e.g. `0.4.0`) without explicitly asking the user first.

