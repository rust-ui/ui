# PLAN — align `dioxus-ui` workspace deps, lints, profiles, and format config with `leptos-ui`

Status: proposed, not executed.
Goal: make `dioxus-ui` match `leptos-ui` for dependency management, lint config,
formatting config, and build profiles. Component code stays framework-specific.

## Base constraint

The two repos use different frameworks. Keep everything Dioxus-only untouched:

- deps: `dioxus`, `dioxus-html`, `pulldown-cmark`, `syntect`, `html_parser`,
  `html-escape`, `gloo-timers`, `toml`
- `clippy.toml`: `await-holding-invalid-types` (generational_box / dioxus_signals)

Root manifest shape is NOT changed: `dioxus-ui/Cargo.toml` is a combined
`[package]` + `[workspace]` manifest, while `leptos-ui/Cargo.toml` is a virtual
manifest. Moving the root bin crate into a member crate is out of scope.

## Before / after

```
BEFORE                                 AFTER
dioxus-ui/                             dioxus-ui/
  Cargo.toml                            Cargo.toml
    [package] + [workspace]               [package] + [workspace]   (unchanged)
    (no workspace.deps)             +    [workspace.dependencies]   <- single source
    (no lints)                     +    [workspace.lints.clippy]   <- copy from leptos
    (no profiles)                  +    [workspace.lints.rust]
    [dependencies] direct pins     ~    [dependencies] -> .workspace = true
                                  +    [profile.*]                <- copy from leptos
                                  ~    resolver = "2"
  app_crates/*/Cargo.toml         ~    + [lints] workspace = true
    duplicated direct pins         ~    shared deps -> .workspace = true
  (no rustfmt.toml)               +  rustfmt.toml                 <- copy from leptos
  clippy.toml (dioxus-only)       ~  clippy.toml + allow-*-in-tests (merge, not replace)
  registry/blocks/sidenav_routes.rs ~  to_title() via heck (see Step 3, sub-decision)
```

## Shared-dependency divergence table

| dep | leptos-ui | dioxus-ui current | action |
|-----|-----------|-------------------|--------|
| management | `[workspace.dependencies]` + `.workspace = true` | direct pins duplicated per crate | migrate to workspace |
| `strum` | `0.27` | root `0.28`, app_routes `0.28`, registry `0.27` | unify on `0.27` |
| `[workspace.lints]` | ~30 clippy rules + 1 rust | absent | copy from leptos |
| `[profile.*]` | dev / build-override / server-dev (cranelift) / release | absent | copy from leptos |
| `web-sys` | workspace, `default-features = false`, one shared feature list | per crate, root list != registry list, no `default-features=false` | union of features, `default-features = false` |
| `time` | `["serde", "wasm-bindgen"]` | registry `["formatting", "macros"]`, build-dep `["local-offset"]` | workspace = `["serde", "wasm-bindgen"]`; build-dep stays inline `["local-offset"]` |
| `icons` | `["leptos", "leptos_animated"]` | root `["dioxus", "dioxus_animated"]`, sub-crates `["dioxus"]` only | workspace = `["dioxus", "dioxus_animated"]` |
| `tw_merge` | path dep, `["variant", "debug"]` | crates.io `0.1`, `["debug"]` | workspace = `{ version = "0.1", features = ["debug"] }` (no path, no `variant`) |
| `reqwest` | `default-features = false`, `["json", "rustls-tls"]` | `["json"]` (native TLS) | workspace = rustls |
| `heck` | workspace, used in registry (`use_breadcrumb`, `sidenav_routes`) | app_routes only; registry hardcodes | add `heck.workspace` to registry + refactor code |
| `resend-rs` / `rusqlite` / `tokio` / `tower-http` / `validator` / `serde` / `serde_json` / `js-sys` / `wasm-bindgen` | workspace | per crate, same versions | migrate to workspace |
| `resolver` | `"2"` | unset (root `[package]` edition 2024 => resolver 3) | pin `"2"` |

## Step 1 — `dioxus-ui/Cargo.toml` root

### 1a. `[workspace]`
- Add `resolver = "2"`.

### 1b. Add `[workspace.dependencies]`

Target values (aligned with `leptos-ui/Cargo.toml`):

