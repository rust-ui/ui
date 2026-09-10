# Dev Changelog

Internal changelog for the dioxus-ui site (not user-facing).

## 2026-09-10

### Improvements

- **Verbatim sidenav routing port**: Ported the Leptos sidenav demo family to
  Dioxus with nested `/view/:sidenav/docs/...` routes, responsive sheets,
  section switching, search filtering, breadcrumb updates, and direct sidenav
  block URLs. Added the required primitive props and documented the single
  dynamic-route deviation in `PLAN_SIDENAV_BLOCKS_VERBATIM_PORT.md`.

- **Home header parity**: Mirrored the leptos home header. Brand text is now
  `Rust/UI` (was `Dioxus/UI`), a `LeptosLink` cross-link (small brand-coloured
  Leptos mark linking to `https://rust-ui.com`, new
  `public/images/logos/leptos.svg`) sits next to the GitHub stars in both the
  desktop right nav and the mobile menu, and the mobile menu now opens with the
  `DemoAccordionIcons` demo like leptos. `src/components/leptos_link.rs`,
  `src/components/navigation/header_home.rs`, `src/components/mod.rs`

- **CSS parity fixes (ui primitives)**: Realigned five `ui/*.rs` class strings
  that had drifted from the leptos site.
  - `toggle_group.rs`: `ToggleGroupItem` base regained
    `aria-invalid:ring-destructive/20 aria-invalid:border-destructive shrink-0
    dark:aria-invalid:ring-destructive/40` (leptos carries it between
    `transition-[color,box-shadow]` and `hover:bg-muted`).
  - `accordion.rs`: `AccordionTrigger` chevron dropped the extra `size-4
    shrink-0`; leptos renders `<ChevronDown class="transition-all
    duration-300" />` and the parent label already sizes the svg via
    `[&_svg:not([class*='size-'])]:size-4`.
  - `input_otp.rs`: hidden proxy `<input>` uses `hidden` (was `sr-only`) to
    match leptos.
  - `item.rs`: `ItemSeparator` now emits the full `Separator` class string
    (`shrink-0 bg-border w-full h-[1px] my-0`) instead of a bare `<div>` with a
    doubled `my-0`.
  - `form.rs`: `FormInput` now renders the complete leptos `Input` base string
    (`text-foreground`, `file:*`, `selection:*`, `dark:bg-input/30`,
    `md:text-sm`, `aria-invalid:*`, `read-only:bg-muted`) and wires
    `aria-invalid` reactively from the field touched / error state, like the
    leptos `FormInput` which delegates to `<Input />`.
  - `multi_select.rs`: `MultiSelectLabel` is a `<span>` with
    `px-2 py-1.5 text-sm font-medium data-inset:pl-8 mb-1` (was a `<li>` that
    added `text-muted-foreground` and dropped `data-inset:pl-8`), matching the
    leptos re-export of `select::SelectLabel`.

## 2026-09-09

### Improvements

- **Bug report system**: Ported the leptos bug report cluster. Client-side
  panics and framework / browser `console.warn` output are now captured on the
  WASM client (`src/utils/client_diagnostic_handler.rs`, wired in `App()`),
  forwarded through a `report_client_bug` server fn to local SQLite plus the
  shared RUSTIFY endpoint (`RUSTIFY_API_URL` + `BUG_REPORTS_API_KEY`), and 404s
  are reported server-side from `PageNotFound`. The SQLite file, schema and wire
  format match the leptos site so both apps share one database. A new admin page
  at `/bug-reports/d7f3a9c2e1b5` lists reports grouped by similarity hash, with
  per-group and delete-all actions, expandable stack traces and a parsed
  user-agent summary. `src/domain/bug_report/{mod,bug_reports,bug_reports_sqlite,
  page_bug_reports}.rs`, `src/utils/client_diagnostic_handler.rs`,
  `src/routes/page_not_found.rs`, `src/main.rs`, `Cargo.toml`

