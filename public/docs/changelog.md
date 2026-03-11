+++
title = "Changelog"
description = "Latest updates and announcements."
tags = []
is_new = false
order = 99
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++


## March 2026 - DataTable Faceted Filters

### New Demos

- **[Data Table](/docs/components/data-table)**: New "With Filters" demo — Status faceted filter using a Popover with per-value count badges, multi-select checkboxes, email text search, and a combined Reset button.

---

## March 2026 - Drawer + Dialog Responsive Pattern

### New Demos

- **[Drawer](/docs/components/drawer)**: New "Responsive Dialog" demo — single trigger that opens a Drawer on mobile and a Dialog on desktop using the `use_media_query` hook. Same content, zero duplication.

---

## March 2026 - DatePicker Time & Booked Dates

### New Demos

- **[Date Picker](/docs/components/date-picker)**: New **"With Time Picker"** demo — combines a date calendar with start/end time inputs (`InputGroup` + `Clock2` icon) in the card footer for full datetime selection.
- **[Date Picker](/docs/components/date-picker)**: New **"Booked Dates"** demo — marks a range of unavailable dates as struck-through and non-selectable, ideal for booking flows where certain dates are already taken.

---

## March 2026 - Badge Colors & Dropdown Destructive

### New Variants

- **[Badge](/docs/components/badge)**: New `Success`, `Warning`, and `Info` semantic color variants — soft background colors using the design system's color tokens (`--success-light`, `--warning-light`, `--info-light`) with proper dark mode support.
- **[Dropdown Menu](/docs/components/dropdown-menu)**: New `DropdownMenuActionVariant::Destructive` — renders delete/remove actions in red (`text-destructive`) to signal irreversible actions.

---

## March 2026 - DatePicker Dropdown Caption

### New Demos

- **[Date Picker](/docs/components/date-picker)**: New "Dropdown Caption" demo — month and year `<select>` dropdowns in the calendar header instead of prev/next arrows, for fast navigation to any month/year (e.g., birthdate pickers).

---

## March 2026 - DirectionProvider Component

### New Components

- **[Direction Provider](/docs/components/direction-provider)**: New `DirectionProvider` component — wraps children in a `dir` attribute container for LTR/RTL layout support. Tailwind's `rtl:` variants activate automatically inside the provider. Includes demos with an Arabic (RTL) login card.

---

## March 2026 - AlertDialog Media & Small Variants

### New Demos

- **[Alert Dialog](/docs/components/alert-dialog)**: Three new composition demos — `With Media` (icon above content), `Small` (compact narrow dialog with side-by-side buttons), and `Small with Media` (compact + centered icon). Pure composition using existing components, no new API.

---

## March 2026 - File Upload List Demo

### New Demos

- **[Item](/docs/components/item)**: New "File Upload List" demo — compact `ItemSize::Xs` items with file icon, `Progress` bar, and time-remaining label. Also adds `Xs` size variant to `Item`.

---

## March 2026 - Collapsible Advanced Demos

### New Demos

- **[Collapsible](/docs/components/collapsible)**: New "File Tree" demo — recursive nested collapsibles with `ChevronRight` rotation for folder expand/collapse. New "Settings" demo — corner radius inputs using `grid-cols-subgrid` with `outer_class` on `CollapsibleContent` for precise grid participation.

---

## March 2026 - HoverCard Component

### New Components

- **[Hover Card](/docs/components/hover-card)**: New `HoverCard` component — hover-triggered floating card for rich contextual previews (user profiles, link previews). Uses CSS anchor positioning with `HoverCard`, `HoverCardTrigger`, and `HoverCardContent`. Supports `Top`, `Bottom`, `Left`, `Right` sides.

---

## March 2026 - Collapsible Component

### New Components

- **[Collapsible](/docs/components/collapsible)**: New `Collapsible`, `CollapsibleTrigger`, and `CollapsibleContent` components — animated show/hide panel with CSS grid height transition. Supports controlled state via `RwSignal<bool>`.

