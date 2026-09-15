# Plan: Dioxus Framework Support

## Context

The `ui` CLI hardcodes Leptos everywhere. Dioxus components exist in `dioxus-ui/src/registry/*.rs`
but `dioxus.rust-ui.com` doesn't serve the registry files yet (only `/docs/{name}.md` which is
prose, not source code). Two tasks needed: generate the registry files for the dioxus site,
then update the CLI to route dioxus projects to `dioxus.rust-ui.com/registry/`.

---

## Task A — Generate dioxus registry files (in `dioxus-ui` project)

**Goal:** make `dioxus.rust-ui.com/registry/tree.md` and
`dioxus.rust-ui.com/registry/styles/default/{name}.md` return 200.

### What exists

| Path | Status |
|------|--------|
| `dioxus-ui/src/registry/*.rs` | ✅ all component source files |
| `dioxus-ui/public/registry/file_names.md` | ✅ list of component names |
| `dioxus-ui/public/registry/tree.md` | ❌ missing |
| `dioxus-ui/public/registry/styles/default/{name}.md` | ❌ missing |
| `rust-ui.com/registry/tree.md` | ✅ reference format |
| `rust-ui.com/registry/styles/default/button.md` | ✅ reference format |

### Steps

**1. `public/registry/styles/default/{name}.md` generation**

For each `src/registry/{name}.rs`, create `public/registry/styles/default/{name}.md`:

```markdown
---
title: "Button"
name: "button"
cargo_dependencies: []
registry_dependencies: []
type: "components:ui"
path: "ui/button.rs"
---

```rust
<content of src/registry/button.rs>
```
```

Add a build script (Rust or shell) at `dioxus-ui/scripts/generate_registry.rs` (or `build.rs`)
that reads `src/registry/*.rs` and writes the `.md` files. Run as part of CI / before deploy.

**2. `public/registry/tree.md` generation**

Mirror the format of `rust-ui.com/registry/tree.md` but for dioxus components.
Read from `public/registry/file_names.md` (already exists) to get component list.
Generate a `tree.md` with dependency tree per component (start simple: no deps).

**3. Wire into build**

Add script call to `dioxus-ui/package.json` scripts:
```json
"registry": "cargo run --manifest-path scripts/Cargo.toml -- generate-registry"
```

Or a simple shell script `scripts/generate_registry.sh` if simpler.

---

## Task B — CLI dioxus support (in `crates/ui-cli`)

### Files to modify

| File | Change |
|------|--------|
| `src/shared/framework.rs` (**new**) | `Framework` enum + `detect_framework()` |
| `src/command_init/workspace_utils.rs` | Remove hard Leptos requirement |
| `src/command_init/config.rs` | Add `framework` to `UiConfig` |
| `src/command_init/crates.rs` | Add `DIOXUS_INIT_CRATES` |
| `src/command_init/_init.rs` | Auto-detect + write framework to config |
| `src/shared/rust_ui_client.rs` | Framework-aware URLs |
| `src/command_add/registry.rs` | Pass framework through |
| `src/command_mcp/tools.rs` | Framework-aware checklist + list/search |

