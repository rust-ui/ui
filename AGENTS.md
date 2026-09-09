# AGENTS.md

## Crates (`crates/`)

`leptos_ui` · `icons` · `tw_merge` · `tw_merge_variants` · `ui-cli` · `_starters` · `_markdown_config` · `_markdown_crate` · `autoform`


## Instructions

### Adding new demos

1. Check if a similar demo exists to reuse the pattern.
2. Create the demo and add it in `mod.rs`.
3. Add the demo doc in `public/docs/*` following existing patterns.
4. Run `cargo run` from `build_registry/` — auto-generates `public/registry/*` and `__registry__`.

### CHANGELOG

- Write every user-facing change to `CHANGELOG_DEV.md` first (top entry, dated `## YYYY-MM-DD`).
- Do **not** edit `public/docs/changelog.md` directly. The user reviews `CHANGELOG_DEV.md` and promotes entries to `public/docs/changelog.md` when ready.
- Skip internal refactors, styling tweaks, and build changes.