---

## March 2026 - DatePicker Quick Presets

### New Demos

- **[Date Picker](/docs/components/date-picker)**: New "Quick Presets" demo — preset buttons for one-click date selection with automatic calendar month navigation.

---

## March 2026 - DatePicker Week Numbers

### New Demos

- **[Date Picker](/docs/components/date-picker)**: New "Week Numbers" demo — shows ISO week numbers in a leftmost column using `DatePickerWeekNumberHeader` and `DatePickerWeekNumberCell`.

---

## March 2026 - Alert Color Variants & Accordion in Card

### New Demos

- **[Alert](/docs/components/alert)**: New "Alert with Custom Colors" demo showing semantic color overrides (amber/warning) via Tailwind class props — no variant needed.
- **[Accordion](/docs/components/accordion)**: New "Accordion in Card (FAQ)" demo — Accordion nested inside a Card for FAQ and settings sections.

---

## March 2026 - ChoiceCard Patterns

### New Demos

- **[Field](/docs/components/field)**: Two new composition patterns — `SwitchChoiceCard` (settings toggle with highlighted card border) and `RadioChoiceCard` (plan/tier selector where the full row is clickable). No new components — pure `FieldLabel` + `Field` + `FieldContent` composition.

---

## March 2026 - Aspect Ratio Component

### New Components

- **[Aspect Ratio](/docs/components/aspect_ratio)**: New `AspectRatio` component — a simple wrapper that constrains content to a given width/height ratio. Supports any numeric ratio (16:9, 1:1, 9:16, etc.) with three demos: landscape, square, and portrait.

---

## March 2026 - Slider Variants

### Component Improvements

- **[Slider](/docs/components/slider)**: Three new variants — `Vertical` (rotated, Round and Flat), `Multiple` (3 independent sliders with labeled values), and `Controlled` (reactive signal binding with live value display).

---

## March 2026 - Theme Customizer, New Components & Enhancements

### New Page

- **[Theme Customizer](/create)**: Live theme builder — pick base color, radius, and preview your theme across real components. Shareable presets via compact URL encoding (`?preset=...`). Full color set: Neutral, Stone, Zinc, Mauve, Olive, Mist, Taupe with exact OKLCH values.

### New Components

- **[Callout](/docs/components/callout)**: Styled alert block for documentation pages, with `Default`, `Info`, and `Warning` variants, optional title, and a `--surface` design token for the background.
- **[Toggle Group](/docs/components/toggle-group)**: Flexible set of toggle buttons for grouping related options, with `variant`, `orientation`, and `spacing` props.
- **[Progress](/docs/components/progress)**: Simple, accessible progress bar for displaying task completion.

### Component Improvements

- **[Avatar](/docs/components/avatar)**: `size` prop (`Sm` / `Default` / `Lg`), `AvatarBadge` (status dot or icon), `AvatarGroup` (stacked overlapping), and `AvatarGroupCount` (overflow counter).
- **[Tabs](/docs/components/tabs)**: `variant="Line"` (underline indicator) and `orientation="Vertical"` layout.
- **[ButtonGroup](/docs/components/button-group)**: New demos — sizes, button+select, and button+input combos.
- **[InputGroup](/docs/components/input-group)**: `BlockStart` / `BlockEnd` alignment on `InputGroupAddon` for vertical layouts — ideal for AI prompt inputs and rich text editors.
- **[Empty](/docs/components/empty)**: Five new variants — Background Fill, Inside a Card, Outline (dropzone), Avatar Group, and With Input Group.
- **[Item](/docs/components/item)**: `ItemGroup` list container, `ItemHeader` / `ItemFooter` full-width rows, `ItemSeparator`, and `ItemMedia variant="Image"` thumbnail — with new demos.
- **[Dialog](/docs/components/dialog)**: Sticky footer demo — scrollable body with pinned footer using `-mx-6 px-6` bleed pattern.


------------------------------------------------
------------------------------------------------


