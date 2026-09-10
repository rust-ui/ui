# PLAN_BOTTOM_NAV.md

App-level mobile bottom navigation for the `dioxus-ui` site (repo root app),
shown only below the `sm` breakpoint. Ported from the Leptos app, which already
runs this in production.

## Goal

```
 phone (< 640px)                        sm+ (>= 640px)
 +-----------------------------+        +-----------------------------------+
 |  HeaderHome / HeaderDocs    |        |  Navbar (top)                     |
 |                             |        |                                  |
 |                             |        |                                  |
 |     <main> scroll area      |        |        <main> scroll area         |
 |     (pb = 4rem so last      |        |        (no bottom padding)        |
 |      row clears the bar)    |        |                                  |
 |                             |        |                                  |
 +-----------------------------+        +-----------------------------------+
 | [House][Blks][Cmps][Srch][Ch]|  <-- fixed, sm:hidden          (no bar)
 +-----------------------------+
   Home  Comp  Hook  Icon  Chart
```

Five entries, identical to Leptos:

| Label      | Icon (`icons` crate) | Route                          | active when                                        |
|------------|----------------------|--------------------------------|---------------------------------------------------|
| Home       | `House`              | `Route::Home {}`               | `Route::Home {}`                                   |
| Components  | `Blocks`             | `Route::DocsComponentsIndexPage {}` | `DocsComponentsIndexPage` or `ComponentPage { .. }` |
| Hooks      | `Compass`            | `Route::DocsHooksIndexPage {}`  | `DocsHooksIndexPage` or `HookPage { .. }`          |
| Icons      | `Search`             | `Route::PageIcons {}`          | `PageIcons {}`                                     |
| Charts     | `ChartSpline`        | `Route::AreaChartPage {}`      | any `*ChartPage {}`                                |

## What Leptos has (reference)

- `leptos-ui/app_crates/app_components/src/app_bottom_nav.rs` — `AppBottomNav`
  component. `NavPage` enum (`Home/Components/Hooks/Icons/Charts`) with
  `strum::{Display, EnumIter}`; iterates variants, renders a `BottomNavButton`
  per page, `on:click` calls `use_navigate()`, `attr:aria-current` set from a
  `use_location()` pathname `starts_with` check.
- `leptos-ui/app_crates/registry/src/ui/bottom_nav.rs` — primitives via `clx!`:
  `BottomNav` (`nav`), `BottomNavGrid` (`div`), `BottomNavLabel` (`span`),
  `BottomNavButton` (`button`).
- `leptos-ui/app/src/app.rs:149` — `<AppBottomNav />` rendered once, as the last
  child inside `<Router>`, sibling to `<AppWrapper>`.
- `leptos-ui/app/src/components/.../app_wrapper.rs` — root div carries
  `class="... bottom__safe"` and an empty `ontouchstart` handler.
- `leptos-ui/style/tailwind.css`:
  - `:root { --bottom__nav__height: 4rem; }`
  - `@theme inline { --spacing-bottom__nav__height: var(--bottom__nav__height); }`
  - `@utility bottom__safe { padding-bottom: var(--bottom__nav__height); @media (min-width:640px){ padding-bottom:0 } }`
  - `@utility safe__dvh__content { ... same padding-bottom ... }`

## iOS / tauri hacks: keep all of them

The Leptos `BottomNav` primitives carry three WebKit-specific hacks. `dx serve
--platform ios` renders in **WKWebView**, the same engine as Safari and the
Tauri webview, so every one of these still applies. They were already ported
verbatim into the Dioxus registry primitive
(`app_crates/registry/src/ui/bottom_nav.rs`) and need **no change**:

1. `pb-[env(safe-area-inset-bottom,0px)]` on `BottomNav` — clears the home
   indicator.
2. `supports-[-webkit-touch-callout:none]:{justify-end,pb-0,translate-y-1}` on
   `BottomNavButton` — nudges icons toward the home indicator on iOS Safari
   (comment in the file: `SHORTFIX 🚑 iOS Safari`).
3. `touch-manipulation [-webkit-tap-highlight-color:transparent] select-none
   [-webkit-touch-callout:none]` on `BottomNavButton` — kills the tap flash and
   long-press callout.

Separate hack, **not** bottom-nav related, do not touch here: the `sr-only ->
hidden` swaps elsewhere in the tree (`// sr-only breaks with tauri`). Out of
scope.

### tauri vs `dx serve ios`: what actually simplifies

The Leptos app shipped inside a **Tauri** shell (custom asset protocol, IPC
bridge, no dev server on device). The Dioxus app uses `dx serve --platform ios`,
which is a plain **WKWebView pointed at a dev server** (or the bundled webview in
a release build). So:

| Leptos/Tauri concern | Needed for `dx serve ios`? |
|----------------------|----------------------------|
| `env(safe-area-inset-bottom)` padding | yes, same notch/home-indicator hardware |
| `supports-[-webkit-touch-callout:none]:` icon nudge | yes, still iOS Safari/WebKit layout |
| `-webkit-tap-highlight-color`, `-webkit-touch-callout`, `select-none`, `touch-manipulation` | yes, WebKit touch UX |
| empty `ontouchstart` to unlock `:active` | yes, WebKit-level quirk, not Tauri |
| `sr-only -> hidden` swap (`// breaks with tauri`) | **no**, that was a Tauri asset-scheme / hydration quirk, and it is not in the bottom-nav path anyway |
| Tauri `invoke()` / asset-protocol URL rewriting | **no**, none in the bottom nav |
| routing via `use_navigate()` string paths | replace with typed `Route`, see below |

Net: the CSS/touch hacks stay (already ported into the registry primitive,
zero work). The only real simplification versus the Leptos version is on the
**Rust** side: no string-path `use_navigate`, no `use_location` pathname
`starts_with` parsing. Use `use_route::<Route>()` + `matches!` on typed
variants and `navigator.push(Route::X {})`. That removes the whole class of
"path string does not match" bugs.

One genuinely WebKit (not Tauri) bit we still want: an **empty `ontouchstart`
handler** on the app shell root div. iOS only fires CSS `:active` (used by
`active:scale-[0.98]` on the button) while a touch listener is attached. Leptos
put it on `AppWrapper`; the Dioxus starter puts it on its `AppWrapper` too. The
main Dioxus app's `src/routes/app_layout.rs` root div does **not** have it yet,
so we add it.

## What Dioxus already has

- `app_crates/registry/src/ui/bottom_nav.rs` — `BottomNav`, `BottomNavGrid`,
  `BottomNavLabel`, `BottomNavButton` (`#[component]` fns, `tw_merge!`). Button
  props: `class`, `onclick: EventHandler<MouseEvent>`, `aria_current:
  Option<String>`, `children`. Exported via `registry::ui::bottom_nav`.
- `app_crates/registry/src/demos/demo_bottom_nav.rs` — local-state demo, proves
  the primitives render.
- `tailwind.css` (root) already has **all** the CSS from the Leptos side:
  `--bottom__nav__height: 4rem` (line 23), the `@theme inline` spacing alias
  (line 156), `@utility bottom__safe` (line 294), `@utility safe__dvh__content`
  (line 302). **No CSS work needed.**
- `crates/_starters/start-dioxus-fullstack/src/components/layout/app_bottom_nav.rs`
  — a starter's app-level nav wired to `use_navigator()` + `navigator.push(Route::..)`;
  useful shape reference (its `BottomNavButton` copy takes `active: bool`, ours
  takes `aria_current`).

## What is missing

1. An `AppBottomNav` component in the **main app** (`src/`), wired to the real
   `Route` enum.
2. Registering it in `src/components/mod.rs`.
3. Mounting it once in the app shell (`src/routes/app_layout.rs`) + bottom
   padding on the scroll container + the `ontouchstart` shim.

No new dependency: `strum` is already used in the codebase, `icons` exports
`House / Blocks / Compass / Search / ChartSpline` for Dioxus (verified in
`leptos-ui/crates/icons/src/dioxus/compatibility.rs`).

## Active-state approach

Follow the existing idiom in `src/components/navigation/nav_desktop.rs`:
`let route = use_route::<Route>();` (subscribes the component, re-renders on
navigation) then `matches!(route, Route::X {} | Route::Y { .. })`. Type-safe,
no string parsing, no `use_location`.

## Implementation

### 1. New file: `src/components/app_bottom_nav.rs`