- **CSS parity sweep**: Audited every shared `ui/*.rs` primitive and both
  `tailwind.css` files against the leptos site and realigned the class strings
  that had drifted.
  - `button.rs`: `ButtonSize::Sm` carried a hardcoded `text-xs` (12px); leptos
    inherits the base `text-sm` (14px), which made the `ui add <demo>` label and
    every other `size=Sm` button render visibly smaller. Sizes, variants and the
    base string now match leptos exactly (Sm gets `gap-1.5 has-[>svg]:px-2.5`,
    Default/Lg get `has-[>svg]` padding, `shadow-xs` removed from Warning/Success,
    `dark:` classes added to Destructive/Outline/Ghost, base gains
    `aria-invalid:*`, `active:scale-[0.98]`, `touch-manipulation`, tap-highlight
    resets).
  - `label.rs`: `peer-disabled:opacity-70` -> `opacity-50`; added
    `flex items-center gap-2` and `group-data-[disabled=true]:*`.
  - `input.rs`: restored `text-foreground`, `file:*`, `selection:*`,
    `dark:bg-input/30`, `md:text-sm`, `aria-invalid:*`, `read-only:bg-muted`.
  - `textarea.rs`: replaced the old shadcn classes (`min-h-[80px]`,
    `bg-background`, `ring-offset-*`, `focus-visible:ring-ring`, `text-sm`) with
    the leptos v4 set (`min-h-16`, `field-sizing-content`, `dark:bg-input/30`,
    `focus-visible:border-ring focus-visible:ring-ring/50`, `aria-invalid:*`,
    `shadow-xs`, `text-base md:text-sm`, `transition-[color,box-shadow]`).
  - `card.rs`: `CardHeader` regained `flex flex-col items-start`, the `sm:` grid
    prefixes and `[[data-size=sm]_&]:px-4`; `CardContent` / `CardFooter` regained
    the `data-size` padding; `CardAction` regained the `sm:` prefixes; added the
    `CardSize` prop + `data-size` attribute and the `CardList` / `CardItem`
    components.
  - `alert.rs`: base regained `[&>svg]:text-foreground`; `AlertTitle` now renders
    `<h4>` and `AlertDescription` `<p>` (were `<div>`).
  - `breadcrumb.rs`: `BreadcrumbList` uses `gap-1 ... break-words`;
    `BreadcrumbSeparator` matches the leptos `[&>svg]:size-3.5` wrapper instead of
    a `text-muted-foreground/50` tint.
  - `button_group.rs`: `ButtonGroup` regained the `[&>input]:flex-1`,
    `[&>[data-slot=select-trigger]...]` and `has-[>[data-slot=button-group]]:gap-2`
    selectors; added the `ButtonGroupText` component.
  - `pagination.rs`: the active page now uses `bg-primary text-primary-foreground
    hover:bg-primary/90` (was `border bg-background shadow-sm`).
  - `field.rs`: `FieldDescription` regained `nth-last-2:-mt-1`; `FieldLabel`
    regained `[&>*]:data-[name=Field]:p-4` and `opacity-70` -> `opacity-50`.
  - `tailwind.css`: restored the missing "Chat interface utilities" block
    (`wrap-break-word`, `scroll-fade-b` / `-x`, `scrollbar-thin` /
    `-gutter-stable` / `-none`, the `--shimmer-angle` `@property`, the
    `tw-shimmer` keyframes and the `shimmer` / `shimmer-once` / `shimmer-reverse`
    / `shimmer-none` utilities).

  `app_crates/registry/src/ui/{button,label,input,textarea,card,alert,breadcrumb,button_group,pagination,field}.rs`,
  `tailwind.css`

- **CSS parity sweep (wave 2)**: Second pass over the remaining shared
  `ui/*.rs` primitives, class strings copied verbatim from the leptos source.
  - `kbd.rs`: `Kbd` regained the `[[data-slot=tooltip-content]_&]:*` and
    `dark:[[data-slot=tooltip-content]_&]:*` classes.
  - `dialog.rs`: `DialogContent` base matches leptos (`sm:max-w-[425px]` and
    `overflow-y-auto` dropped, `top-1/2 left-1/2 -translate-*` -> `top-[50%]
    left-[50%] translate-*-[-50%]`); added the close `X` button + `show_close_button`
    prop and `data-target`.
  - `input_group.rs`: `InputGroupAddon` regained
    `[&>kbd]:rounded-[calc(var(--radius)-5px)]`; `BlockStart` / `BlockEnd` aligns
    regained the `[.border-b]:pb-3` / `[.border-t]:pt-3` and
    `group-has-[>input]/input-group:*` classes.
  - `dropdown_menu.rs`: `DropdownMenuLabel` regained `data-inset:pl-8`.
  - `chips.rs`: `ChipsContainer` regained the `chips__main__container` marker and
    the bounce-easing transitions on the root and `*` / `*:before` / `*:after`.
  - `stepper.rs`: `StepperTitle` / `StepperDescription` / `StepperSeparator`
    switched from Rust runtime state conditionals to the leptos
    `group-data-[state=*]/stepper-item:*` CSS variants.
  - `sonner.rs`: `SonnerTrigger` regained the 6-way toast variant colour sets and
    the leptos base string.
  - `table.rs`: `<table>` is now bare (inline wrapper `<div>` removed);
    `TableHeader` gains `sticky top-0 z-10 bg-card`; `TableHead` / `TableCell`
    regained the `[&:has([role=checkbox])]:*` selectors; `TableRow` class order
    matches leptos.

  `app_crates/registry/src/ui/{kbd,dialog,input_group,dropdown_menu,chips,stepper,sonner,table}.rs`