## February 2026 - AutoForm, DataGrid & CLI

### New Components

- **AutoForm**: Derive macro for automatic form generation with on-blur validation
- **DataGrid**: Full-featured data grid component
- **Shimmer**: Auto-adapting skeleton loader

### Component Improvements

- **Input** + **Textarea**: `bind_value` support for controlled inputs
- **Sonner**: Position customization (top/bottom)

### ui-cli 0.3.8

- `ui update` — check installed components against the registry
- `ui search` — filter registry by name
- `ui list --json` — machine-readable output
- `ui docs` — open documentation in browser
- `ui add --dry-run` — preview changes before installing

### tw_merge 0.1.17

- Container query variant support

### Download

- Linux and Windows added to the download page


------------------------------------------------
------------------------------------------------


## January 2026 - iOS & Tauri Native Experience

First-class iOS support lands in Rust UI. Native loading screens, proper safe area handling, and a dedicated bottom navigation bring a polished mobile experience. Tauri builds are now optimized with `trim-paths` for significantly smaller binaries.

### Highlights

- Native iOS loading screen with safe area support
- Bottom navigation designed for mobile
- Tauri binary size reduction with trim-paths optimization
- Desktop releases v0.1.6 → v0.1.9
- iOS releases v0.1.21 → v0.1.24

### tw_merge v0.1.16

Major improvements to the Tailwind merge utility, now supporting modern Tailwind features.

- `tailwindcss-animate` classes support
- CSS logical inset properties (`inset-inline-start/end`, `inset-block-start/end`)
- Child selector variant (`*:`) for Tailwind 3.4+
- Arbitrary duration/delay values

### Component Improvements

- **Command**: `should_filter` prop for server-side search, `on_search_change` callback
- **Pagination**: Scroll option for navigation control
- **Bug Reports**: Deduplication and grouping for similar bugs
- **Signals**: `try_set`/`try_get` to prevent Wasm panic on disposed signals

### Developer Experience

- New `justfile` for cross-platform dev commands
- TUI scrollbar component for Ratatui CLI


------------------------------------------------
------------------------------------------------


## December 2025 - CLI 2.0 & Interactive Components

The CLI gets a complete rewrite with Ratatui, bringing a modern terminal UI with tabs, scrolling, mouse support, and keyboard shortcuts. New interactive components make building rich user experiences easier.

### Highlights

- **ui-cli 2.0**: Built with Ratatui featuring tabs, scrolling, mouse support, and help popup
- **Context Menu**: Right-click menus with full keyboard navigation
- **ButtonAction**: Press-and-hold interaction pattern
- **Chat Blocks**: Ready-to-use chat interface components

### New Hooks & Components

- `use_press_hold` hook for long-press interactions
- Select scroll indicators for better UX
- Combobox improvements
- Mobile responsive Sheet for all sidenavs


------------------------------------------------
------------------------------------------------


## November 2025 - Mobile First & Charts

A major shift towards mobile-first design. Extensions are consolidated into Core components, with select pieces moving to Blocks. The UI now works seamlessly with Tauri for native mobile apps.

### Highlights

- Mobile-first approach, optimized for Tauri
- Removed Extensions in favor of Core components
- Newsletter components
- Charts integration


------------------------------------------------
------------------------------------------------


## October 2025 - Forms & ScrollArea

A major focus on form handling and scrollable content. The new `use_form` hook brings validation to Leptos forms, while ScrollArea adds smooth scrolling with snap support.

### Highlights

- **Form Validation**: New `use_form` hook with field validation
- **ScrollArea**: Complete scroll component with horizontal, vertical, and snap modes
- **Pagination**: URL query params support for shareable pagination state
- **Icons v0.15**: Animated icons and performance optimizations

### Component Improvements

- **DropdownMenu**: Flexible positioning with `position` prop
- **Switch**: Added `checked` prop for controlled state
- **Dialog**: Scroll lock improvements and scrollable content support
- **Select/MultiSelect**: Better dropdown positioning and scroll behavior