```rust
use dioxus::prelude::*;
use icons::{Blocks, ChartSpline, Compass, House, Search};
use registry::ui::bottom_nav::{BottomNav, BottomNavButton, BottomNavGrid, BottomNavLabel};

use crate::Route;

#[derive(Clone, Copy, PartialEq, Eq, strum::IntoStaticStr)]
enum NavPage {
    Home,
    Components,
    Hooks,
    Icons,
    Charts,
}

const PAGES: &[NavPage] =
    &[NavPage::Home, NavPage::Components, NavPage::Hooks, NavPage::Icons, NavPage::Charts];

impl NavPage {
    fn label(self) -> &'static str {
        self.into()
    }

    fn target(self) -> Route {
        match self {
            Self::Home => Route::Home {},
            Self::Components => Route::DocsComponentsIndexPage {},
            Self::Hooks => Route::DocsHooksIndexPage {},
            Self::Icons => Route::PageIcons {},
            Self::Charts => Route::AreaChartPage {},
        }
    }

    fn icon(self) -> Element {
        match self {
            Self::Home => rsx! { House { class: "size-5" } },
            Self::Components => rsx! { Blocks { class: "size-5" } },
            Self::Hooks => rsx! { Compass { class: "size-5" } },
            Self::Icons => rsx! { Search { class: "size-5" } },
            Self::Charts => rsx! { ChartSpline { class: "size-5" } },
        }
    }

    fn is_active(self, route: &Route) -> bool {
        match self {
            Self::Home => matches!(route, Route::Home {}),
            Self::Components => {
                matches!(route, Route::DocsComponentsIndexPage {} | Route::ComponentPage { .. })
            }
            Self::Hooks => matches!(route, Route::DocsHooksIndexPage {} | Route::HookPage { .. }),
            Self::Icons => matches!(route, Route::PageIcons {}),
            Self::Charts => matches!(
                route,
                Route::AreaChartPage {}
                    | Route::BarChartPage {}
                    | Route::LineChartPage {}
                    | Route::PieChartPage {}
                    | Route::RadarChartPage {}
                    | Route::RadialChartPage {}
            ),
        }
    }
}

#[component]
pub fn AppBottomNav() -> Element {
    let route = use_route::<Route>();
    let navigator = use_navigator();

    rsx! {
        BottomNav { class: "fixed inset-x-0 bottom-0 sm:hidden",
            BottomNavGrid {
                for page in PAGES {
                    {
                        let page = *page;
                        let active = page.is_active(&route);
                        rsx! {
                            BottomNavButton {
                                aria_current: if active { "page" } else { "" },
                                onclick: move |_| {
                                    navigator.push(page.target());
                                },
                                {page.icon()}
                                BottomNavLabel { {page.label()} }
                            }
                        }
                    }
                }
            }
        }
    }
}
```

Notes:
- `strum::IntoStaticStr` gives `label()` for free; variant names are the visible
  labels (matches `demo_bottom_nav.rs`).
- `navigator.push(Route)` is the same call `nav_desktop.rs` context uses for
  `Link`; here we need the click handler because `BottomNavButton` is a
  `<button>`, not an `<a>`. This is the exact Leptos design (button + programmatic
  navigate), and it keeps us clear of the raw-`<a>` iOS bug fixed in `7ac104c`.
- `use_route::<Route>()` re-runs this component on every route change so the
  `aria-current` highlight tracks navigation.

### 2. `src/components/mod.rs`

Add the module (keep alphabetical):

```diff
 pub mod app_footer;
+pub mod app_bottom_nav;
 pub mod command_search_docs;
```

(Alphabetical would put `app_bottom_nav` before `app_footer`; match whatever the
file already does. Current file is alpha-sorted, so:)

```diff
+pub mod app_bottom_nav;
 pub mod app_footer;
```

### 3. `src/routes/app_layout.rs`

```diff
 use dioxus::prelude::*;

 use crate::Route;
+use crate::components::app_bottom_nav::AppBottomNav;
 use crate::components::command_search_docs::CommandSearchDocsDialog;
 use crate::utils::page_transition::ScrollToTop;

 #[component]
 pub fn AppLayout() -> Element {
     rsx! {
-        div { class: "flex flex-col h-full",
+        // Empty ontouchstart lets iOS WKWebView fire CSS :active (button press scale).
+        div { class: "flex flex-col h-full", ontouchstart: move |_| {},
             ScrollToTop {}
-            main { id: "data-scroll-target", class: "overflow-y-auto flex-1 overflow-x-clip",
+            main {
+                id: "data-scroll-target",
+                // bottom__safe: pb = --bottom__nav__height on mobile, 0 at sm+.
+                class: "overflow-y-auto flex-1 overflow-x-clip bottom__safe",
                 Outlet::<Route> {}
             }
+            AppBottomNav {}
             CommandSearchDocsDialog {}
         }
     }
 }
```

Why here: `AppLayout` is the outermost `#[layout(..)]` in `src/main.rs` (opens
at the top of the `Route` enum, `#[end_layout]` before the `SidenavDemoLayout`
group). So the bar shows on every normal page and is automatically absent on the
full-screen routes that sit outside it (`SidenavDemo*`, `WorkflowViewPage`,
`ViewRouter`, `PageNotFound`), which is the behaviour we want.

`bottom__safe` on the scroll container (not a wrapper div) so the extra 4rem is
real scroll room and the last content row can be scrolled above the fixed bar.

## Optional refinements (not required for parity)

- Exact clearance including the home indicator: change the `bottom__safe`
  utility in `tailwind.css` to
  `padding-bottom: calc(var(--bottom__nav__height) + env(safe-area-inset-bottom))`.
  Leptos does not do this today; skip unless a device shows clipping.
