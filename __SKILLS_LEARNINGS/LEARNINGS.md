# Learnings

One line per learning. Format: `YYYY-MM-DD [domain] avoid X, do Y, because Z`.

New captures land in **Inbox** (not yet reviewed). Move a line up to
**Confirmed** once validated as worth keeping. Delete from Inbox what you do
not want.

## Confirmed

- 2026-09-10 [rust] avoid hand-written `match self { Enum::Variant => "Variant", ... }` for enum-to-string, do derive `strum::IntoStaticStr` / `strum::AsRefStr` (+ `#[strum(serialize_all=...)]` or per-variant `#[strum(to_string=...)]`) and call `.into()` / `.as_ref()`, because single source of truth, no variant/string drift, less boilerplate. Keep a manual match only when one enum needs several different string projections (e.g. `dot_color()` + `text_color()` + `label()`) or computed values; then still use `Self::Variant` arms (clippy::use_self). Precedent: src/markdown/highlight_language.rs, use_workflow.rs EdgeStyle.

## Inbox

- 2026-09-10 [dioxus] avoid `dx fmt` in this workspace, do use `cargo fmt` only (rustfmt leaves rsx! bodies alone), because `dx fmt` reformats the entire workspace (300+ files, incl. leptos-ui submodule) and corrupts `app_crates/registry/src/blocks/sidenav_common.rs` (unexpected closing delimiter at :260) since it cannot round-trip that file's rsx.
- 2026-09-10 [dioxus] avoid raw `a { href: "/internal/path" }` for in-app navigation, do use `Link { to: Route::Variant { .. } }` (or `Link { to: format!(..) }`), because on `dx serve --platform ios` the webview loads from a non-http custom scheme so a real `<a>` nav triggers wry `Failed to open URL: not an http url`; `Link` intercepts the click in Rust and routes in-memory.
- 2026-09-11 [dioxus/ios] avoid calling `web_sys::window()`/`js_sys` (or any wasm-bindgen extern) unconditionally on a code path reachable from the iOS build, do gate it behind `#[cfg(target_arch = "wasm32")]` (whole `use_effect`/fn body, mirroring `use_pagination.rs`), because `dx serve --platform ios` compiles the crate natively for `aarch64-apple-ios(-sim)`, not wasm32, and wasm-bindgen's non-wasm32 stub panics unconditionally the instant the FFI call is made, even inside `if let Some(...)`/`.and_then(...)` guards on the `Option` it returns — the guard never gets a chance to run. Confirmed crash: `registry::hooks::use_table_of_contents`'s `use_effect` called `web_sys::window()` on mount, `EXC_CRASH`/`SIGABRT` navigating to any page using it. Audited and gated every other unconditional `web_sys::window()`/FFI call in `app_crates/registry/src/{hooks,demos}` the same way (a stored `web_sys::Element` *type* in a signal, never actually constructed on native, is safe and does not need gating — only the FFI call itself panics).
- 2026-09-11 [dioxus/ios] avoid `__attribute__((constructor))` on a native ObjC `.m` patch file AND an explicit Rust `unsafe extern "C"` call to the same symbol, do pick exactly one invocation path (here: drop the constructor attribute, keep only the Rust call), because `dx` (unlike xcodegen/Tauri) links the crate as a Rust staticlib and prunes an archive member's `.o` unless something references its symbols — so the explicit Rust call is required just for linker inclusion — but keeping the constructor attribute too makes the patch run twice; for `WKWebView` method swizzling specifically, the second run captures the already-swizzled implementation as "original", causing infinite recursion the next time the swizzled method fires (`EXC_BAD_ACCESS`/stack overflow). Also: `static` (internal-linkage) C functions get their whole `.o` pruned by the linker if unreferenced from Rust — give them external linkage instead.

