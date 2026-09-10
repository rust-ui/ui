# PLAN: Verbatim port of leptos sidenav blocks to dioxus

Status: IMPLEMENTED on 2026-09-10. Registry primitives, route layouts, all 11
sidenav variants, responsive sheets, search filtering, and sidenav URL mapping
are ported. Remaining validation is visual/browser parity review.

Goal (user, verbatim): "bah je veux tout pareil que leptos" then "note que je
veux vraiment verbatim hein, invente rien et match parfaitement, si ya des trucs
qui marchent pas du fait de dioxus, documente le". So: reproduce the leptos
sidenav blocks in dioxus with the same structure and behaviour; where Dioxus
architecture blocks an exact copy, document the gap in this file (section
"Dioxus limitations") instead of improvising a different design.

Two decisions already taken with the user:

1. Routing: "le plus proche du comportement de leptos" -> real Dioxus nested
   routing (`#[layout(...)]` + child `#[route]` + `Outlet`), NOT a
   `/:..segments` catch-all with internal string matching.
2. Generator: "Editer all_blocks.rs a la main pour l'instant" -> hand-edit
   `src/__registry__/all_blocks.rs`; do not patch the private generator repo
   `rust_ui_internals/build_registry_dioxus`.

---

## 0. Current state

### Leptos (port source), `leptos-ui/app_crates/registry/src/blocks/`

- `sidenav_routes.rs`: `SidenavRoutes` (Sidenav01..11), `DocsRoutes`
  (Components, Hooks), `ComponentsRoutes` (Accordion, Alert, AlertDialog,
  Button), `HooksRoutes` (UseCopyClipboard, UseLockBodyScroll, UseRandom).
  strum kebab-case + `heck::ToTitleCase`. Helpers:
  `SidenavRoutes::view_segment() -> "view"`, `from_path`, `to_route`,
  `to_title`; `DocsRoutes::base_segment() -> "docs"`, `to_title`;
  `ComponentsRoutes/HooksRoutes::base_segment()`, `base_url_with_sidenav`,
  `to_route_with_sidenav`, `to_title`.
- `sidenav_routes_selector.rs`: `SidenavRoutesSelector(current_section:
  Memo<DocsRoutes>, sidenav_route: SidenavRoutes)`. A `DropdownMenu`
  (`align=Center`) with a trigger (icon box: `LayoutTemplate` for Components /
  `Sparkles` for Hooks; "Docs" label + `current_section.to_title()` subtitle;
  `ChevronsUpDown`) and a content with 2 `DropdownMenuItem >
  DropdownMenuAction href=<Components|Hooks>::base_url_with_sidenav(sidenav)`.
- `sidenav_routes_simplified.rs`: `#[component(transparent)]
  SidenavRoutesSimplified(data_variant: Option<SidenavVariant>) -> impl
  MatchNestedRoutes + Clone`. Contributes:
  `docs` (view = `SidenavInsetRight(data_variant?)`)
    - `""` -> `()`
    - `components` (view = `Outlet`) -> `""` -> `()`, `*component_path` -> `()`
    - `hooks` (view = `Outlet`) -> `""` -> `()`, `*hook_path` -> `()`
  IMPORTANT: `SidenavInsetRight` has NO `<Outlet/>` inside it. The child routes
  render `()`. They exist so the URLs resolve; they never show content.
- `sidenav_inset_right.rs`: `SidenavInsetRight(data_variant:
  Option<SidenavVariant>)`.
  - `breadcrumb_items = use_breadcrumb_from_segment("docs")` (reactive `Memo`
    in leptos).
  - `current_section = Memo(|_| path.contains("components") ? Components :
    Hooks)`.
  - `sidenav_route = Memo(|_| SidenavRoutes::from_path(&pathname))`.
  - `<SidenavInset attr:data-variant=data_variant.map(to_string)>`
    - `<header class="flex gap-2 items-center h-16 ease-linear shrink-0
      transition-[width,height]
      group-has-data-[collapsible=icon]/sidenav-wrapper:h-12">`
      - `<div class="flex gap-2 items-center px-4">`
        - `match sidenav_route.get()` -> `SidenavNNMobileSheet {
          current_section, sidenav_route=SidenavNN }` (explicit arms
          Sidenav02..11; `route =>` default -> `Sidenav01MobileSheet`)
        - `<div class="hidden md:block"><SidenavTrigger><PanelLeft/><span
          class="hidden">Toggle Sidenav</span></SidenavTrigger></div>`
        - `<Separator orientation=Vertical class="-ml-1 h-4" />`
        - `<Breadcrumb><BreadcrumbList>` iterate
          `(idx, (name, href, is_last))`: separator if `idx > 0`;
          `<BreadcrumbItem>` -> `is_last ? <BreadcrumbPage>{name}` :
          `<BreadcrumbLink attr:href=href>{name}`
    - `<div class="flex flex-col flex-1 gap-4 p-4 pt-0">`
      - `<div class="grid auto-rows-min gap-4 md:grid-cols-3">` x3
        `<div class="rounded-xl bg-muted/50 aspect-video">`
      - `<div class="flex-1 rounded-xl bg-muted/50 min-h-[100vh]
        md:min-h-min">`