```toml
[workspace.dependencies]
registry    = { path = "app_crates/registry" }
app_config  = { path = "app_crates/app_config" }
app_domain  = { path = "app_crates/app_domain" }
app_routes  = { path = "app_crates/app_routes" }

icons     = { version = "0.18", features = ["dioxus", "dioxus_animated"] }
tw_merge  = { version = "0.1", features = ["debug"] }
heck      = "0.5"
strum     = { version = "0.27", features = ["derive"] }
serde     = { version = "1", features = ["derive"] }
serde_json = "1.0"
time      = { version = "0.3", features = ["serde", "wasm-bindgen"] }
reqwest   = { version = "0.12", default-features = false, features = ["json", "rustls-tls"] }
resend-rs = { version = "0.19", default-features = false, features = ["rustls-tls"] }
rusqlite  = { version = "0.32", features = ["bundled"] }
validator = { version = "0.20", features = ["derive"] }
tracing   = "0.1"
tower-http = { version = "0.6", features = ["fs"] }
tokio     = { version = "1", features = ["rt-multi-thread"] }
js-sys    = "0.3"
wasm-bindgen = "0.2"
wasm-bindgen-futures = "0.4"
web-sys   = { version = "0.3", default-features = false, features = [
    # union: leptos list + missing dioxus features
    "Blob", "BlobPropertyBag", "Clipboard", "console", "CssStyleDeclaration",
    "Document", "DomRect", "DomTokenList", "DragEvent", "Element", "Event",
    "EventTarget", "Headers", "History", "HtmlAnchorElement", "HtmlElement",
    "HtmlInputElement", "InputEvent", "KeyboardEvent", "Location", "MediaQueryList",
    "MouseEvent", "MutationObserver", "MutationObserverInit", "MutationRecord",
    "Navigator", "Node", "NodeList", "Request", "RequestInit", "RequestMode",
    "Response", "ScrollToOptions", "Storage", "Url", "Window",
] }
```

Stay inline (dioxus-only): `dioxus`, `dioxus-html`, `pulldown-cmark`, `syntect`,
`html_parser`, `html-escape`, `gloo-timers`, `toml`.

### 1c. Add `[workspace.lints.clippy]` + `[workspace.lints.rust]`
Exact copy of the `leptos-ui/Cargo.toml` block:
- deny: `unwrap_used`, `expect_used`, `panic`, `todo`, `indexing_slicing`,
  `await_holding_lock`, `dbg_macro`, `undocumented_unsafe_blocks`, `enum_glob_use`,
  `missing_safety_doc`, `get_unwrap`, `infinite_loop`, `large_stack_arrays`
- warn: `lossy_float_literal`, `cast_precision_loss`, `redundant_clone`,
  `inefficient_to_string`, `cloned_instead_of_copied`, `manual_let_else`,
  `trivially_copy_pass_by_ref`, `print_stdout`, `print_stderr`, `explicit_iter_loop`,
  `from_over_into`, `fallible_impl_from`, `clone_on_ref_ptr`, `needless_borrow`
- rust: `irrefutable_let_patterns = "deny"`

### 1d. Add `[profile.*]`
Copy from leptos: `[profile.dev]`, `[profile.dev.build-override]`,
`[profile.server-dev]` (`codegen-backend = "cranelift"`),
`[profile.dev.package."*"]`, `[profile.release]`.

Prerequisite: `codegen-backend = "cranelift"` needs a nightly toolchain with
`-Zcodegen-backend`. Check `dioxus-ui/rust-toolchain.toml` before including
`[profile.server-dev]`; otherwise drop it or add it in a later pass.

### 1e. Convert root `[dependencies]` + `[build-dependencies]`
- Every shared dep -> `<dep>.workspace = true` (keep `optional = true` where present).
- `[build-dependencies] time` stays inline (`features = ["local-offset"]`, != workspace).
- Root `[features]` block (`default`, `server`, `web`) is unchanged.

## Step 2 — member crates

`app_crates/registry`, `app_crates/app_config`, `app_crates/app_domain`, `app_crates/app_routes`.

For each:
1. Add at the top:
   ```toml
   [lints]
   workspace = true
   ```
2. Replace every shared dep with `<dep>.workspace = true`.
3. Keep `dioxus`, `dioxus-html`, `pulldown-cmark`, `gloo-timers`, etc. inline.

