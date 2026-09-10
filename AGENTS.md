# AGENTS.md

## Crates (`crates/`)

`icons` · `tw_merge` · `tw_merge_variants` · `ui-cli` · `_starters` · `_markdown_config`


## Instructions

### Adding new demos

Demos belong in their owning website repository:

- Leptos demos: `leptos-ui/`.
- Dioxus demos: `dioxus-ui/`.

Follow that repository's local instructions. Parent repository keeps shared generated registry artifacts and tooling only.

### CHANGELOG

- Write every user-facing change to `CHANGELOG_DEV.md` first (top entry, dated `## YYYY-MM-DD`).
- Do **not** edit `public/docs/changelog.md` directly. The user reviews `CHANGELOG_DEV.md` and promotes entries to `public/docs/changelog.md` when ready.
- Skip internal refactors, styling tweaks, and build changes.