- `sidenav01.rs` .. `sidenav11.rs`: each has
  - `#[component(transparent)] SidenavNNRoutes() -> impl MatchNestedRoutes +
    Clone`: `<ParentRoute path=StaticSegment("view")
    view=Outlet><ParentRoute path=StaticSegment(SidenavRoutes::SidenavNN.as_ref())
    view=SidenavLayout(sidenav_route=SidenavNN)><SidenavRoutesSimplified
    [data_variant=..] /></ParentRoute></ParentRoute>`.
  - `SidenavLayout(sidenav_route)`: `use_location`; `current_section` Memo;
    `<div class="bg-background"><SidenavWrapper attr:style="--sidenav-width:16rem;">
    <Sidenav [variant=..]><SidenavNNContent current_section sidenav_route/>
    </Sidenav><Outlet/></SidenavWrapper></div>`.
    - sidenav04: `<Sidenav variant=Floating>`.
    - sidenav08: `<Sidenav variant=Inset>` and
      `<SidenavRoutesSimplified data_variant=Inset />`.
    - sidenav09: `<SidenavWrapper attr:style="--sidenav-width:18rem;
      --sidenav-width-icon:3rem">`; `<Sidenav data_collapsible=Icon
      class="overflow-hidden *:data-[sidenav=Sidenav]:flex-row">` wrapping
      `<FirstSidenav .../>` + `<SecondSidenav .../>` then `<Outlet/>`.
    - sidenav11: `<SidenavWrapper attr:style="--sidenav-width:16rem;">
      <Outlet/><Sidenav data_side=Right><Sidenav11Content .../></Sidenav>
      </SidenavWrapper>` (Outlet FIRST, no `div.bg-background` wrapper... see
      note in per-block section).
  - `SidenavNNContent(current_section: Memo<DocsRoutes>, sidenav_route)`:
    shared by desktop `<Sidenav>` and mobile `<Sheet>`. `let sheet_ctx =
    use_context::<SheetContext>()` (Some inside Sheet, None on desktop). Header,
    content (links), footer `<DemoDropdownMenuUser/>`. Per-block body differs
    (see per-block section).
  - `SidenavNNMobileSheet(current_section, sidenav_route)`: `<Sheet><div
    class="md:hidden"><SheetTrigger class="size-7" variant=Ghost size=Icon>
    <PanelLeft class="size-4"/><span class="hidden">Toggle Sidenav</span>
    </SheetTrigger></div><SheetContent direction=Left class="p-0 w-[18rem]
    bg-sidenav text-sidenav-foreground" show_close_button=false><div
    class="flex flex-col h-full"><SidenavNNContent .../></div></SheetContent>
    </Sheet>`. sidenav11: `direction=Right`, `<PanelLeft class="rotate-180
    size-4" />`.
  - Link wrap inside content: `if let Some(target_id) = sheet_target_id { <div
    data-sheet-close=target_id.clone()><SidenavLink href=*href>{*title}
    </SidenavLink></div> } else { <SidenavLink href=*href>{*title}</SidenavLink>
    }`.
  - `COMPONENT_LINKS` (7): `/view/sidenavNN/docs/components/{accordion, alert,
    alert-dialog, button, card, checkbox, dialog}` with titles Accordion,
    Alert, Alert Dialog, Button, Card, Checkbox, Dialog.
  - `HOOKS_LINKS` (3): `/view/sidenavNN/docs/hooks/{use-copy-clipboard,
    use-lock-body-scroll, use-random}` with titles Use Copy Clipboard, Use Lock
    Body Scroll, Use Random.

### Dioxus (port target)

- `app_crates/registry/src/blocks/sidenav01.rs` .. `sidenav11.rs`,
  `sidenav_routes.rs`, `sidenav_routes_selector.rs`,
  `sidenav_routes_simplified.rs`, `sidenav_inset_right.rs`: ALL currently
  invented static mockups (plain `div`/`aside`/`a`, hardcoded fake copy). Zero
  `ui::sidenav` usage, no routing, no Sheet, no DropdownMenu. Full rewrite.
- `app_crates/registry/src/blocks/sidenav_routes.rs`: hand-rolled enums,
  `to_title()` returns `"Sidenav 01"` (space); missing
  `base_segment`/`base_url_with_sidenav`/`to_route_with_sidenav` on
  Docs/Components/Hooks; `ComponentsRoutes` has 12 variants (leptos has 4).
- `app_crates/registry/Cargo.toml`: no `heck` dep. `heck 0.5.0` is already in
  `Cargo.lock` (transitive), so adding `heck = "0.5"` needs no network.
- Primitives already present and near-parity: `ui/sidenav.rs`, `ui/sheet.rs`,
  `ui/accordion.rs`, `ui/dropdown_menu.rs`, `ui/breadcrumb.rs`,
  `ui/separator.rs`, `ui/label.rs`. `DropdownMenuTriggerEllipsis` lives in
  `ui/sidenav.rs`.
- `demos/demo_dropdown_menu_user.rs` + `demo_dropdown_menu_user_icon.rs`: both
  exist in dioxus, RSX equivalents of the leptos ones. Reuse as-is.
- Icons (`icons` crate, `features=["dioxus"]`, components take `class:
  Option<String>`): PanelLeft, Search, ChevronsUpDown, LayoutTemplate,
  Sparkles, ChevronRight, BookOpen, Bot, SquareTerminal, Settings, Frame,
  Ellipsis, Component, Layers all confirmed available.
- `src/main.rs` `Route` enum has `#[rustfmt::skip]`. Sidenav iframe currently
  routed via `#[route("/view/block/:id")] ViewRouter { id }` ->
  `BlockIdKebab::to_component()` -> `registry::blocks::sidenavNN::SidenavNN()`.
- `src/__registry__/all_blocks.rs` (generated, hand-edit approved):
  `BlockIdKebab::to_full_view_url(&self) -> String { format!("/view/block/{}",
  self) }` where `Display` = dash form (`"sidenav-01"`).
  `to_component()` maps each `Sidenav01..11` to
  `registry::blocks::sidenavNN::SidenavNN()`.

---

## 1. Routing changes (`src/main.rs`)

Add a nested layout group as a sibling of `WorkflowViewPage` / `ViewRouter`
(that is, OUTSIDE `#[layout(AppLayout)]`, so the iframe page has no site
header/footer, exactly like `ViewRouter` today and like leptos `/view/*`):