- **CSS parity sweep (wave 3)**: Third pass over the overlay primitives that
  had been left as "architectural divergence". The vanilla-JS open/close
  wiring (`data-state` attributes) stays, only the class strings and rendered
  markup were realigned verbatim to leptos.
  - `popover.rs`: `PopoverContent` base copied from leptos (`bg-card`,
    `overflow-visible relative`, `my-[1ch] w-[250px] min-h-[150px]`) with the
    dioxus `data-[state=open]:*` animation classes kept; `PopoverTitle` renders
    `<h3>` with `mb-3` (was `<p>`); `PopoverDescription` drops `mt-1` and
    reorders to `text-muted-foreground text-sm`.
  - `select.rs`: `SelectLabel` regained `data-inset:pl-8`; `Select` root
    `w-full` -> `w-fit`; `SelectTrigger` `px-3` -> `p-2` and regained the
    `focus-visible:*`, `disabled:*` and `[&_svg:not(:last-child)]:mr-2` /
    `:not(:first-child)]:ml-2` classes, trigger chevron drops `shrink-0 ml-2`;
    `SelectOption` regained `no-underline`; `SelectContent` regained
    `w-[150px]` and the full `data-[position=Above]:*` / origin variant set,
    with a new `SelectPosition` prop + `data-position` attribute and an
    above/below detection port in the inline script (leptos `updatePosition`),
    pointer-events now toggled via inline style like leptos.
  - `sheet.rs`: `SheetContent` base -> leptos (`bg-card`, `overflow-y-auto
    overscroll-y-contain`, no `flex flex-col` / `ease-in-out`), per-direction
    sizing -> `w-[400px]` / `h-[400px]` with no border; renders the close `X`
    button gated by the existing `show_close_button` prop; `SheetHeader`
    `gap-0.5 p-4`, `SheetTitle` `font-bold text-2xl`, `SheetDescription` drops
    `text-sm`, `SheetBody` `flex flex-col gap-4`, `SheetFooter`
    `mt-auto flex flex-col gap-2 p-4`.
  - `drawer.rs`: `DrawerTitle` renders `<h3>` (was `<h2>`); `DrawerBody` drops
    `py-4 w-full`; `DrawerFooter` drops `mx-auto w-full max-w-[500px]`;
    `DrawerHandle` `mb-6` -> `mb-8`, regained `active:opacity-100`, bg
    `bg-muted-foreground/30` -> `bg-[#e2e2e4]`. `DrawerContent` base + the
    vaul-vs-bespoke position enums stay divergent (different animation engine).

  `app_crates/registry/src/ui/{popover,select,sheet,drawer}.rs`

- **Download page**: Ported the leptos `/download` page (`routes/page_download.rs`,
  route wired in `main.rs`). Lists the Rust UI Desktop (Tauri) builds for macOS,
  Linux and Windows with per-platform download buttons (real `<a download>` to the
  GitHub `rust-ui/releases` latest release) and a requirements grid. The home hero
  "Download Desktop" button already pointed at `/download`; it 404'd until now.
  `src/routes/page_download.rs`, `src/routes/mod.rs`, `src/main.rs`

- **Component doc metadata**: Synced the TOML frontmatter of every
  `public/docs/components/*.md` to match the leptos site exactly (same keys,
  same values): `title`, `description`, `tags`, `is_new`, `image`,
  `image_dark`. 75 files updated. Many cards on `/docs/components` had an empty
  `description` or `tags`, ad-hoc dioxus-only wording, or a missing
  `image` / `image_dark` (falling back to nothing or `_placeholder.webp`);
  they now carry the same copy and thumbnails as leptos. Labels also
  normalise (`Auto Form` -> `AutoForm`, `Multi Select` -> `MultiSelect`,
  `Drag And Drop` -> `Drag and Drop`). 3 dioxus-only docs with no leptos
  counterpart (`image`, `mask`, `toolbar`) left untouched. Doc bodies are
  unchanged. Regenerated `src/__registry__/{demos_sidenav,static_md_registry,
  my_command_bar_constants}.rs` and `public/documentation_map.md`.
  `public/docs/components/*.md`

