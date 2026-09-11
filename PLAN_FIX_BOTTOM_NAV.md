# PLAN_FIX_BOTTOM_NAV.md

iOS bottom-nav clipping regression: fixed. Reference doc for the root causes
and how to verify the fix still holds.

## Current state (2026-09-11)

No custom `index.html` (dx's default template), no `viewport-fit=cover`, no
native ObjC WKWebView patches. Layout correctness on iOS relies on the
WKWebView's default automatic `contentInsetAdjustmentBehavior`: without
`viewport-fit=cover`, WebKit keeps the CSS layout viewport inside the safe
area, so `position: fixed; bottom: 0` lands above the home indicator for
free. `env(safe-area-inset-*)` CSS rules are still present/pinned in
`tailwind.css` (harmless, resolve to `0px` without `viewport-fit=cover`, not
load-bearing right now).

`app_crates/registry/src/ui/bottom_nav.rs` and the layout wrapper
(`src/routes/app_layout.rs`, `src/components/navigation/app_wrapper.rs`) are
unmodified application code, not part of any hotfix.

## Root causes found (all fixed)

1. **WKWebView disk cache, two separate locations.** An installed sim app has
   two containers linked by bundle id: `Containers/Bundle/Application/<UUID>/DioxusUi.app`
   (binary + assets, overwritten every `dx serve` reinstall) and
   `Containers/Data/Application/<UUID>/Library/...` (WKWebView's disk cache,
   **not** touched by a plain reinstall). Two independent caches live there:
   - `Library/WebKit/com.rust-ui/WebsiteData/...` (LocalStorage, IndexedDB, etc.)
   - `Library/Caches/com.rust-ui/WebKit/...` (NSURLCache HTTP/network disk
     cache: `NetworkCache`, `CacheStorage`, `HSTS`) — the one that actually
     caches HTML/CSS/JS responses.

   A plain Ctrl+C + `dx serve --platform ios` relaunch replaces the `.app`
   bundle but leaves both caches intact, so old HTML/CSS/JS keeps being served
   even though the new build is correct on disk.

   **Fix**: `build.rs` calls `purge_stale_simulator_webkit_cache()` whenever
   `CARGO_CFG_TARGET_OS == "ios"`. It resolves the booted simulator's UDID via
   `xcrun simctl list devices booted -j`, finds the data container whose
   `.com.apple.mobile_container_manager.metadata.plist` contains
   `com.rust-ui`, and `rm -rf`s both `Library/WebKit` and
   `Library/Caches/com.rust-ui` in it. `cargo:rerun-if-changed` points at a
   path that can never exist, so this runs on every build, not just when
   `build.rs` changes. Best-effort (`let-else` returns, no panics): never
   fails the build if no simulator is booted or the app isn't installed yet.

2. **Tailwind content-scan tree-shaking.** `dx serve --platform ios` reruns
   Tailwind's content scan from scratch every pass and tree-shakes
   unreferenced `@utility` rules. An incremental/fast relaunch scan could miss
   the bottom-nav safe-area classes even with unchanged `.rs` source,
   producing CSS that renders everything except the safe-area padding.
   Symptom: brief FOUC (unstyled HTML), then CSS applies, but nav still
   clipped — meaning the loaded CSS itself was missing the utilities, not that
   a stale CSS was cached.

   **Fix**: explicit `@source inline(...)` pins in `tailwind.css` (right after
   the `@source "./src/**/*.rs"` lines) for every literal class string the
   bottom nav uses: `bottom__safe`, `safe__dvh__content`,
   `pb-[env(safe-area-inset-bottom,0px)]`, and the three
   `supports-[-webkit-touch-callout:none]:*` variants. See the `DO NOT REMOVE`
   comment block in `tailwind.css` above those pins for the source-of-truth
   pointer for each class if one gets renamed.

## Abandoned approach: native WKWebView patches

Tried porting the Leptos site's fix (`viewport-fit=cover` +
`__HideKeyboardAccessory.m` / `__DisableContentInsetAdjustment.m` ObjC
swizzles) to this Dioxus app. Reverted: the two fixes above already solve the
clipping, and the native-patch path adds real complexity for no gain here.

Why it doesn't port cleanly if ever revisited: Leptos ships iOS via Tauri +
xcodegen, which produces a real `.xcodeproj` where `__attribute__((constructor))`
`.m` files just work. The Dioxus app is native dioxus-mobile (wry + tao), no
Tauri/xcodeproj — `dx` compiles the app as a staticlib and links the bundle
itself, and **silently drops `cargo:rustc-link-arg` directives** (i.e.
`-force_load`), so a constructor-only `.o` with no Rust-referenced symbol gets
dead-stripped and never runs. The only way to force linkage is an explicit
Rust `unsafe extern "C"` call site — which then needs
`unsafe_code = "deny"` instead of `"forbid"` in `[workspace.lints.rust]`
(`forbid` can't be locally `#[allow]`-ed).

If the keyboard accessory bar (grey iOS prev/next/Done bar above the
keyboard) or an edge-to-edge redesign ever makes this worth revisiting: reuse
the explicit-call-site approach, not `-force_load`, and confirm
`env(safe-area-inset-bottom)` is actually non-zero on device before trusting
it (Safari Web Inspector > Develop > Simulator, or a temporary CSS probe:
`body::after { content: "sab=" env(safe-area-inset-bottom, -1); position: fixed; bottom: 0; }`).

## Dev loop / verification

`./run_ios_dioxus.sh` does the known-good reset: `xcrun simctl uninstall
booted com.rust-ui`, kill any running `dx serve`, then
`dx serve --platform ios`. Use it whenever testing an iOS nav change.

Checklist when verifying a fix:

1. Uninstall + fresh serve, not just Ctrl+C + relaunch (the whole point of the
   `build.rs` purge is that plain relaunch should now also be safe — test
   both).
2. `cargo clippy --bin dioxus-ui` clean, `unsafe_code = "forbid"` intact (no
   `unsafe` anywhere in the crate).
3. Bottom nav: five items visible, labels fully above the home indicator,
   active item highlighted, tapping routes.
4. Scroll a long page to bottom: last row not hidden behind the bar
   (`bottom__safe`).
5. `xcrun simctl io booted screenshot <path>` + Read it to confirm visually
   rather than guessing from logs.

## Reference facts

- Dioxus `=0.7.9` (`router`, `fullstack`), dx CLI `0.7.10` (prints a
  non-fatal version-mismatch warning, harmless).
- wry `0.53.5`, tao `0.34.8`: no `contentInsetAdjustmentBehavior` set, no
  `didMoveToWindow` override — relies on WebKit's default behavior.
- Rust edition 2024: `unsafe extern "C" {}` required (not bare `extern`).
- iOS internal navigation must use `Link`, never raw `<a href>` ("not an http
  url" crash).
- Never run `dx fmt` in this repo (corrupts `sidenav_common.rs`).
- Dioxus app shares the `com.rust-ui` bundle id with the Leptos/Tauri app
  (same App Store Connect record, App ID, distribution profile).