```
    #[end_layout]                               // end AppLayout (unchanged)
    #[route("/view/:id")]
    WorkflowViewPage { id: String },

    // ---- sidenav demo: verbatim port of leptos SidenavNNRoutes +
    //      SidenavRoutesSimplified. See PLAN_SIDENAV_BLOCKS_VERBATIM_PORT.md.
    #[layout(SidenavDemoLayout)]
        #[layout(SidenavInsetRightLayout)]
            #[route("/view/:sidenav/docs")]
            SidenavDemoDocs { sidenav: String },
            #[route("/view/:sidenav/docs/components")]
            SidenavDemoComponents { sidenav: String },
            #[route("/view/:sidenav/docs/components/:name")]
            SidenavDemoComponentPage { sidenav: String, name: String },
            #[route("/view/:sidenav/docs/hooks")]
            SidenavDemoHooks { sidenav: String },
            #[route("/view/:sidenav/docs/hooks/:name")]
            SidenavDemoHookPage { sidenav: String, name: String },
        #[end_layout]
    #[end_layout]

    #[route("/view/block/:id")]
    ViewRouter { id: String },
    #[route("/:..segments")]
    PageNotFound { segments: Vec<String> },
```

Route specificity check: `/view/block/x` keeps matching `ViewRouter` (`block`
literal beats `:sidenav`). `/view/x` (2 segs) keeps matching `WorkflowViewPage`.
`/view/x/docs...` (>=3 segs) matches the new group. `/:..segments` catch-all
still loses to any concrete route.

New imports in `main.rs`:

```rust
use domain::blocks::routing::sidenav_demo_layout::{SidenavDemoLayout, SidenavInsetRightLayout};
use domain::blocks::routing::sidenav_demo_pages::{
    SidenavDemoComponentPage, SidenavDemoComponents, SidenavDemoDocs, SidenavDemoHookPage, SidenavDemoHooks,
};
```

### Deviation vs leptos (documented, user-approved)

Leptos contributes 11 independent route subtrees, one per block, keyed on
`StaticSegment(SidenavRoutes::SidenavNN.as_ref())` (each `SidenavNNRoutes`
transparent component). Dioxus `#[route]` needs literal path strings, cannot
parametrize a static segment, and the `registry` crate (a dependency) cannot
add `Route` variants. So the 11 subtrees collapse to one `:sidenav` dynamic
segment plus `match SidenavRoutes::from_path(path)` inside the layout. Still
real nested routing with `Outlet`, not a catch-all. This is the single
approved structural deviation.

---

## 2. New bin-crate files

### `src/domain/blocks/routing/sidenav_demo_layout.rs`

```rust
use dioxus::prelude::*;
use registry::blocks::sidenav_inset_right::SidenavInsetRight;
use registry::blocks::sidenav_routes::{DocsRoutes, SidenavRoutes};
use registry::ui::sidenav::{SidenavVariant, SidenavWrapper};
use registry::blocks::{sidenav01, sidenav02, /* ... */ sidenav11};
use crate::Route;

#[component]
pub fn SidenavDemoLayout() -> Element {
    let path = use_route::<Route>().to_string();          // subscribe -> reactive
    let sidenav_route = SidenavRoutes::from_path(&path);
    let current_section = if path.contains(DocsRoutes::Components.as_ref()) {
        DocsRoutes::Components
    } else {
        DocsRoutes::Hooks
    };

    rsx! {
        div { class: "bg-background",
            match sidenav_route {
                SidenavRoutes::Sidenav09 => rsx! {
                    SidenavWrapper { style: "--sidenav-width:18rem;--sidenav-width-icon:3rem",
                        sidenav09::Sidenav09Sidebar { current_section, sidenav_route }
                        Outlet::<Route> {}
                    }
                },
                SidenavRoutes::Sidenav11 => rsx! {
                    SidenavWrapper { style: "--sidenav-width:16rem;",
                        Outlet::<Route> {}
                        sidenav11::Sidenav11Sidebar { current_section, sidenav_route }
                    }
                },
                other => rsx! {
                    SidenavWrapper { style: "--sidenav-width:16rem;",
                        SidenavDemoSidebar { sidenav_route: other, current_section }
                        Outlet::<Route> {}
                    }
                },
            }
        }
    }
}

// dispatch helper for the "default shape" blocks (01..08, 10)
#[component]
fn SidenavDemoSidebar(sidenav_route: SidenavRoutes, current_section: DocsRoutes) -> Element {
    match sidenav_route {
        SidenavRoutes::Sidenav02 => rsx! { sidenav02::Sidenav02Sidebar { current_section, sidenav_route } },
        // ... 03..08, 10
        _ => rsx! { sidenav01::Sidenav01Sidebar { current_section, sidenav_route } },
    }
}

#[component]
pub fn SidenavInsetRightLayout() -> Element {
    let path = use_route::<Route>().to_string();          // subscribe -> breadcrumb reactive
    let sidenav_route = SidenavRoutes::from_path(&path);
    let data_variant = match sidenav_route {
        SidenavRoutes::Sidenav08 => Some(SidenavVariant::Inset),
        _ => None,
    };
    rsx! {
        SidenavInsetRight { path, data_variant }
    }
}
```

Notes:
- `SidenavNNSidebar` is the registry-side component that renders exactly the
  leptos `<Sidenav [variant=..]>{SidenavNNContent}</Sidenav>` subtree (NOT the
  `div.bg-background` / `SidenavWrapper` / `Outlet` wrapper, which live here in
  the bin crate because `Outlet::<Route>` needs the `Route` type).
- sidenav09 `Sidenav09Sidebar` renders the full
  `<Sidenav data_collapsible=Icon class="...">{FirstSidenav}{SecondSidenav}
  </Sidenav>` (still no wrapper/outlet).
- Passing `path` as a prop (instead of the registry component reading
  `window.location` via `web_sys`) keeps the breadcrumb / active-link updates
  reactive on client navigation, matching leptos `Memo` semantics. See "Dioxus
  limitations" #4.

### `src/domain/blocks/routing/sidenav_demo_pages.rs`