Specifics:
- `registry`: add `heck.workspace = true`.
- `app_routes`: `strum` + `heck` -> `.workspace = true` (fixes `strum 0.28` -> `0.27`).
- `app_domain`: `icons` -> `.workspace = true` (gains `dioxus_animated`).
- `app_config`: `serde` / `serde_json` -> `.workspace = true`.

Risks to verify:
- `strum 0.28 -> 0.27`: `EnumIter` / `Display` / `AsRefStr` derives are stable
  between the two, low risk but recompile `app_routes` + `registry` to confirm.
- `web-sys` union: no feature removed, so no breakage expected.

## Step 3 — heck refactor in `registry`

File: `dioxus-ui/app_crates/registry/src/blocks/sidenav_routes.rs`

Goal: `to_title()` via `heck::to_title_case()` instead of hardcoded `match` arms.

```
BEFORE                                  AFTER
use std::fmt;                           use heck::ToTitleCase;
                                        use std::fmt;

fn to_title(&self) -> &'static str      fn to_title(&self) -> String
  match { 11 arms "Sidenav 01".. }        self.as_str().to_title_case()
```

Gotcha: `"sidenav01".to_title_case()` yields `"Sidenav01"`, not `"Sidenav 01"`
(heck does not split between letter and digit). Space is lost.

### Sub-decision (Step 3)

To truly match leptos, `leptos-ui` declares the enum with
`#[strum(serialize_all = "kebab-case")]` + `strum::AsRefStr`, so `sidenav-01`
then `.to_title_case()` -> `"Sidenav 01"`.

Option A (real alignment, larger diff):
- add `#[derive(strum::AsRefStr, strum::EnumIter, strum::Display)]`
  + `#[strum(serialize_all = "kebab-case")]` on `SidenavRoutes`
- `as_str()` -> `self.as_ref()`, `from_path()` -> `strum::IntoEnumIterator`
- remove the manual `impl fmt::Display`
- `to_title()` -> `self.as_ref().to_title_case()`

Option B (minimal): keep `to_title()` hardcoded for `SidenavRoutes` only,
apply heck only where a kebab source already exists.

Neighboring enums:
- `ComponentsRoutes`: `"alert-dialog".to_title_case()` -> `"Alert Dialog"` OK -> heck applicable
- `DocsRoutes`, `HooksRoutes`: `"use-copy-clipboard"` -> `"Use Copy Clipboard"` OK -> heck applicable
- decide per enum, consistent with the option picked for `SidenavRoutes`

## Step 4 — config files

- Create `dioxus-ui/rustfmt.toml` = copy of `leptos-ui/rustfmt.toml`:
  ```toml
  edition = "2024"
  max_width = 120
  imports_granularity = "Module"
  group_imports = "StdExternalCrate"
  struct_field_align_threshold = 0
  use_small_heuristics = "Max"
  ```
- `dioxus-ui/clippy.toml`: MERGE, add the leptos lines, keep the dioxus block:
  ```toml
  allow-unwrap-in-tests = true
  allow-expect-in-tests = true
  allow-panic-in-tests = true
  allow-indexing-slicing-in-tests = true

  # dioxus-only, kept
  await-holding-invalid-types = [ ... ]
  ```

## Step 5 — verification

In `dioxus-ui/`:
1. `cargo metadata --format-version 1 >/dev/null` (resolves graph, catches manifest errors)
2. `cargo build --no-default-features --features web`
3. `cargo build --no-default-features --features server`
4. `cargo clippy --all-targets --all-features` (new `deny` rules may surface
   pre-existing violations -> handle case by case, possibly outside this plan)
5. `cargo fmt --check` (no `view!` macro here, so no leptosfmt step)

## Recommended execution order

1. Step 4 (rustfmt.toml + clippy.toml) — no risk
2. Step 1b + 1e (workspace.dependencies + root) — recompile
3. Step 2 (members) — recompile after each crate
4. Step 1c (lints) — recompile clippy, triage violations
5. Step 1d (profiles) — after confirming nightly/cranelift toolchain
6. Step 3 (heck refactor) — depends on sub-decision A or B
7. Step 5 (full verification)

## Open decisions

- Step 3: option A (strum + kebab) or option B (keep hardcode for SidenavRoutes)
- Step 1d: include `[profile.server-dev]` cranelift now or defer
- Step 1c: what to do with pre-existing `deny` violations revealed by the new lints