- **Page transitions**: Ported the leptos `page_transition` util.
  `src/utils/page_transition.rs` adds `retrigger_page_fade()` (replays the
  `page__fade` intro on `#page__outlet` on every route change) and a
  `ScrollToTop` component (resets `#data-scroll-target` scroll on nav, skips
  `/blocks/*`). Wired into `AppLayout` plus the docs / blocks / charts /
  workflows layouts; the `@keyframes page__fade_in` / `.page__fade` rule now
  ships from `main.rs`. Before, the fade played only on the first hard load and
  pages kept their previous scroll position after client-side nav.
  `src/utils/`, `src/main.rs`, `src/routes/app_layout.rs`,
  `src/routes/docs_layout.rs`,
  `src/domain/{blocks,charts,workflows}/routing/*_layout.rs`, `Cargo.toml`

- **Animate demo**: Brought `/docs/components/animate` to parity with the leptos
  site. `AnimateHoverVariant` gained the 8 variants leptos carries but dioxus was
  missing (`BounceCustom`, `FadeOutDownV2`, `FlashV0`, `JiggleV0`, `PulseCustom`,
  `RubberBandV0`, `ShakeV0`, `SwingV0`) plus a `strum::Display` derive, and the
  demo now renders the full `HOVER_ANIMATIONS` list (85 swatches) in the same
  `Grid3` of `Card` + `Animate` + `AnimatedChildren` + `CardDescription` layout
  as leptos, with a heading that shows the count. Known gap, shared with leptos:
  Tailwind v4 does not emit `hover:animate-<Name>` for names ending in `V0`/`V2`,
  so those 6 swatches are present but inert until the keyframes are renamed.
  `app_crates/registry/src/ui/animate.rs`,
  `app_crates/registry/src/demos/demo_animate.rs`
- **DemoWrapper**: Brought the demo block to parity with the leptos site. The
  Preview/Code tab row now carries a right-side action column: a `ui add <demo>`
  outline button that copies the CLI command, and a kebab menu with "Copy Demo"
  (copies the source) and "View as Markdown". Code panel markup now matches
  leptos `SyntectHighlighterCode` (same wrapper, classes, no floating copy
  button); source is path-rewritten (`use crate::registry::` -> `use
  crate::components::`) before display.
  `src/domain/markdown_ui/components/static_demo_wrapper.rs`
- **Syntax highlighting**: syntect now runs on the client (WASM) too, not just
  SSR. Code kept its colours after hydration before only on server-rendered
  pages; client-rendered code fell back to plain escaped text. `syntect` /
  `html-escape` are now unconditional deps and `highlight_code` mirrors the
  leptos `_markdown_crate` shape (shared impl + per-target `SyntaxSet` /
  `ThemeSet` caches). Grows the WASM bundle (embedded syntax + theme dumps).
  `src/markdown/highlight_code.rs`, `src/markdown/mod.rs`, `Cargo.toml`
- **DocHeader**: Aligned spacing/layout classes with the leptos header
  (`mt-2` gaps restored above description and tags, `min-w-0` + `truncate` on the
  title, `z-20` / `shrink-0` on the action cluster). The tags row is now always
  rendered, matching leptos: it doubles as the `mb-6` spacer below the header, so
  components with no tags (Avatar, etc.) keep the same gap before the demo block
  instead of having it collapse. `src/components/doc_header.rs`

- **Components index**: Wired preview thumbnails (light + dark) into the doc
  frontmatter for 20 components that already had screenshots on disk in
  `public/images/thumbnails/`: Accordion, Alert, Badge, Breadcrumb, Button,
  Checkbox, Date Picker, Dialog, Dropdown Menu, Dropzone, Input, Pagination,
  Popover, Select, Switch, Table, Tabs, Textarea, Toast, Tooltip. Cards on
  `/docs/components` previously showed the `_placeholder.webp` art or nothing
  because the `image` / `image_dark` frontmatter keys were missing.
  `public/docs/components/*.md`, regenerated `src/__registry__/demos_sidenav.rs`

### Bug Fixes

- **DemoWrapper**: Reset the Preview/Code tab to Preview when navigating between
  component docs. Client-side nav reuses the component instance, so the `tab`
  signal survived route changes and stayed stuck on Code. Fixed by comparing the
  previous `demo_name` during render (no `use_effect`).
  `src/components/demo_wrapper.rs`

- **Table of contents**: "On This Page" showed the headings of the first doc
  visited instead of the current one, and briefly listed every entry twice
  during navigation. `ComponentPage` / `HookPage` are reused across client-side
  nav and pushed the TOC into a context signal owned by `DocsLayout`; the
  `use_effect` doing it had no reactive dependency on `name` (ran once, never
  updated), and writing a parent signal during child render also corrupted the
  diff and duplicated the list. Removed the context signal entirely: `DocsLayout`
  now derives the TOC directly from `use_route()`, which is reactive, so it
  tracks the current page with no effect and no cross-component write.
  `src/routes/docs_layout.rs`, `src/routes/component_page.rs`,
  `src/routes/hook_page.rs`