```rust
use dioxus::prelude::*;

#[component] pub fn SidenavDemoDocs(sidenav: String) -> Element { rsx! {} }
#[component] pub fn SidenavDemoComponents(sidenav: String) -> Element { rsx! {} }
#[component] pub fn SidenavDemoComponentPage(sidenav: String, name: String) -> Element { rsx! {} }
#[component] pub fn SidenavDemoHooks(sidenav: String) -> Element { rsx! {} }
#[component] pub fn SidenavDemoHookPage(sidenav: String, name: String) -> Element { rsx! {} }
```

Verbatim of leptos `<Route ... view=|| () />`.

### `src/domain/blocks/routing/mod.rs`

Add `pub mod sidenav_demo_layout;` and `pub mod sidenav_demo_pages;`.

---

## 3. Registry primitive changes (`app_crates/registry/src/ui/`)

| File / item | Change | Reason |
| --- | --- | --- |
| `sidenav.rs` `SidenavWrapper` | add `#[props(into, optional)] style: Option<String>`; render `style: "--sidenav-width: 256px;{}", style.as_deref().unwrap_or("")` (custom value appended so it wins) | leptos passes `attr:style="--sidenav-width:16rem;"` (16rem == 256px, no-op for 01-08/10/11) and sidenav09 needs `--sidenav-width:18rem;--sidenav-width-icon:3rem` |
| `sidenav.rs` `SidenavInput` | add `#[props(into, optional)] value: Option<String>` and `#[props(optional)] oninput: Option<EventHandler<FormEvent>>`; forward to the inner `input` | sidenav10 reactive search filter (`search_query_signal` + `on:input`) |
| `sheet.rs` `SheetContext` + field `target_id` | make both `pub` | leptos `SidenavNNContent` does `use_context::<SheetContext>()` then reads `ctx.target_id` for the `data-sheet-close` wrapper. Dioxus port uses `try_consume_context::<SheetContext>()`. (Dioxus Sheet close JS only checks attribute presence, but we still want the real id for parity.) |

Everything else (`SidenavInset` has no `data-variant` prop; `SidenavGroup`
etc. have no `attr:data-sidenav` passthrough) is handled by adding the small
props below or documented as cosmetic. See "Dioxus limitations".

Extra props needed for verbatim class/attr parity (small, additive, no
behaviour change):

- `sidenav.rs` `SidenavInset`: add `#[props(into, optional)] data_variant:
  Option<String>` -> render `"data-variant": data_variant` (leptos
  `attr:data-variant=...`).
- `sidenav.rs` `SidenavGroup`, `SidenavGroupContent`: accept an optional
  `#[props(into, optional)] data_sidenav: Option<String>` -> render
  `"data-sidenav": ...` (leptos `attr:data-sidenav="group"` /
  `"group-content"` in sidenav01/03/10 search form). If this turns out to
  cascade into many primitives, fall back to documenting it as cosmetic and
  skip (search form styling only).

---

## 4. Registry: `blocks/sidenav_routes.rs` rewrite (match leptos)

Replace the hand-rolled file with a straight port:

```rust
use heck::ToTitleCase;
use strum::{AsRefStr, Display, EnumIter, EnumString, IntoStaticStr};

#[derive(Clone, Copy, Display, AsRefStr, IntoStaticStr, EnumString, EnumIter, Debug, PartialEq)]
#[strum(serialize_all = "kebab-case")]
pub enum SidenavRoutes { Sidenav01, Sidenav02, /* ... */ Sidenav11 }

impl SidenavRoutes {
    pub fn view_segment() -> &'static str { "view" }
    pub fn from_path(path: &str) -> Self {
        use strum::IntoEnumIterator;
        Self::iter().rev().find(|r| path.contains(r.as_ref())).unwrap_or(Self::Sidenav01)
    }
    pub fn to_route(self) -> String { format!("{}/{}", Self::view_segment(), self.as_ref()) }
    pub fn to_title(self) -> String { self.as_ref().to_title_case() }
}

#[derive(... same derives ...)]
#[strum(serialize_all = "kebab-case")]
pub enum DocsRoutes { Components, Hooks }
impl DocsRoutes {
    pub fn base_segment() -> &'static str { "docs" }
    pub fn to_title(self) -> String { self.as_ref().to_title_case() }
}

#[derive(... same derives ...)]
#[strum(serialize_all = "kebab-case")]
pub enum ComponentsRoutes { Accordion, Alert, AlertDialog, Button }   // shrink 12 -> 4
impl ComponentsRoutes {
    pub fn base_segment() -> &'static str { "components" }
    pub fn base_url_with_sidenav(sidenav: SidenavRoutes) -> String {
        format!("/{}/{}/{}", sidenav.to_route(), DocsRoutes::base_segment(), DocsRoutes::Components.as_ref())
    }
    pub fn to_route_with_sidenav(self, sidenav: SidenavRoutes) -> String {
        format!("{}/{}", Self::base_url_with_sidenav(sidenav), self.as_ref())
    }
    pub fn to_title(self) -> String { self.as_ref().to_title_case() }
}

#[derive(... same derives ...)]
#[strum(serialize_all = "kebab-case")]
pub enum HooksRoutes { UseCopyClipboard, UseLockBodyScroll, UseRandom }
impl HooksRoutes {
    pub fn base_segment() -> &'static str { "hooks" }
    pub fn base_url_with_sidenav(sidenav: SidenavRoutes) -> String {
        format!("/{}/{}/{}", sidenav.to_route(), DocsRoutes::base_segment(), DocsRoutes::Hooks.as_ref())
    }
    pub fn to_route_with_sidenav(self, sidenav: SidenavRoutes) -> String {
        format!("{}/{}", Self::base_url_with_sidenav(sidenav), self.as_ref())
    }
    pub fn to_title(self) -> String { self.as_ref().to_title_case() }
}
```

Check callers of the current dioxus `ComponentsRoutes` (12 variants) before
shrinking. If a non-sidenav file imports the dropped variants, keep the extra
variants (they are harmless) rather than break that file, and note it here.

`Cargo.toml` (`app_crates/registry/`): add under `[dependencies]`

```toml
heck = "0.5"
```

---

