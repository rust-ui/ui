# Plan: Match Leptos Architecture As Literally As Possible

Goal: make `dioxus-ui` match the Leptos app architecture and file responsibilities as closely as possible.

Constraint:

- this is not "rough parity"
- this is not "same behavior, different structure"
- Dioxus should copy the Leptos architecture, module split, naming, and ownership boundaries as literally as possible
- a difference is acceptable only when Dioxus creates a real technical constraint

This file intentionally keeps only the remaining mismatches.

Rule for maintaining this file:

- remove completed items immediately as work lands
- do not keep "done" cleanup tasks in the plan
- if a mismatch is resolved, delete it from this file instead of marking it done

Decision already made:

- keep `workflows` in Dioxus
- but keep it clearly isolated from the Leptos-parity path instead of letting it distort the Leptos-matching structure

## Target Rule

When there is a design choice:

- prefer the same file split as Leptos
- prefer the same module naming as Leptos
- prefer the same ownership boundaries as Leptos
- do not keep a Dioxus-specific structure just because it already works

## Current Remaining Gaps

### 1. Tighten `src/__registry__/` to the Leptos shape

Leptos exposes:

- `all_blocks`
- `demos_sidenav`
- `my_command_bar_constants`
- `static_md_registry`

Dioxus still has extra modules in `src/__registry__/`:

- `all_workflows.rs`
- `sidenav_get_started.rs`
- `sidenav_hooks.rs`
- `source_map.rs`

Required work:

- [ ] reduce `dioxus-ui/src/__registry__/mod.rs` toward the exact Leptos role
- [ ] rename or collapse helper modules when Leptos keeps that responsibility elsewhere
- [ ] isolate Dioxus-only registry extras from the Leptos-parity path

### 2. Finish shrinking `src/registry/` to the literal Leptos role

Leptos `app/src/registry/` contains only:

- `mod.rs`
- `md_docs/mod.rs`
- `md_docs/docs_installation_cli_tree_view.rs`

Dioxus still keeps one extra module:

- `src/registry/types.rs`

Required work:

- [ ] decide whether `src/registry/types.rs` should move into `src/__registry__/` or another Leptos-matching owner
- [ ] make `dioxus-ui/src/registry/` as close as possible to the Leptos tree

### 3. Match `app_crates/registry/src/` ownership more literally

Leptos `app_crates/registry/src/` contains:

- `blocks/`
- `charts/`
- `constants/`
- `demos/`
- `hooks/`
- `ui/`
- `utils/`

Dioxus currently differs:

- missing `constants/`
- missing `utils/`
- has Dioxus-only `workflows/`

Required work:

- [ ] add `dioxus-ui/app_crates/registry/src/constants/` if that ownership should mirror Leptos
- [ ] add `dioxus-ui/app_crates/registry/src/utils/` if that ownership should mirror Leptos
- [ ] keep `app_crates/registry/src/workflows/`, but isolate it as a Dioxus-only surface that does not redefine the Leptos-parity target

### 4. Match `app_crates/app_domain/src/` ownership more literally

Leptos `app_crates/app_domain/src/` contains:

- `constants/`
- `icons/`
- `markdown_config/`
- `themes/`
- `utils/`

Dioxus currently differs:

- missing `markdown_config/`
- missing `utils/`

Required work:

- [ ] add `dioxus-ui/app_crates/app_domain/src/markdown_config/` or move equivalent logic there
- [ ] add `dioxus-ui/app_crates/app_domain/src/utils/` or move equivalent logic there
- [ ] audit whether logic currently under `src/markdown/` belongs in the Leptos-style crate boundary

### 5. Match top-level `src/` layout more literally

Leptos top-level `src/` includes:

- `app.rs`
- `lib.rs`
- `shell.rs`
- `domain/tests/`

Dioxus currently differs:

- has `main.rs` instead of the Leptos-style split
- has no `shell.rs`
- uses `domain/test/` instead of `domain/tests/`
- still exposes top-level `src/markdown/`

Required work:

- [ ] port the entry layout toward the closest Dioxus equivalent of Leptos `app.rs` / `lib.rs` / `shell.rs`
- [ ] rename `src/domain/test/` to `src/domain/tests/` if Dioxus does not block it
- [ ] decide whether `src/markdown/` should be folded into Leptos-style ownership boundaries instead of staying top-level
- [ ] add `dioxus-ui/app_crates/app_components/` if Leptos responsibilities require that literal split

### 6. Restore Leptos page surfaces and route intent more literally

Leptos currently exposes these page responsibilities:

- home page
- download page
- docs layout under `domain/docs/routing/`
- all-demos overview pages for components and hooks
- shared demo page flow for components and hooks
- icons page under `domain/icons/`
- views route shell under `domain/views/`
- bug-report page

Current Dioxus mismatches:

- `/download` is linked from the home page but no Dioxus route serves it
- there is no all-demos overview equivalent for components/hooks
- docs page responsibilities live in top-level `src/routes/` instead of `src/domain/docs/routing/`
- component/hook/docs pages are split differently from Leptos shared route ownership
- icons page is routed from top-level `src/routes/` instead of a Leptos-like `domain/icons/`
- Dioxus has `view/block/:id` but not the same `view/:name` shell shape as Leptos
- bug-report surface is absent

Required work:

- [ ] add a Dioxus download page and route equivalent to Leptos `page_download.rs`
- [ ] add all-demos overview page equivalents for components and hooks
- [ ] move docs-routing ownership closer to Leptos `src/domain/docs/routing/`
- [ ] decide whether component/hook/docs page rendering should be re-converged toward a Leptos-style shared route owner
- [ ] move icons page ownership closer to Leptos `src/domain/icons/`
- [ ] move views routing closer to Leptos `domain/views/views_layout.rs` + `view_router.rs`
- [ ] decide whether the bug-report page should be ported literally or explicitly treated as intentionally omitted

### 7. Support-page and route parity still missing

Confirmed remaining gaps versus the Leptos app shape:

- `/download` exists as a Dioxus route constant and home-page CTA target, but no Dioxus route currently serves it
- Leptos has `page_download.rs`
- Dioxus still lacks the matching page/route

Other remaining support-page gaps:

- [ ] `public/docs/workflow.md` if we want the same support-page set as Leptos
- [ ] all-demos overview page equivalent
- [ ] download page and route equivalent

### 8. Restore domain ownership that still differs from Leptos

Leptos `src/domain/mod.rs` currently exposes:

- `blocks`
- `bug_report`
- `charts`
- `create`
- `docs`
- `icons`
- `markdown_ui`
- `tests`
- `themes`
- `views`

Dioxus currently differs:

- missing `bug_report`
- missing `docs` as a domain owner
- missing `icons` as a domain owner
- missing `themes` as a domain owner
- uses `test` instead of `tests`
- adds `workflows`

Required work:

- [ ] add back the missing Leptos-style domain owners where Dioxus should mirror them literally
- [ ] rename `test` to `tests` if no Dioxus constraint blocks it
- [ ] keep `workflows`, but treat it as an explicit Dioxus extension rather than part of the Leptos-parity target

### 9. Keep the public docs IA literally Leptos-like

Decision already made:

- cleanup items already removed should stay out of this plan
- keep `mask` as an intentional retained Dioxus surface
- keep `image` as an intentional retained Dioxus surface
- keep `toolbar` as an intentional Dioxus-only exception because we want to keep it

Required work:

- [ ] keep `components/toolbar.md`, but mark it as an explicit Dioxus-only exception outside the Leptos-parity target

### 10. Restore app-components ownership more literally

Leptos keeps shared app-level UI pieces in `app_crates/app_components/src/`.

Dioxus still keeps analogous responsibilities in top-level `src/components/`, including things like:

- footer layout
- newsletter/signup surface
- doc header
- table of contents
- sidenav
- app footer

Required work:

- [ ] add `dioxus-ui/app_crates/app_components/`
- [ ] decide which current `src/components/*` files should move into the Leptos-style `app_components` crate
- [ ] leave only genuinely app-shell-specific Dioxus wiring in top-level `src/components/` if needed

### 11. Remaining docs cleanup

Required work:

- [ ] review `public/docs/**` frontmatter completeness for `image` / `image_dark`
- [ ] enrich `public/docs/figma.md` only if we want the same content depth as Leptos

### 12. Test parity after structure parity

Do not expand tests before the remaining architecture gaps above are settled.

Required work:

- [ ] add Dioxus hook e2e coverage
- [ ] port Leptos component specs where DOM and behavior truly match
- [ ] validate markdown wrappers, tags, and install blocks through e2e coverage

## Confirmed Non-Gaps

These should not stay in this file anymore as active plan items:

- central markdown registry exists in `src/__registry__/static_md_registry.rs`
- shared markdown wrappers already exist
- `MarkdownType` already exists in Dioxus
- `get_static_registry_entry(...)` already exists in Dioxus
- old per-component docs registry files are already gone
- docs/component/hook rendering already goes through the central markdown path
- `src/registry/md_docs/docs_installation_cli_tree_view.rs` already exists

## Recommended Execution Order

1. Tighten `src/__registry__/` and `src/registry/` until they match the Leptos responsibility split as literally as possible.
2. Restore Leptos page surfaces and docs-route ownership, including `/download` and all-demos pages.
3. Close the crate-boundary mismatches in `app_crates/registry`, `app_crates/app_domain`, and `app_crates/app_components`.
4. Move the top-level `src/` and `src/domain/*` layout closer to the Leptos split where Dioxus allows it.
5. Keep `workflows`, but isolate it clearly from the Leptos-parity path.
6. Separate Dioxus-only docs from the Leptos-parity docs surface.
7. Finish docs frontmatter cleanup.
8. Expand e2e parity only after the structure is stable.
