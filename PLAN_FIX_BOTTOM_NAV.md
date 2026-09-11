# PLAN_FIX_BOTTOM_NAV.md

Exhaustive debug dossier for the iOS mobile bottom navigation regression.

Status: **fixed, Option B applied and verified on device (2026-09-11)**. See
section 13. Known-good revision was `d2f4eb7`. The bottom nav labels were
clipped by the iOS home indicator on `main`, and rendered correctly on
`d2f4eb7`.

The user's stated preference: revert to the `d2f4eb7` behaviour rather than keep
the pile of iOS hotfixes added on top of it.

## 1. TL;DR

`d2f4eb7` works because it does **not** set `viewport-fit=cover`. Without that
meta, the iOS WKWebView keeps the CSS layout viewport **inside the safe area**,
so a `position: fixed; bottom: 0` bar sits just above the home indicator for
free, and the fixed `4rem` (`--bottom__nav__height`) nav is fully visible.

Commit `dffbbe5` (this session) added a root `index.html` with
`viewport-fit=cover`, matching the Leptos site. That makes the layout viewport
go **edge to edge, under the home indicator**. `fixed bottom-0` is now behind
the indicator. The only thing that compensates for that is
`pb-[env(safe-area-inset-bottom,0px)]` on the `BottomNav` element, and for
`env(safe-area-inset-bottom)` to resolve to a non-zero value in WKWebView you
need `scrollView.contentInsetAdjustmentBehavior = .never`, which is what the
ported ObjC patch `__DisableContentInsetAdjustment.m` does.

That patch is unreliable in the Dioxus build (see section 6), so `env()` stays
`0`, the compensation is `0`, and the bar renders behind the home indicator.
Net result: we replaced a zero-dependency static layout with a fragile chain
(`viewport-fit=cover` + a runtime ObjC swizzle + `env()` + launch timing), and
it regressed.

Recommended fix: **Option B in section 8** (full revert of the nav + WKWebView
patch changes, keep the unrelated iOS TestFlight plumbing).

## 2. Component and layout inventory

### 2.1 `--bottom__nav__height` and the `bottom__safe` / `safe__dvh__content` utilities

`tailwind.css`:

```css
:root, :host {
  --bottom__nav__height: 4rem;
}

@utility bottom__safe {
  padding-bottom: var(--bottom__nav__height);   /* 4rem below sm */
  @media (min-width: 640px) { padding-bottom: 0; }  /* 0 at sm+ */
}

@utility safe__dvh__content {           /* defined, kept for leptos parity, */
  height: 100vh;                        /* not referenced in any .rs so it is */
  height: 100dvh;                       /* pinned via @source inline(...) to   */
  padding-bottom: var(--bottom__nav__height);   /* survive the dx serve rebuild */
  @media (min-width: 640px) { padding-bottom: 0; }
}
```

`tailwind.css` also has `@source inline("safe__dvh__content");` near the top so
the `dx serve --platform ios` Tailwind pass (which rebuilds
`assets/tailwind.css` from scratch) does not tree-shake the rule and churn the
tracked artifact. That pin came from commit `2528c21` and should stay
regardless of which fix option is chosen.

### 2.2 `registry::ui::bottom_nav` primitives

`app_crates/registry/src/ui/bottom_nav.rs`. **Unchanged since `d2f4eb7`.**
Class strings are byte-for-byte identical to the Leptos
`leptos-ui/app_crates/registry/src/ui/bottom_nav.rs`:

```
BottomNav        nav   z-50 mx-auto w-full max-w-lg border-t border-border bg-background
                       pb-[env(safe-area-inset-bottom,0px)]
BottomNavGrid    div   grid grid-flow-col auto-cols-fr h-[var(--bottom__nav__height)] font-medium
BottomNavLabel   span  text-sm text-muted-foreground group-hover:text-primary
                       group-aria-[current=page]:text-primary
BottomNavButton  button inline-flex flex-col justify-center items-center px-5 group
                       [&_svg]:mb-2 ... active:scale-[0.98]
                       touch-manipulation [-webkit-tap-highlight-color:transparent] select-none
                       [-webkit-touch-callout:none]
                       supports-[-webkit-touch-callout:none]:justify-end
                       supports-[-webkit-touch-callout:none]:pb-0
                       supports-[-webkit-touch-callout:none]:translate-y-1
```

Note the `supports-[-webkit-touch-callout:none]:*` variants. On any WebKit
(iOS Safari, WKWebView) `-webkit-touch-callout` is a supported property, so on
device the buttons get `justify-end` + `translate-y-1` (nudged down 0.25rem) and
`pb-0`. This is intentional Leptos behaviour and depends on the parent `nav`
carrying the safe-area bottom padding to leave room. If that padding is `0`, the
`translate-y-1` pushes the labels straight into the clip zone. This is why the
symptom is "labels clipped" specifically, not "whole bar hidden".

The generated CSS rule is present in `assets/tailwind.css`:

```css
.pb-\[env\(safe-area-inset-bottom\,0px\)\] { padding-bottom: env(safe-area-inset-bottom,0px); }
```

so this is not a Tailwind tree-shaking problem. The rule exists; `env()`
evaluates to `0`.

### 2.3 `AppBottomNav`

`src/components/app_bottom_nav.rs`. **Unchanged since `d2f4eb7`.** Renders
`BottomNav { class: "fixed inset-x-0 bottom-0 sm:hidden" }` with five
`BottomNavButton`s (Home, Components, Hooks, Icons, Charts). Active item tracked
with `use_route::<Route>()` + typed `matches!`. Leptos equivalent:
`leptos-ui/app_crates/app_components/src/app_bottom_nav.rs`, same structure,
same `fixed right-0 bottom-0 left-0 sm:hidden`.

### 2.4 Layout wrapper: this is one of the two axes that changed

**`d2f4eb7` (`src/routes/app_layout.rs`):**

```rust
rsx! {
    // Empty ontouchstart lets iOS WKWebView fire CSS :active
    div { class: "flex flex-col h-full", ontouchstart: move |_| {},
        ScrollToTop {}
        main {
            id: "data-scroll-target",
            class: "overflow-y-auto flex-1 overflow-x-clip bottom__safe",   // <- 4rem pad HERE
            Outlet::<Route> {}
        }
        AppBottomNav {}
        CommandSearchDocsDialog {}
    }
}
```

`AppWrapper` existed but was dead code. `bottom__safe` lived on the scrolling
`<main>`.

**`main` (after `1c3703f`):**

```rust
rsx! {
    ScrollToTop {}
    AppWrapper {                                   // div.flex.flex-col.h-full.bottom__safe + ontouchstart
        main {
            id: "data-scroll-target",
            class: "overflow-y-auto flex-1 overflow-x-clip",   // <- pad removed from here
            Outlet::<Route> {}
        }
    }
    AppBottomNav {}
    CommandSearchDocsDialog {}
}
```

`bottom__safe` moved onto the non-scrolling outer flex column via `AppWrapper`;
`AppBottomNav` became a sibling of `AppWrapper`. **This now matches the Leptos
`app/src/app.rs` + `app/src/components/navigation/app_wrapper.rs` structure**
(Leptos `AppWrapper` is `<div class="flex flex-col h-full bottom__safe"
on:touchstart=|_| {}>`).