## 5. Registry: `blocks/sidenav_routes_selector.rs` rewrite

leptos `Memo<DocsRoutes>` has no dioxus equivalent as a param type; pass plain
values (the parent layout already recomputes on nav).

```rust
#[component]
pub fn SidenavRoutesSelector(current_section: DocsRoutes, sidenav_route: SidenavRoutes) -> Element {
    let docs_routes = [DocsRoutes::Components, DocsRoutes::Hooks];
    rsx! {
        DropdownMenu { align: DropdownMenuAlign::Center,
            DropdownMenuTrigger { class: "flex justify-between px-2 w-full h-12 bg-transparent border-0",
                div { class: "flex gap-2 items-center",
                    div { class: "flex justify-center items-center rounded-lg bg-primary text-primary-foreground aspect-square size-8",
                        match current_section {
                            DocsRoutes::Components => rsx! { LayoutTemplate {} },
                            DocsRoutes::Hooks => rsx! { Sparkles {} },
                        }
                    }
                    div { class: "grid flex-1 text-sm leading-tight text-left",
                        span { class: "font-medium", "Docs" }
                        span { class: "text-xs", "{current_section.to_title()}" }
                    }
                }
                ChevronsUpDown {}
            }
            DropdownMenuContent {
                DropdownMenuGroup {
                    for doc_route in docs_routes {
                        DropdownMenuItem {
                            DropdownMenuAction {
                                href: match doc_route {
                                    DocsRoutes::Components => ComponentsRoutes::base_url_with_sidenav(sidenav_route),
                                    DocsRoutes::Hooks => HooksRoutes::base_url_with_sidenav(sidenav_route),
                                },
                                "{doc_route.to_title()}"
                            }
                        }
                    }
                }
            }
        }
    }
}
```

leptos wraps `DropdownMenuContent` in `<Portal>`; dioxus `DropdownMenuContent`
already portals internally (see `demo_dropdown_menu_user.rs`), so no wrapper.

---

## 6. Registry: `blocks/sidenav_routes_simplified.rs`

leptos `#[component(transparent)]` returning `impl MatchNestedRoutes` has NO
Dioxus equivalent. Its entire job (contribute `docs` / `docs/components[/*]` /
`docs/hooks[/*]` routes whose leaves render `()`, with `SidenavInsetRight` as
the `docs` view) is taken over by:

- `src/main.rs` route entries (section 1), and
- `SidenavInsetRightLayout` + `SidenavDemoDocs/...` pages (section 2).

Action: delete `sidenav_routes_simplified.rs` and drop its `mod` line, OR keep
the file with just a doc comment pointing here. Prefer delete + a note in
`blocks/mod.rs`.

---

## 7. Registry: `blocks/sidenav_inset_right.rs` rewrite

```rust
#[component]
pub fn SidenavInsetRight(path: String, data_variant: Option<SidenavVariant>) -> Element {
    let breadcrumb_items = breadcrumb_from_path(&path, DocsRoutes::base_segment()); // pure fn, see below
    let current_section = if path.contains(DocsRoutes::Components.as_ref()) {
        DocsRoutes::Components
    } else {
        DocsRoutes::Hooks
    };
    let sidenav_route = SidenavRoutes::from_path(&path);

    rsx! {
        SidenavInset {
            data_variant: data_variant.map(|v| v.to_string()),
            header {
                class: "flex gap-2 items-center h-16 ease-linear shrink-0 transition-[width,height] group-has-data-[collapsible=icon]/sidenav-wrapper:h-12",
                div { class: "flex gap-2 items-center px-4",
                    match sidenav_route {
                        SidenavRoutes::Sidenav02 => rsx! { sidenav02::Sidenav02MobileSheet { current_section, sidenav_route: SidenavRoutes::Sidenav02 } },
                        // ... 03..11 explicit arms ...
                        other => rsx! { sidenav01::Sidenav01MobileSheet { current_section, sidenav_route: other } },
                    }
                    div { class: "hidden md:block",
                        SidenavTrigger {
                            PanelLeft {}
                            span { class: "hidden", "Toggle Sidenav" }
                        }
                    }
                    Separator { orientation: SeparatorOrientation::Vertical, class: "-ml-1 h-4" }
                    Breadcrumb {
                        BreadcrumbList {
                            for (idx , (name , href , is_last)) in breadcrumb_items.into_iter().enumerate() {
                                if idx > 0 { BreadcrumbSeparator {} }
                                BreadcrumbItem {
                                    if is_last {
                                        BreadcrumbPage { "{name}" }
                                    } else {
                                        BreadcrumbLink { href, "{name}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            div { class: "flex flex-col flex-1 gap-4 p-4 pt-0",
                div { class: "grid auto-rows-min gap-4 md:grid-cols-3",
                    div { class: "rounded-xl bg-muted/50 aspect-video" }
                    div { class: "rounded-xl bg-muted/50 aspect-video" }
                    div { class: "rounded-xl bg-muted/50 aspect-video" }
                }
                div { class: "flex-1 rounded-xl bg-muted/50 min-h-[100vh] md:min-h-min" }
            }
            Outlet-equivalent: NONE here (leptos has none either). The bin-crate
            SidenavInsetRightLayout owns `Outlet::<Route>{}`; it is placed AFTER
            <SidenavInsetRight/> in that layout so the (empty) leaf pages mount
            without adding visible markup. Revisit: if dioxus requires the
            Outlet to be a descendant of the layout component's own subtree, move
            the `Outlet::<Route>{}` call to the end of this component and accept
            that `SidenavInsetRight` then needs the `Route` type (make it a
            bin-crate component instead of registry). Decide during impl.
        }
    }
}
```

`breadcrumb_from_path(path, "docs")`: pure reimplementation of dioxus
`use_breadcrumb_from_segment` (which is non-reactive, reads `web_sys`). Same
output shape `Vec<(title, "/abs/path", is_last)>`: find the `"docs"` segment,
take it plus every segment after it, build cumulative hrefs, title-case each
label (`heck::ToTitleCase`), last one `is_last = true`. Keeping it pure over
the `path` prop preserves leptos reactive-`Memo` behaviour.