### Developer Experience

- Centralized scroll-lock utility for consistent behavior
- TableOfContents moved to app components for easier customization
- Standardized z-index values across all components


------------------------------------------------
------------------------------------------------


## September 2025 - Headers & Navigation

New header blocks with smooth sliding animations and improved navigation components. The Select component gets a complete overhaul with better UX.

### Highlights

- **Header01 Block**: Sliding mega-menu with scroll detection
- **NavigationMenu**: Full navigation system with triggers and content panels
- **Accordion**: Smooth open/close animations
- **Select**: Complete refactor with SelectValue and placeholder support

### leptos_ui v0.2 Breaking Changes

We refactored the `leptos_ui` crate with breaking changes.

**New macro system:**
- `clx!`: Elements with children
- `void!`: Self-closing elements (no children)

> See [MDN Docs](https://developer.mozilla.org/en-US/docs/Glossary/Void_element) for reference about void elements.

**Attribute changes:** Removed direct props from macros. Use `attr:*` pattern instead:
```rust
// Define component
void! {MyImage, img, "rounded-lg border"}

// Before
<MyImage src="image.jpg" />

// After
<MyImage attr:src="image.jpg" />
```

### New Blocks

New [Sidenav Blocks](https://www.rust-ui.com/blocks/sidenav) are now accessible in the Blocks Registry!

<img src="/images/changelog/changelog-2025-10.png" alt="Sidenav01 in the Blocks Registry." />


------------------------------------------------
------------------------------------------------


## August 2025 - BlockViewer & AI Integration

The BlockViewer component brings an interactive preview experience for blocks. AI-friendly documentation makes integration with LLM agents seamless.

### Highlights

- **BlockViewer**: Resizable iframe preview with responsive controls
- **Parallax Blocks**: Three parallax scroll demos (scroll, iPad, zoom words)
- **Login Blocks**: Ready-to-use authentication pages
- **Link Component**: Type-safe path matching with `PathMatchType` enum

### Documentation for LLMs

We've added better documentation, accessible by any LLM agents:
- [Documentation](https://www.rust-ui.com/documentation.md): The full documentation of the Rust/UI Registry
- [Index](https://www.rust-ui.com/registry/index.md): Simple list of the Registry for token optimization
- [Tree](https://www.rust-ui.com/registry/tree.md): Tree structure of the Registry, used by our [CLI](https://www.rust-ui.com/docs/components/cli)

Every demo now has its own `.md` file: [Badge](https://rust-ui.com/docs/components/badge.md), [Button](https://rust-ui.com/docs/components/button.md), [Card](https://rust-ui.com/docs/components/card.md), and many more.

### Component Improvements

- **DropdownMenu**: Start, End, StartOuter, EndOuter alignment options
- **Popover**: EndOuter alignment for flexible positioning
- **Button**: Added `href`, `rel`, and `target` attribute support

### "Copy Page" & "Copy CLI" feature

You can now copy the full documentation page or the CLI.

<img src="/images/changelog/changelog-2025-09.png" alt="Copy Page and Copy CLI buttons." />


------------------------------------------------
------------------------------------------------


## July 2025 - Icons Crate & Blocks Foundation

The icons library is now a standalone crate on crates.io with multi-framework support. Blocks infrastructure is laid for pre-built UI sections.

### Highlights

- **Icons Crate**: Published to crates.io with Leptos and Dioxus support
- **Blocks**: Foundation for ready-made UI sections (headers, footers, login pages)
- **Animation System**: Refactored to PascalCase naming convention
- **Build Registry**: Automatic directory scanning with WalkDir

### Icons Multi-Framework Support

The icons crate now supports both Leptos and Dioxus frameworks:
```rust
// Leptos
use icons::LucideIcon;

// Dioxus
use icons::LucideIcon;
```

### Developer Experience

- Simplified icon imports across the codebase
- Consolidated animation imports into single file
- Registry flattening for cleaner directory structure
- Slot component simplified to support only `A` variant