### 1. `src/shared/framework.rs` (new)

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Framework { #[default] Leptos, Dioxus }

pub fn detect_framework(manifest: &Manifest) -> Framework {
    if manifest.dependencies.contains_key("dioxus") {
        Framework::Dioxus
    } else {
        Framework::Leptos
    }
}
```

Add `pub mod framework;` to `src/shared/mod.rs`.

### 2. `workspace_utils.rs` — remove hard Leptos requirement

```rust
// before
fn check_leptos_in_manifest(manifest: &Manifest) -> bool {
    manifest.dependencies.contains_key("leptos")
}

// after
fn check_any_framework_in_manifest(manifest: &Manifest) -> bool {
    manifest.dependencies.contains_key("leptos")
        || manifest.dependencies.contains_key("dioxus")
}
```

Update all call sites + error messages:
`"Leptos dependency not found"` → `"No supported framework (leptos/dioxus) found in Cargo.toml"`

### 3. `config.rs` — add framework to UiConfig

```rust
pub struct UiConfig {
    pub base_color: String,
    pub color_theme: String,
    pub base_path_components: String,
    #[serde(default)]
    pub rtl: bool,
    #[serde(default)]
    pub framework: Framework,
}
```

`Default` impl: call `detect_framework()` on local manifest, fall back to `Leptos`.

### 4. `crates.rs` — DIOXUS_INIT_CRATES

```rust
pub const DIOXUS_INIT_CRATES: [Crate; 2] = [
    Crate::new("dioxus", None, Some(&["web", "router", "fullstack"])),
    Crate::new("tw_merge", None, Some(&["variant"])),
];
```

### 5. `_init.rs` — detect and write framework

After workspace analysis:
1. Call `detect_framework()` on manifest
2. If `Dioxus` → use `DIOXUS_INIT_CRATES`
3. Write `framework = "dioxus"` to `ui_config.toml`

### 6. `rust_ui_client.rs` — framework-aware URLs

```rust
impl RustUIClient {
    fn base_url(framework: &Framework) -> &'static str {
        match framework {
            Framework::Leptos => "https://www.rust-ui.com/registry",
            Framework::Dioxus => "https://dioxus.rust-ui.com/registry",
        }
    }

    fn tree_url(framework: &Framework) -> String {
        format!("{}/tree.md", Self::base_url(framework))
    }

    fn component_url(name: &str, framework: &Framework) -> String {
        format!("{}/styles/default/{name}.md", Self::base_url(framework))
    }

    pub async fn fetch_tree_md_for(framework: &Framework) -> CliResult<String> { ... }
    pub async fn fetch_styles_for(name: &str, framework: &Framework) -> CliResult<String> { ... }
}
```

Keep `fetch_tree_md()` and `fetch_styles_default()` as-is (call with `&Framework::Leptos`)
for backwards compat. Add the new `_for` variants.

### 7. `registry.rs` — pass framework through

`RegistryComponent::fetch_from_registry(name, framework)` calls `fetch_styles_for()`.
`process_add_components()` reads `UiConfig.framework` and passes it down.

### 8. `tools.rs` (MCP) — framework-aware

`list_components` / `search_components` / `view_component` read `ui_config.toml` for framework,
call `fetch_tree_md_for()` / `fetch_styles_for()` accordingly.

`audit_checklist()`:
```rust
fn audit_checklist() -> String {
    let fw = read_framework_from_config().unwrap_or(Framework::Leptos);
    match fw {
        Framework::Leptos => LEPTOS_CHECKLIST,  // current text
        Framework::Dioxus => DIOXUS_CHECKLIST,  // dioxus::prelude::*, no ssr feature flags
    }
}
```

---

## Resulting Workflow (after both tasks done)

```bash
# 1. Generate registry files (Task A, run once per release)
cd dioxus-ui && npm run registry
# → writes public/registry/tree.md
# → writes public/registry/styles/default/*.md
# → deploy → dioxus.rust-ui.com/registry/... now returns 200

# 2. Use CLI in dioxus project (Task B)
cd realesate-ai-manager
ui init
# → detects dioxus in Cargo.toml
# → writes ui_config.toml: framework = "dioxus"

ui add table select dialog
# → reads framework = dioxus
# → fetches from dioxus.rust-ui.com/registry/styles/default/table.md
# → copies Dioxus component source
```

---

## Order of implementation

1. **Task A first** — generate + verify registry files locally, check URLs return 200
2. **Task B** — CLI changes (can be done in parallel but testing needs Task A deployed)

---

## Verification

1. `dioxus.rust-ui.com/registry/tree.md` → 200 with component list
2. `dioxus.rust-ui.com/registry/styles/default/button.md` → 200 with Dioxus Rust code
3. `cd realesate-ai-manager && ui init` → detects dioxus, no error
4. `ui add table` → fetches from dioxus site, writes correct file
5. `cargo build` in app → component compiles
6. MCP `list_components` + `search_components` → lists dioxus components
7. MCP `get_audit_checklist` → returns dioxus checklist
8. Leptos project regression: `ui init` + `ui add button` still work unchanged
9. `cargo test -p ui-cli` → all tests pass + new `detect_framework()` tests