---

## 8. Registry: `blocks/sidenavNN.rs` rewrites (11 files)

Each file exports (all `pub`):

- `SidenavNNSidebar(current_section: DocsRoutes, sidenav_route: SidenavRoutes)`
  -> `rsx! { Sidenav { [variant/side/collapsible as per block] SidenavNNContent
  { current_section, sidenav_route } } }`. (sidenav09: renders the dual-sidenav
  tree instead, see below.)
- `SidenavNNContent(current_section: DocsRoutes, sidenav_route: SidenavRoutes)`
  -> shared body. `let sheet_ctx = try_consume_context::<SheetContext>();` then
  `let sheet_target_id = sheet_ctx.map(|c| c.target_id);`.
- `SidenavNNMobileSheet(current_section: DocsRoutes, sidenav_route:
  SidenavRoutes)` -> the `Sheet` wrapper (identical across blocks except
  sidenav11 direction/icon).
- `SidenavNN()` -> keep as a thin `pub fn` returning `rsx! {}` so
  `all_blocks.rs::to_component()` still compiles. (After section 9 the sidenav
  arm of `to_component` is unreachable, but must type-check.) Alternative:
  change those arms to `rsx! {}` in `all_blocks.rs` and delete `SidenavNN()`.
  Prefer keeping `SidenavNN()` empty (smaller diff to generated file).
- `const COMPONENT_LINKS` / `const HOOKS_LINKS` (as in leptos, `/view/sidenavNN/
  docs/...`).

Link rendering helper (used in every content body):

```rust
// inside SidenavNNContent, `links: &[(&str, &str)]`
for (href , title) in links.iter().copied() {
    if let Some(target_id) = sheet_target_id.clone() {
        div { "data-sheet-close": target_id,
            SidenavLink { href, "{title}" }
        }
    } else {
        SidenavLink { href, "{title}" }
    }
}
```

Per-block body (from the leptos files):

- sidenav01 "Sidenav with Grouped Sections": header = `SidenavRoutesSelector` +
  search `<form>` (`SidenavGroup data_sidenav="group"` > `SidenavGroupContent
  data_sidenav="group-content" class="relative"` > `Label html_for="search"
  class="hidden" "Search"` + `SidenavInput class="pl-8" id="search"
  placeholder="Search the docs..."` + `Search` icon
  `class="absolute left-2 top-1/2 opacity-50 -translate-y-1/2 pointer-events-none
  select-none size-4"`). content = `SidenavGroup` >
  `SidenavGroupLabel{current_section.to_title()}` > `SidenavGroupContent` >
  `SidenavMenu` > links. footer = `DemoDropdownMenuUser`.
- sidenav02 "Sidenav with Collapsible Menus": header = selector only. content =
  `SidenavGroup > SidenavGroupContent > SidenavGroupLabel{title} > SidenavMenu >
  SidenavMenuItem > AccordionItem`: `AccordionTrigger open=true
  class="p-2 peer-checked:bg-accent hover:bg-accent" > AccordionHeader >
  AccordionTitle{current_section.to_title()}`; `AccordionContent class="pt-0"` >
  links (NO `SidenavMenuSub`).
- sidenav03 "Sidenav with Submenus": header HAS search form. content =
  `SidenavGroup > SidenavGroupLabel{title} > SidenavGroupContent > SidenavMenu >
  SidenavMenuSub` > links.
- sidenav04 "Floating Sidenav with Submenus": `SidenavNNSidebar` uses `Sidenav
  variant=Floating`. `current_section` in the leptos `SidenavLayout` is derived
  the same way (path.contains components). content = `SidenavContent >
  SidenavGroup > SidenavGroupLabel{current_section.to_title()} >
  SidenavGroupContent > SidenavMenu` > links. footer `DemoDropdownMenuUser`.
- sidenav05 "Sidenav with Collapsible Submenus": `Sidenav` default. content =
  `SidenavGroup > SidenavGroupContent > SidenavGroupLabel{title} > SidenavMenu >
  SidenavMenuItem > AccordionItem`: `AccordionTrigger open=true
  class="p-2 peer-checked:bg-accent hover:bg-accent" > AccordionHeader >
  AccordionTitle{title}`; `AccordionContent class="p-0" > SidenavMenuSub` >
  links.
- sidenav06 "Sidenav with Dropdown Submenus": static (no reactive block).
  `SidenavHeader > SidenavRoutesSelector`. `SidenavContent > SidenavGroup >
  SidenavGroupLabel "Navigation" > SidenavGroupContent > SidenavMenu` with 2
  `SidenavMenuItem`, each a `DropdownMenu align=End`:
  - trigger `class="flex overflow-hidden gap-2 items-center p-2 w-full h-8
    text-sm text-left rounded-md focus-visible:ring-2 peer/menu-button
    ring-sidenav-ring outline-hidden transition-[width,height,padding]
    hover:bg-sidenav-accent hover:text-sidenav-accent-foreground"` >
    `LayoutTemplate class="size-4 shrink-0"` (Hooks item: `Sparkles`) + `span
    class="flex-1 truncate"` "Components" / "Hooks" + `ChevronRight
    class="opacity-50 size-4 shrink-0"`.
  - content `DropdownMenuContent > DropdownMenuGroup` > for each link
    `DropdownMenuItem > DropdownMenuAction href=*href {*title}`.
  footer `DemoDropdownMenuUser`. MobileSheet standard (no `SheetContext`
  import).