- `nav_desktop.rs:61` still has a raw `a { href: "/blocks" }` (same class of iOS
  bug as `7ac104c`). Swap to `Link { to: Route::LoginBlocks {} }` in a separate
  change; out of scope here.

## Edge cases

1. **Routes outside `AppLayout`** (`SidenavDemo*`, `WorkflowViewPage`,
   `ViewRouter`, `PageNotFound`). The bar is mounted inside `AppLayout`, so it
   simply does not render there. Intended: those are full-screen views. No guard
   needed.
2. **`PageNotFound` reached via `/docs/hooks/:name` miss.** `HookPage` /
   `ComponentPage` render `PageNotFound` *inside* their own body, still under
   `AppLayout`, so the bar stays visible and the Hooks/Components item stays
   active. Fine, arguably desirable.
3. **`/charts` bare path.** Redirects to `/charts/area-chart` before render, so
   `use_route` never yields a bare `Charts` state. The `matches!` list covers
   all 6 concrete chart routes. If a 7th chart route is added later, add it to
   `is_active` (leave a comment on the arm).
4. **`/blocks` group.** No bottom-nav entry (Leptos had none either). `/blocks`
   redirects to `Route::LoginBlocks {}`; while on a blocks page no item is
   active, all five render inactive. Acceptable.
5. **Deep-link / cold load** (open `/docs/hooks/use-copy-clipboard` directly).
   `use_route::<Route>()` returns the resolved route on first render, so the
   correct item is active immediately, no flash.
6. **Fast repeated taps / double nav.** `navigator.push` to the current route is
   a no-op in dioxus-router; no history spam. No debounce needed.
7. **`sm` boundary (exactly 640px).** Tailwind `sm:` is `min-width: 640px`, so
   at exactly 640 the bar is hidden and `bottom__safe` padding is `0`. Matches
   navbar's own `sm` breakpoint, no gap where both or neither show.
8. **Landscape phone / short viewport.** Bar is `fixed`, height
   `--bottom__nav__height` (4rem). `<main>` is the scroll container with
   `bottom__safe` padding, so content still reaches above the bar. No
   `100vh`-vs-`dvh` trap because we pad the scroller, not a `vh`-sized box.
9. **Safe-area on non-notch devices.** `env(safe-area-inset-bottom, 0px)`
   fallback is `0`, so older/simulator devices get no dead space.
10. **Icon crate missing a symbol.** Verified `House / Blocks / Compass /
    Search / ChartSpline` all exist for the `dioxus` feature. If one is renamed
    on an `icons` bump, it is a compile error, not a silent blank.
11. **SSR / `fullstack` first paint.** `AppBottomNav` is pure render off
    `use_route`, no `use_effect`, no client-only API, so it renders identically
    server-side and hydrates without mismatch.
12. **Route enum has no bottom-nav-specific variant.** We navigate to existing
    index routes only; nothing to add to `src/main.rs`. Lower risk of route
    drift.

## Testing

1. `cargo check --features web` — compiles.
2. `dx serve` (web), narrow the window under 640px: bar visible, fixed to
   bottom, five items, active item tinted, tap navigates, content scrolls clear
   of the bar. Widen past 640px: bar gone, no leftover bottom padding.
3. `just ios` (`dx serve --platform ios`): bar sits above the home indicator,
   icons nudged down, tap has the press-scale, no tap-highlight flash, no
   "not an http url" error on navigation.
4. Route coverage: from `/docs/components/button` the Components item is active;
   from `/charts/bar-chart` the Charts item is active.

## Files touched

| File | Change |
|------|--------|
| `src/components/app_bottom_nav.rs` | new, ~95 lines |
| `src/components/mod.rs` | +1 `pub mod` |
| `src/routes/app_layout.rs` | mount `AppBottomNav`, `bottom__safe` on `<main>`, `ontouchstart` shim |
| `tailwind.css` | none (vars + utilities already present) |
| `CHANGELOG_DEV.md` | add under today's date: `### Improvements` -> mobile bottom nav |

## Changelog entry (draft)

```
### Improvements

- **Mobile bottom nav**: Added `AppBottomNav` (Home, Components, Hooks, Icons,
  Charts) shown below `sm`, mounted in `AppLayout`. Fixed to the viewport
  bottom, respects the iOS safe-area inset, active item tracks the current
  route via `use_route`. Ported from the Leptos site; reuses the existing
  `registry::ui::bottom_nav` primitives and the `bottom__safe` /
  `--bottom__nav__height` CSS already in `tailwind.css`.
  `src/components/app_bottom_nav.rs`, `src/components/mod.rs`,
  `src/routes/app_layout.rs`
```
