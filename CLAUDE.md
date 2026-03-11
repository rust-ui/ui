# CLAUDE.md

## Crates (`crates/`)

`leptos_ui` · `icons` · `tw_merge` · `tw_merge_variants` · `ui-cli` · `_starters` · `_markdown_config` · `_markdown_crate` · `autoform`


## Instructions

### Adding new demos

1. Check if a similar demo exists to reuse the pattern.
2. Create the demo and add it in `mod.rs`.
3. Add the demo doc in `public/docs/*` following existing patterns.
4. Run `cargo run` from `build_registry/` — auto-generates `public/registry/*` and `__registry__`.

### CHANGELOG

- Update `public/docs/changelog.md` on every user-facing change (new features, components, fixes). Add at the top.
- Skip internal refactors, styling tweaks, or build changes.