- sidenav07 "Collapsible Sidenav with Icons": content reactive block = first
  `SidenavGroup > SidenavGroupContent > SidenavGroupLabel{title} > SidenavMenu`
  with 4 `SidenavMenuItem > AccordionItem`:
  1. `AccordionTrigger open=true class="p-2 peer-checked:bg-accent
     hover:bg-accent" > AccordionHeader > BookOpen + AccordionTitle{title}`;
     `AccordionContent class="p-0" > SidenavMenuSub` > links (sheet-wrapped).
  2. `AccordionTrigger` (no `open`) `> Bot + AccordionTitle "Models"`;
     `AccordionContent` > 4x `SidenavLink href="#" "Link"`.
  3. `SquareTerminal + AccordionTitle "Playground"` + 4x Link.
  4. `Settings + AccordionTitle "Settings"` + 4x Link.
  Then OUTSIDE the reactive block: `SidenavGroup > SidenavGroupLabel "Projects"
  > SidenavMenu > ListExampleProjects {} ListExampleProjects {}`.
  `ListExampleProjects()`: `const SHARED_CLASS_PEER_MENU_BUTTON: &str = "..."`
  (copy verbatim from leptos), renders `SidenavMenuItem > a href="#"
  data-name="sidenav-menu-button" data-sidenav="menu-button"
  data-size="default" class={SHARED} > Frame + span "Design Engineering"` +
  `DropdownMenuTriggerEllipsis aria-haspopup="menu" aria-expanded="false"
  data-state="closed" > Ellipsis + span class="hidden" "More"`.
- sidenav08 "Inset Sidenav with Secondary Navigation": `Sidenav variant=Inset`;
  `SidenavInsetRightLayout` passes `data_variant=Some(Inset)`. content =
  Accordion (`AccordionTrigger open=true class="p-2 peer-checked:bg-accent
  hover:bg-accent"`) + `AccordionContent class="p-0" > SidenavMenuSub` > links.
- sidenav09 "Nested Sidenav with Route-Based Navigation": `Sidenav09Sidebar`
  renders `Sidenav data_collapsible=Icon class="overflow-hidden
  *:data-[sidenav=Sidenav]:flex-row"` wrapping `FirstSidenav
  { data_collapsible: SidenavCollapsible::None, sidenav_route }` +
  `SecondSidenav { data_collapsible: SidenavCollapsible::None, current_section
  }`. (The bin-crate layout puts `Outlet` after it.)
  - `FirstSidenav`: `const SHARED_TOOLTIP_CLASS` + inline `<style>` CSS-counter
    tooltip (copy verbatim); `Sidenav data_collapsible class="border-r
    w-[calc(var(--sidenav-width-icon)+1px)]!"` with two `SidenavMenuButton
    href=format!("/{}/docs/{}", sidenav_route.to_route(),
    DocsRoutes::Components.as_ref())` (`Component` icon) and `...Hooks.as_ref()`
    (`Layers` icon); footer `DemoDropdownMenuUserIcon`.
  - `SecondSidenav`: `Sidenav data_collapsible class="hidden flex-1 md:flex"`,
    header `span class="font-medium" "Routes"`, Accordion + `SidenavMenu` >
    links.
  - `Sidenav09Content` (mobile-sheet body only): selector header, Accordion +
    `SidenavMenu` with `data-sheet-close` wrapping, `DemoDropdownMenuUser`
    footer.
  - `Sidenav09MobileSheet` standard.
- sidenav10 "Sidenav with Search": `search_query` state
  (`let mut q = use_signal(String::new)`). Search `SidenavInput value:"{q}"
  oninput: move |e| q.set(e.value())`. content: `let query =
  q().to_lowercase(); let filtered = links.iter().filter(|(_, t)|
  t.to_lowercase().contains(&query))`. Empty state `div class="py-4 px-2
  text-sm text-muted-foreground" "No results found"` when `filtered.is_empty()
  && !query.is_empty()`. Plain `SidenavMenu` links.
- sidenav11 "Right-Side Sidenav": `Sidenav11Sidebar` = `Sidenav
  data_side=Right > Sidenav11Content`. Bin-crate layout renders `Outlet` BEFORE
  `Sidenav11Sidebar` inside the wrapper. content = `SidenavGroup >
  SidenavGroupLabel{title} > SidenavGroupContent > SidenavMenu` > links.
  MobileSheet: `SheetContent direction=Right`, trigger `PanelLeft
  class="rotate-180 size-4"`.

`blocks/mod.rs`: no new modules (all `sidenavNN` already declared). Remove
`sidenav_routes_simplified` if deleted.

---

## 9. Hand-edit `src/__registry__/all_blocks.rs`

Only one required change:

```rust
// BlockIdKebab::to_full_view_url
pub fn to_full_view_url(&self) -> String {
    match self {
        Self::Sidenav01 | Self::Sidenav02 | Self::Sidenav03 | Self::Sidenav04
        | Self::Sidenav05 | Self::Sidenav06 | Self::Sidenav07 | Self::Sidenav08
        | Self::Sidenav09 | Self::Sidenav10 | Self::Sidenav11 => {
            // leptos parity: /view/{no-dash}/docs/components
            format!("/view/{}/docs/components", SidenavRoutes::from(*self).as_ref())
        }
        _ => format!("/view/block/{}", self),
    }
}
```

Simplest concrete form (no new `From` impl): map inline.

```rust
Self::Sidenav01 => "/view/sidenav01/docs/components".to_string(),
// ... 02..11
```

Optional: change `to_component()` sidenav arms to `rsx! {}` (only if
`SidenavNN()` is dropped in section 8; otherwise leave untouched).

Leave `files()`, `file_tree()`, `to_md()`, `to_title()`, `ALL_SIDENAV_BLOCKS`
as-is. `block_id_str` stays `"sidenav-01"` (dash) for the anchor id / code
panel; only the iframe URL changes.

---

## 10. Dioxus limitations (the "documente le" deliverable)

1. `#[component(transparent)] -> impl MatchNestedRoutes` / `.into_inner()` /
   `StaticSegment` / `WildcardSegment`: no Dioxus equivalent. The 11
   `SidenavNNRoutes` + `SidenavRoutesSimplified` route contributors are
   replaced by hand-written `#[layout]` + `#[route]` entries in `src/main.rs`
   plus `SidenavDemoLayout` / `SidenavInsetRightLayout`.