Effect analysis: `AppBottomNav` is `position: fixed`, so it is out of flow and
its on-screen position does not depend on where `bottom__safe` sits. Moving the
`4rem` pad from `<main>` to the outer `h-full` column only changes whether the
last scroll row is hidden behind the bar, not whether the bar itself is
clipped. **So axis 1 (the restructure) is not the clip regression.** It is a
correctness/parity improvement and is safe to keep or revert. It was originally
changed to fix a separate complaint ("bar overlapped content / scrollbar ran
under it").

## 3. The regression trigger: `viewport-fit=cover`

`d2f4eb7` has no root `index.html`. `dx` uses its default template, whose
viewport meta is:

```html
<meta name="viewport" content="width=device-width, initial-scale=1">
```

Commit `dffbbe5` added `index.html` at the repo root (dx picks it up as the
base template) with:

```html
<meta name="viewport"
      content="width=device-width, initial-scale=1.0, minimum-scale=1.0, maximum-scale=1.0, user-scalable=no, viewport-fit=cover" />
```

copied from `leptos-ui/app/src/shell.rs` line 18-21.

### Why this matters on iOS

WKWebView safe-area behaviour:

- **No `viewport-fit=cover`**: WebKit constrains the CSS layout/visual viewport
  to the safe area. The bottom of the viewport is the top of the home indicator.
  `position: fixed; bottom: 0` lands above the indicator automatically.
  `env(safe-area-inset-*)` all resolve to `0` (there is nothing unsafe inside
  the viewport to inset against).
- **With `viewport-fit=cover`**: the layout viewport is the full screen,
  extending under the home indicator, the notch, and the status bar.
  `position: fixed; bottom: 0` is now at the physical screen bottom, behind the
  home indicator. The page is expected to pull safe UI back in using
  `env(safe-area-inset-*)`.
- `env(safe-area-inset-*)` only becomes non-zero **and stays exposed to CSS**
  when `WKWebView.scrollView.contentInsetAdjustmentBehavior == .never`. With the
  default `.automatic`, WebKit itself consumes the inset (shrinking the layout
  viewport and moving scroll content), and `env()` reads back `0` in CSS. This
  is the exact reason the Leptos site ships `__DisableContentInsetAdjustment.m`.

So `viewport-fit=cover` is only safe to add if `.never` is reliably set before
first paint. On `main` it is not (section 6), so:

```
env(safe-area-inset-bottom)  == 0
pb-[env(safe-area-inset-bottom,0px)] == 0
nav sits at physical screen bottom
+ BottomNavButton translate-y-1 nudges labels down
=> labels clipped by the home indicator   [matches every screenshot]
```

## 4. Timeline of this session's commits (`d2f4eb7..main`)

```
16e96f5  fix(ios): allow the single WKWebView FFI site under unsafe_code   [nav-patch]
52ba9eb  fix(ios): add App Store Info.plist keys missing from dx bundle    [ship, keep]
1319517  fix(ios): call the WKWebView patches explicitly from main()       [nav-patch]
ecde64f  fix(build): avoid clippy expect_used in the iOS webview-fix script[nav-patch]
8c89e87  fix(ios): port the Leptos WKWebView ObjC patches                  [nav-patch]
8b6137f  docs(changelog): note the viewport-fit=cover bottom-nav fix       [nav-patch, docs]
dffbbe5  fix(ios): add viewport-fit=cover so the bottom nav clears ...     [nav-patch, TRIGGER]
c6e375b  chore(ios): gitignore the distribution provisioning profile       [ship, keep]
0a796ed  chore(ios): add 1024 App Store icon source                        [ship, keep]
1c3703f  fix(mobile-nav): move bottom__safe to the non-scrolling column    [layout axis 1]
787e794  build(ios): use shared com.rust-ui bundle id for TestFlight       [ship, keep]
acde191  chore: gitignore .env.apple (Apple signing credentials)          [ship, keep]
db65882  build(ios): wire dx-native TestFlight bundling for the Dioxus app [ship, keep]
2528c21  fix(css): pin safe__dvh__content against dx serve tree-shaking    [css, keep]
```

Buckets:

- **nav-patch** (`dffbbe5`, `8b6137f`, `8c89e87`, `ecde64f`, `1319517`,
  `16e96f5`): the regression + its follow-on hotfixes. Candidates for revert.
- **layout axis 1** (`1c3703f`): parity restructure. Not the regression. Keep
  or revert independently. Recommend keep (matches Leptos), but the `d2f4eb7`
  form is also fine.
- **ship** (`db65882`, `acde191`, `787e794`, `0a796ed`, `c6e375b`, `52ba9eb`):
  TestFlight bundling, bundle id, icon, entitlements, Info.plist keys,
  gitignores. Unrelated to the nav. **Keep all.**
- **css** (`2528c21`): the `@source inline` pin. **Keep.**

## 5. Files touched by the nav-patch bucket

| File | `d2f4eb7` state | `main` state | Revert action |
| --- | --- | --- | --- |
| `index.html` | does not exist | 18-line template with `viewport-fit=cover` | delete |
| `__HideKeyboardAccessory.m` | does not exist | ObjC, swizzles `-[WKContentView inputAccessoryView]` to `nil` | delete |
| `__DisableContentInsetAdjustment.m` | does not exist | ObjC, swizzles `-[WKWebView didMoveToWindow]` to set `.never` | delete |
| `build.rs` | 6-line date stamp only | + `ios_webview_fixes()` compiling the two `.m` with `cc`, `-force_load`, framework links | restore to `d2f4eb7` |
| `src/main.rs` | plain `fn main()` | + `#[cfg(target_os="ios")] unsafe extern "C"` block + `unsafe {}` call + `#[allow(unsafe_code)]` | restore to `d2f4eb7` |
| `Cargo.toml` `[build-dependencies]` | `time` only | + `cc = "1"` | drop `cc` |
| `Cargo.toml` `[workspace.lints.rust]` | `unsafe_code = "forbid"` | `unsafe_code = "deny"` + 3-line comment | restore `"forbid"` |
| `src/routes/app_layout.rs` | pad on `<main>`, `AppWrapper` unused | pad on `AppWrapper`, restructured | see Option A vs B |
| `src/components/navigation/app_wrapper.rs` | `bottom__safe`, no `ontouchstart` | `bottom__safe` + `ontouchstart` | see Option A vs B |
| `CHANGELOG_DEV.md` | | 3 bullets under `## 2026-09-10 / ### Fixes` | drop the nav bullets, keep ship bullets |

## 6. Why the ObjC patch is unreliable under Dioxus (and reliable under Leptos)

Leptos ships iOS via Tauri + xcodegen. The `justfile` copies
`__HideKeyboardAccessory.m` and `__DisableContentInsetAdjustment.m` into
`src-tauri/gen/apple/Sources/<AppName>/`, then `xcodegen generate` picks them up
as compile sources in the real Xcode app target. Each file is an
`__attribute__((constructor))`, so it runs at `dlopen`/launch, before the first
webview is created. `.never` is set before first paint. Deterministic.

The Dioxus app is **native dioxus-mobile** (wry + tao WKWebView), no Tauri, no
`.xcodeproj`. `dx` compiles the app crate as a **staticlib** and links the final
`.app` bundle itself. In that link step:

1. `cargo:rustc-link-arg` directives from `build.rs` (used for
   `-force_load`) are dropped by `dx`. Without `-force_load`, the object files
   from the `cc`-compiled `.m` are not referenced by any Rust symbol, so the
   linker dead-strips them and the `constructor` never ends up in the binary.
2. To work around 1, `8c89e87` + `1319517` rewrote each `.m` to also export a
   plain C entrypoint (`rust_ui_hide_keyboard_accessory`,
   `rust_ui_disable_content_inset_adjustment`), declared in `src/main.rs` via
   `unsafe extern "C"` and called explicitly at the top of `fn main()`. Both
   entrypoints are `dispatch_once`-guarded so the retained `constructor` (if it
   is ever pulled in) does not double-apply. The `didMoveToWindow` swizzle in
   particular is **not idempotent** without the guard: a second
   `method_setImplementation` makes `original_didMoveToWindow` point at the
   swizzled IMP and the next call infinitely recurses.
3. The explicit call in `main()` forces `unsafe`, which trips
   `unsafe_code = "forbid"` in `[workspace.lints.rust]` (`forbid` cannot be
   locally `#[allow]`-ed). `16e96f5` relaxed it to `deny` so the one FFI site
   can opt in.

Even with all of that, the last observed run (build finished 22:44:22) logged:

```
HideKeyboardAccessory: Successfully hidden
DisableContentInsetAdjustment: Successfully configured
```

but the nav was **still clipped** in the 22:44:43 screenshot. Candidate
explanations, in rough priority order:

1. **Wrong / stale app.** The booted simulator had **five** Rust UI style apps
   installed simultaneously: `com.rust-ui` ("Rust/UI", current), plus
   `com.example.DioxusUi` (older Dioxus build with the default bundle id),
   `com.example.DioxusFullstackAuth`, `app.rust-ui.starterios`,
   `com.rustui.app`. Opening the wrong icon shows an old build. All five were
   uninstalled during this session:
   `xcrun simctl uninstall booted com.rust-ui com.example.DioxusUi ...`.
   This alone could explain several of the "still broken" reports and means we
   may have been chasing a phantom.
2. **WKWebView HTTP cache.** In `dx serve` fullstack mode the iOS webview loads
   `http://127.0.0.1:8080/` (SSR HTML from the dx server, which does inject the
   `viewport-fit=cover` meta, verified with `curl`). WKWebView caches HTML and
   CSS aggressively (see MEMORY.md "Browser Caching Issues with Static Assets").
   A cached pre-`index.html` document has no `viewport-fit=cover`, so `env()`
   would still be `0` even with the swizzle working. Fixed only by a clean app
   reinstall, not by `r` / rebuild.
3. **Swizzle timing.** "Successfully configured" only means the swizzle was
   installed, not that `-[WKWebView didMoveToWindow]` fired afterwards for the
   wry `WryWebView` instance. wry does `ns_view.addSubview(&webview)` during
   webview construction, which happens after `main()` runs the swizzle, so the
   ordering should be fine, but this was never verified with a log line inside
   `swizzled_didMoveToWindow`.
4. **`.never` set but `env()` still 0.** Possible if the WKWebView is not
   actually laid out under the home indicator. Checked against wry 0.53.5 and
   tao 0.34.8 source: tao's `TaoUIViewController` view is created at full
   `UIScreen bounds`, and wry inits the webview with `ns_view.frame()` +
   `autoresizingMask = 31`, so the webview is edge to edge. This explanation is
   unlikely but not disproven.

The point: the Dioxus path needs `viewport-fit=cover` + a runtime swizzle +
correct cache state + correct app, all at once. The `d2f4eb7` path needs none
of it.

## 7. Known-good reference: `d2f4eb7` served DOM

```
#main
  (ScrollToTop: no DOM)
  div.flex.flex-col.h-full            [ontouchstart]
    main#data-scroll-target.overflow-y-auto.flex-1.overflow-x-clip.bottom__safe
      <route outlet>
  nav[data-name=BottomNav].z-50.mx-auto.w-full.max-w-lg.border-t.border-border
     .bg-background.pb-[env(safe-area-inset-bottom,0px)].fixed.inset-x-0.bottom-0.sm:hidden
    div[data-name=BottomNavGrid].grid.grid-flow-col.auto-cols-fr.h-[var(--bottom__nav__height)]
      button[data-name=BottomNavButton] x5
  (CommandSearchDocsDialog)

viewport meta: width=device-width, initial-scale=1        (dx default, NO viewport-fit=cover)
env(safe-area-inset-bottom): 0
effective nav bottom padding: 0
nav position: fixed bottom-0 == top of home indicator (WebKit keeps viewport in safe area)
result: nav fully visible, labels not clipped
bottom__safe on <main>: 4rem, last scroll row clears the bar
```

## 8. Fix options

### Option A: minimal (delete `index.html` only)

Remove the regression trigger, leave the rest.

```
git rm index.html
# rebuild, clean-reinstall on the simulator
```

Pros: one file, smallest diff, keeps the Leptos-parity layout restructure
(`1c3703f`).
Cons: the dead `.m` files, the `cc` build-dependency, the
`unsafe_code = "deny"` relaxation and the `src/main.rs` FFI block all stay in
the tree as unused weight. They still compile on iOS and add a
build-dependency + an `unsafe` site for no benefit. Also loses the
`__HideKeyboardAccessory.m` fix for the grey keyboard accessory bar (see
section 9).

### Option B: full revert of the nav-patch bucket (recommended, matches the user's "no hotfixes" preference)

Restore every nav-patch file to its `d2f4eb7` state, keep the ship + css
buckets.

Concrete steps (single commit on `main`):

```
# 1. delete the files that did not exist at d2f4eb7
git rm index.html __HideKeyboardAccessory.m __DisableContentInsetAdjustment.m

# 2. restore the modified files to their d2f4eb7 content
git checkout d2f4eb7 -- build.rs src/main.rs

# 3. Cargo.toml, by hand:
#    - [build-dependencies]: remove the `cc = "1"` line (+ its comment)
#    - [workspace.lints.rust]: unsafe_code = "deny"  ->  unsafe_code = "forbid"
#      (drop the 3-line explanatory comment)

# 4. layout: pick ONE
#    B1 (keep leptos parity, recommended): leave app_layout.rs / app_wrapper.rs as they are on main
#    B2 (full d2f4eb7): git checkout d2f4eb7 -- src/routes/app_layout.rs src/components/navigation/app_wrapper.rs

# 5. CHANGELOG_DEV.md: remove the three nav bullets under 2026-09-10 / Fixes
#    ("Mobile bottom nav overlapped content" may stay if B1),
#    ("Bottom nav labels clipped by the iOS home indicator") delete,
#    ("iOS keyboard accessory bar covered the bottom nav") delete.
#    Add one bullet: reverted the viewport-fit=cover + ObjC WKWebView patch
#    approach, back to the no-cover static safe-area layout.

# 6. verify
cargo clippy --bin dioxus-ui        # must be clean with unsafe_code = "forbid" again
# clean-reinstall on the simulator, NOT just `r`:
xcrun simctl uninstall booted com.rust-ui
dx serve --platform ios
```

Files that must end up **byte-identical to `d2f4eb7`** after Option B (B2):
`build.rs`, `src/main.rs`, `src/routes/app_layout.rs`,
`src/components/navigation/app_wrapper.rs`, and `index.html` /
`__*.m` gone. `Cargo.toml` differs from `d2f4eb7` only by the intentional ship
+ css keeps.

Files that must be **unchanged from `main`** (do not revert): `Dioxus.toml`,
`ios/Info.plist`, `ios/Entitlements.plist`, `ios/AppIcon-1024.png`,
`.gitignore`, `tailwind.css`, `rust_ui_internals/deploy_ios_testflight_dioxus.sh`.

## 9. Tradeoff: the keyboard accessory bar

`__HideKeyboardAccessory.m` removes the grey iOS "prev / next / Done" input
accessory bar that sits above the keyboard. The Leptos site ships this fix.
Reverting it (Option A or B) brings that bar back whenever a text input is
focused in the app. Today the only reachable text input in the mobile app is
the command/search docs dialog, so the impact is small.

If that bar turns out to matter, re-add **only** `__HideKeyboardAccessory.m`
later, in isolation, with its own before/after test. Do not re-add
`__DisableContentInsetAdjustment.m` or `viewport-fit=cover` unless the nav is
being deliberately redesigned to be edge-to-edge, and only with a device test
that confirms `env(safe-area-inset-bottom)` is non-zero (section 10).

The `API error: <_UIKBCompatInputView ...> returned 0 width` log line is a
benign side effect of returning `nil` from `inputAccessoryView`. It is not an
error to fix and it disappears once the swizzle is reverted.

## 10. How to actually measure `env(safe-area-inset-bottom)` on device

Do not guess. Add a temporary probe and read it in the dx serve console or via
Safari Web Inspector (Develop menu > Simulator > the page).

CSS probe (drop into `tailwind.css` `@layer base` temporarily):

```css
body::after {
  content: "sab=" env(safe-area-inset-bottom, -1) " sat=" env(safe-area-inset-top, -1);
  position: fixed; left: 0; bottom: 0; z-index: 99999;
  background: #f00; color: #fff; font: 12px monospace; padding: 2px 4px;
}
```

or JS in the browser console:

```js
getComputedStyle(document.documentElement).getPropertyValue('--x'); // if you map it
// or read a probe element:
const p = document.createElement('div');
p.style.cssText = 'padding-bottom:env(safe-area-inset-bottom)';
document.body.appendChild(p);
getComputedStyle(p).paddingBottom;   // "0px" => cover/never not effective; "34px" => working
```

Interpretation:

- `0px` with `viewport-fit=cover` present in the served `<head>` => the
  `.never` swizzle is not effective (dead-stripped, wrong timing, cache, or
  wrong app).
- `0px` with no `viewport-fit=cover` in the served `<head>` => expected, and
  the `d2f4eb7` layout is designed for exactly this.
- `~34px` => `env()` is live; if the nav is still clipped the bug is in the CSS
  application, not the native layer.

## 11. Verification checklist for whichever fix lands

1. `xcrun simctl uninstall booted com.rust-ui` (and any other stale Rust UI
   bundle ids) before testing. Confirm only one Rust UI icon on the home
   screen. Plain Ctrl+C + relaunch of `dx serve --platform ios` is **not**
   enough: it only rewrites the `Bundle/Application/<UUID>/DioxusUi.app`
   folder, it does not touch the separate `Data/Application/<UUID>/Library/WebKit/com.rust-ui/`
   container, so WKWebView keeps serving cached HTML/CSS/JS from disk. Either
   `simctl uninstall` (nukes both containers) or manually
   `rm -rf .../Data/Application/<UUID>/Library/WebKit` between runs. See
   section 13 for the full explanation.
2. `cargo clippy --bin dioxus-ui` clean. If Option B, confirm
   `unsafe_code = "forbid"` is back and still compiles (no `unsafe` anywhere in
   the crate).
3. `dx serve --platform web` still emits exactly one `name="viewport"` meta.
4. `dx serve --platform ios`, fresh install. Bottom nav: five items visible,
   labels fully above the home indicator, active item highlighted, press-scale
   works (`:active`), tapping routes.
5. Scroll a long page to the bottom: last content row is not hidden behind the
   bar (`bottom__safe`).
6. `assets/tailwind.css` has no uncommitted churn after the serve (the
   `@source inline` pin holds).
7. TestFlight path unaffected: `rust_ui_internals/deploy_ios_testflight_dioxus.sh`
   still references `ios/Info.plist`, `ios/Entitlements.plist`,
   `ios/Rust_UI_Distribution.mobileprovision`, `Dioxus.toml [bundle].icon`.

## 12. Reference facts

- Dioxus `=0.7.9` (`router`, `fullstack`), dx CLI `0.7.10`. dx prints a
  non-fatal "dx and dioxus versions are incompatible!" ERROR line; bundling and
  serving continue.
- wry `0.53.5`, tao `0.34.8`. iOS webview: `initWithFrame: ns_view.frame()`,
  `autoresizingMask = 31`, `scrollView.setBounces(false)`. No
  `contentInsetAdjustmentBehavior` set by wry. No `didMoveToWindow` override in
  `WryWebView`.
- Rust edition 2024: `unsafe extern "C" {}` required (not bare `extern`).
- `[workspace.lints.rust] unsafe_code`: `forbid` cannot be locally
  `#[allow]`-ed; `deny` can.
- Leptos iOS app (`leptos-ui/src-tauri/tauri.conf.json`): identifier
  `com.rust-ui`, window url `https://rust-ui.com` (the iOS app loads the
  production site, not a local dev server), `developmentTeam` `4JGPL5Q5CD`.
  Ships `__*.m` via `justfile` + `xcodegen`.
- The Dioxus app shares the `com.rust-ui` bundle id (commit `787e794`) so the
  App Store Connect record, App ID and "Rust UI Distribution" profile are
  reused.
- `dx serve --platform ios` runs a from-scratch Tailwind pass that rewrites
  `assets/tailwind.css`; unreferenced `@utility` rules are tree-shaken unless
  pinned with `@source inline(...)`.
- iOS internal navigation must use `Link`, never raw `<a href>` (iOS "not an
  http url" bug, commit `7ac104c`).
- Never run `dx fmt` in this repo (corrupts `sidenav_common.rs`).

## 13. Resolution (2026-09-11)

Applied **Option B1** (section 8): deleted `index.html`,
`__HideKeyboardAccessory.m`, `__DisableContentInsetAdjustment.m`; restored
`build.rs` to the plain date-stamp-only version; dropped the `cc = "1"`
build-dependency from `Cargo.toml`. Kept the layout restructure
(`src/routes/app_layout.rs`, `src/components/navigation/app_wrapper.rs`) as-is
on `main` (parity with `leptos-ui`, confirmed not the regression source per
section 2.4).

By the time this landed, `main` had already moved past the `16e96f5` tip this
doc was written against (history had been rewritten again; see
`git log --oneline -- src/main.rs`), and `src/main.rs` no longer had the
`unsafe extern "C"` FFI block or the `unsafe_code = "deny"` relaxation
described in section 5 / section 8 step 3 — both were already back to their
`d2f4eb7` shape. So the only files actually touched for the revert were
`index.html`, the two `__*.m` files, `build.rs`, and `Cargo.toml`.

**False-negative loop while verifying**: after the revert, a fresh screenshot
still showed the labels clipped, looking identical to the pre-revert bug. Root
cause of *that*: no `dx serve` process was running and the sim app had not
been uninstalled/reinstalled, so the screenshot was WKWebView serving a
cached pre-revert document (still carrying `viewport-fit=cover`) from an old
session. This is exactly failure mode 2 in section 6 ("WKWebView HTTP
cache") plus the stale-app problem in section 6 point 1, just recurring after
the fix instead of before it. Confirms section 11 item 1 and item 4 are not
optional steps: `r` / plain reload is not sufficient to validate an iOS nav
fix, only `xcrun simctl uninstall booted com.rust-ui` + a fresh
`dx serve --platform ios` is.

Once that clean reinstall was actually done, the bug did not reproduce: the
`d2f4eb7` no-`viewport-fit=cover` layout renders the bottom nav fully above
the home indicator, as predicted in section 7.

**Outstanding, not done as part of this fix**: the keyboard accessory bar
(grey prev/next/Done bar) is back, since `__HideKeyboardAccessory.m` was
removed along with the other patch. Per section 9, low impact today (only the
docs-search dialog has a focusable text input on mobile). Revisit in isolation
if it becomes a real complaint; do not resurrect `viewport-fit=cover` or
`__DisableContentInsetAdjustment.m` to do it, and do not rely on
`cargo:rustc-link-arg` / `-force_load` under `dx` — section 6 point 1 in this
doc shows `dx`'s own bundling link step drops those directives, so any future
native-patch approach needs either an explicit call site (like `main()` did
before this revert) or a non-native (JS) fix instead.

**Root cause of "plain kill+relaunch doesn't apply the fix, but `simctl
uninstall` does" (2026-09-11, post-fix)**: an installed sim app has two
separate on-disk containers, linked only by bundle id:

- `Containers/Bundle/Application/<UUID-A>/DioxusUi.app` — the app binary +
  bundled assets (`index.html`, wasm, css). `dx serve` overwrites this on
  every reinstall.
- `Containers/Data/Application/<UUID-B>/Library/WebKit/com.rust-ui/` — the
  WKWebView disk cache (`WebsiteData/Default/...`, LocalStorage, IndexedDB).
  **Not touched** by a plain reinstall of the bundle.

Ctrl+C + `dx serve --platform ios` again only replaces the `.app` bundle.
WebKit's on-disk HTTP/document cache in the Data container survives untouched
and keeps serving the old `index.html`/CSS/JS, so the fix looks like it
"didn't take" even though the new files are sitting right there on disk.
`xcrun simctl uninstall booted com.rust-ui` deletes **both** containers, so
the next install gets a genuinely empty WebKit cache and the new assets load.
Confirmed by inspecting the booted simulator's containers directly:
`Info.plist`'s `CFBundleIdentifier` is `com.rust-ui` inside `DioxusUi.app`,
and a populated `Library/WebKit/com.rust-ui/WebsiteData/` sits in the sibling
Data container. This is the same WKWebView disk-cache behavior as section 6's
"WKWebView HTTP cache" failure mode, just triggered by the app's own
reinstall instead of a dev-server restart. Cheaper alternative to a full
uninstall: `rm -rf` just that `Library/WebKit` folder between runs.

## 14. Automatic cache purge in `build.rs` (2026-09-11)

Manually `rm -rf`-ing `Library/WebKit` (or a full `simctl uninstall`) between
every `dx serve --platform ios` run does not scale to other contributors, so
the purge was moved into `build.rs`: when `CARGO_CFG_TARGET_OS == "ios"`,
`purge_stale_simulator_webkit_cache()` reads the booted simulator's UDID via
`xcrun simctl list devices booted -j`, walks
`~/Library/Developer/CoreSimulator/Devices/<udid>/data/Containers/Data/Application/*`,
matches the data container whose
`.com.apple.mobile_container_manager.metadata.plist` contains `com.rust-ui`,
and purges it. `println!("cargo:rerun-if-changed={out_dir}/__force_rerun_never_exists")`
(a path that can never exist) forces cargo to treat the build script as
always-dirty so this runs on every build, not just when `build.rs` changes.
Best-effort throughout (`let-else` returns, no panics) so it never fails the
build if no simulator is booted or the app isn't installed yet.

**Second cache location found (still reproduced after the first purge
shipped)**: section 13 above only identified and purged
`Data/Application/<UUID>/Library/WebKit/com.rust-ui/` (the WebsiteData store:
LocalStorage, IndexedDB, SearchHistory, ResourceLoadStatistics). There is a
**second, independent** on-disk WKWebView cache in the same data container
that this missed:

- `Data/Application/<UUID>/Library/Caches/com.rust-ui/WebKit/` — the actual
  NSURLCache-backed HTTP/network disk cache (`NetworkCache`, `CacheStorage`,
  `AlternativeServices`, `HSTS`). This is where WKWebView actually caches the
  HTML/CSS/JS network responses; `Library/WebKit/.../WebsiteData` does not
  cover it.

Confirmed by inspecting a live data container after a rebuild: the
`Library/Caches/com.rust-ui/WebKit/NetworkCache` mtime predated the latest
`build.rs` run, i.e. it survived the WebsiteData-only purge untouched and kept
serving stale HTML/CSS/JS even though the shipped `.app` binary and CSS were
already correct (verified by `strings`-grepping the installed binary for
`viewport-fit`/`name="viewport"` and by parsing the bundled Tailwind CSS for
the expected `bottom__nav`/`safe-area-inset-bottom` utilities: both clean).

Fix: `purge_stale_simulator_webkit_cache()` now removes both
`Library/WebKit` and `Library/Caches/com.rust-ui` for the matched data
container. Anyone still seeing stale nav behavior after pulling this fix needs
one manual `xcrun simctl uninstall booted com.rust-ui` to clear an
already-poisoned container from before the fix landed; every `dx serve
--platform ios` after that stays clean automatically since both paths are
purged pre-build.

**Third factor found: Tailwind content-scan tree-shaking, not cache.** After
the double-cache-purge fix above landed and was verified working (fresh
`simctl uninstall` + `dx serve` gave a clean bottom nav, confirmed live via
`xcrun simctl io booted screenshot` and by checking that both
`Library/WebKit` and `Library/Caches/com.rust-ui` mtimes matched the latest
`build.rs` rerun), a **plain Ctrl+C + `dx serve --platform ios` relaunch**
(no `simctl uninstall`) still reproduced clipping intermittently. Observed
symptom: briefly unstyled HTML (raw blue underlined links = FOUC), then CSS
applies, but the bottom nav is still broken after CSS loads. That pattern
means the CSS that loaded was missing the bottom-nav utilities entirely, not
that an old cached CSS was served.

Root cause: `tailwind.css` had no `@source inline(...)` pin (checked,
none existed anywhere in the repo) for `bottom__safe`, `safe__dvh__content`,
or the `pb-[env(safe-area-inset-bottom,0px)]` / `supports-[-webkit-touch-callout:none]:*`
literal classes in `app_crates/registry/src/ui/bottom_nav.rs`. Per section 12,
`dx serve --platform ios` reruns Tailwind's content scan from scratch on
every pass and tree-shakes unreferenced `@utility` rules; an incremental/fast
relaunch scan can miss them even though the `.rs` source is unchanged,
producing CSS that renders everything except the safe-area padding.

Fix: added explicit `@source inline(...)` pins right after the existing
`@source "./src/**/*.rs"` / `@source "./app_crates/**/*.rs"` lines in
`tailwind.css`, one per literal class string used by the bottom nav
(`bottom__safe`, `safe__dvh__content`, `pb-[env(safe-area-inset-bottom,0px)]`,
and the three `supports-[-webkit-touch-callout:none]:*` variants). This makes
Tailwind always emit these utilities regardless of scan completeness, closing
the last remaining source of intermittent (non-cache) clipping on plain
relaunches.

## 15. Native WKWebView patch attempt, reverted; `run_ios_dioxus.sh` added (2026-09-11)

After section 14 shipped, tried porting the Leptos site's native fix
(`viewport-fit=cover` in `index.html` + two ObjC WKWebView swizzle patches,
`__HideKeyboardAccessory.m` / `__DisableContentInsetAdjustment.m`) over to
this Dioxus app, using an explicit Rust `unsafe extern "C"` call site from
`main()` instead of the original `-force_load` + `__attribute__((constructor))`
approach (the latter is silently dropped by `dx`'s bundling link step, so the
patches never actually ran under `dx`). Hit a lint error
(`unsafe extern` block denied by the workspace's `unsafe_code = "forbid"`)
and while fixing that, decided this was adding real complexity
(custom `index.html`, `.m` files, `cc` build-dependency, a lint policy change)
for a fix that section 13/14 already cover. **Reverted** all of it: no
custom `index.html` (dx's generated default is used again, so no
`viewport-fit=cover`), no `.m` files, no `cc` build-dep, `unsafe_code` back
to `"forbid"`. Current layout correctness on iOS relies on the WKWebView's
default automatic `contentInsetAdjustmentBehavior`, not CSS
`env(safe-area-inset-*)` (those rules are still present/pinned per section 14
but resolve to `0px` without `viewport-fit=cover` — harmless, not load-bearing
right now).

Live-app inspection after the revert (fresh `simctl uninstall` +
`dx serve --platform ios`) confirmed: no `viewport-fit` anywhere in the
installed bundle, compiled CSS still has the safe-area utilities pinned
(6x `env(safe-area-inset-top)`, 3x `env(safe-area-inset-bottom)`), bottom nav
and header both render clean.

Added `run_ios_dioxus.sh` (repo root) to make the known-good repro/dev loop a
single command instead of three manual steps: `xcrun simctl uninstall booted
com.rust-ui`, kill any running `dx serve`, then `dx serve --platform ios`.
This does NOT replace the section 13/14 automatic purge in `build.rs` (that
still needs to hold up on a plain Ctrl+C + relaunch with no uninstall) — it's
a manual escape hatch for when a hard reset is wanted, and the fast way to
get back to a known-clean state while testing whether the automatic purge
alone is sufficient on warm relaunches.