2. Dioxus `#[route]` cannot parametrize a static path segment, and the
   `registry` crate cannot add `Route` variants. Leptos keys 11 subtrees on
   `StaticSegment(SidenavRoutes::SidenavNN.as_ref())`; the port uses one
   `:sidenav` dynamic segment + `match SidenavRoutes::from_path(path)`.
   User-approved.
3. `Memo<DocsRoutes>` as a component prop type: replaced by plain `DocsRoutes` /
   `SidenavRoutes` values recomputed by the layout on every navigation (the
   layout calls `use_route::<Route>()` so it re-renders on nav). Behaviour
   matches; the type signature differs.
4. `use_breadcrumb_from_segment` in dioxus returns a plain `Vec` computed from
   `web_sys` at call time (non-reactive), unlike leptos which returns a
   reactive `Memo`. Port reimplements it as a pure `breadcrumb_from_path(path,
   "docs")` over the router-provided `path` prop, so the breadcrumb still
   updates on client navigation.
5. `SidenavLink` active state in dioxus is read from `window.location` via
   `web_sys` at render (non-reactive; empty string on the server). Leptos
   resolves it reactively. On SPA navigation within the iframe the link list is
   re-rendered by the layout, so the highlight still follows the URL; a
   same-component in-place update without re-render would not. Acceptable,
   documented.
6. `SidenavWrapper` hardcodes `--sidenav-width: 256px` and exposes no `style`
   prop. Fix applied: add `#[props(into, optional)] style`. 16rem == 256px so
   the value is a no-op for sidenav01-08/10/11; sidenav09 genuinely needs the
   `--sidenav-width-icon` override.
7. `SidenavInput` has no `value` / `oninput`. Fix applied: add both props so
   sidenav10's reactive search works. Without the fix sidenav10 search would be
   inert.
8. `SheetContext` / `target_id` are private in `ui/sheet.rs`. Fix applied: make
   them `pub` so `SidenavNNContent` can read the real sheet id for the
   `data-sheet-close` wrapper, matching leptos. (Dioxus Sheet close JS only
   checks the attribute's presence, so even without the id the wrapper closes
   the sheet; the fix is for parity, not function.)
9. `SidenavInset` has no `data-variant` passthrough;
   `SidenavGroup`/`SidenavGroupContent` have no `data-sidenav` passthrough.
   Fix: add small optional props (`data_variant`, `data_sidenav`). If that
   cascades into many primitives, document as cosmetic (affects search-form
   styling hooks only) and skip.
10. Leptos `SidenavInsetRight` is the `view` of a `ParentRoute` yet renders no
    `<Outlet/>`, so its declared child routes never display. The port keeps
    that behaviour: the leaf pages (`SidenavDemoComponents` etc.) render
    `rsx!{}`. The `Outlet::<Route>{}` required by the Dioxus router lives in
    `SidenavInsetRightLayout` after `<SidenavInsetRight/>` and renders nothing
    visible.
11. `all_blocks.rs` is generated by the private
    `rust_ui_internals/build_registry_dioxus`. Per user, hand-edit for now; the
    generator should later be taught the sidenav `to_full_view_url` form. Note
    left in this file, section 9.
12. `#[redirect(...)]` exists in Dioxus but is not needed here (leptos uses no
    redirect for the sidenav routes; entry URL is always `.../docs/components`).

---

## 11. Execution order

1. `app_crates/registry/Cargo.toml`: add `heck = "0.5"`.
2. `app_crates/registry/src/blocks/sidenav_routes.rs`: rewrite (section 4).
   `cargo check -p registry`.
3. `app_crates/registry/src/ui/sidenav.rs`, `ui/sheet.rs`: primitive prop
   additions (section 3). `cargo check -p registry`.
4. `app_crates/registry/src/blocks/sidenav_routes_selector.rs`: rewrite
   (section 5).
5. `app_crates/registry/src/blocks/sidenav_inset_right.rs`: rewrite (section 7)
   + `breadcrumb_from_path` helper.
6. `app_crates/registry/src/blocks/sidenav01.rs` .. `sidenav11.rs`: rewrite
   (section 8). `cargo check -p registry` after each 2-3 files.
7. `app_crates/registry/src/blocks/sidenav_routes_simplified.rs`: delete +
   update `blocks/mod.rs` (section 6).
8. `src/domain/blocks/routing/sidenav_demo_layout.rs` +
   `sidenav_demo_pages.rs` + `mod.rs` (section 2).
9. `src/main.rs`: route enum + imports (section 1).
10. `src/__registry__/all_blocks.rs`: hand-edit `to_full_view_url` (section 9).
11. `CHANGELOG_DEV.md`: top entry `## 2026-09-10`.
12. `dx fmt` (see `justfile`: dev recipe is `pnpm run css & dx serve`; format
    is `dx fmt`). Then `cargo check` (workspace) per `dioxus-ui/CLAUDE.md`.

## 12. Open questions to resolve during impl (not blockers)

- Does Dioxus require `Outlet::<Route>{}` to be within the layout component's
  own returned subtree? If yes, `SidenavInsetRight` must move to the bin crate
  (needs `Route`) or `SidenavInsetRightLayout` must inline the inset markup.
  Plan B: inline a thin wrapper in the bin crate that calls the registry
  `SidenavInsetRight` for markup and appends `Outlet`.
- `use_route::<Route>().to_string()`: confirm `Routable`'s `Display` yields the
  URL path (expected) so `SidenavRoutes::from_path` and `breadcrumb_from_path`
  get a real `/view/sidenavNN/docs/...` string.
- Confirm no non-sidenav file imports the `ComponentsRoutes` variants being
  removed (Card, Checkbox, Dialog, ...). If any does, keep the full variant
  list.
- `try_consume_context` vs `use_context` for `SheetContext` inside
  `SidenavNNContent` (must not panic on the desktop render where no Sheet is an
  ancestor): use `try_consume_context`.
