# Data Grid Full Parity Plan

## Goal

Make the Dioxus `data_grid` implementation match the Leptos implementation as closely as possible in:

- behavior
- component structure
- state flow
- runtime integration assumptions

The target is parity with the current Leptos implementation, not extra features beyond it.

## Ground Rules

- If Leptos does not implement a behavior, Dioxus should not invent it.
- If Leptos relies on root-level integration, Dioxus should do the same where possible.
- Framework-specific adaptations are acceptable only where Dioxus requires different hook or context wiring.
- Any Dioxus-only guard should be kept only if it does not change nominal behavior and only prevents crashes.

## Current Confirmed Gaps

### 1. Demo behavior has diverged from Leptos

Dioxus demo currently diverged by implementing `Cut` and `Clear` actions.
Leptos renders those menu entries but does not implement those actions.

Required action:

- remove Dioxus-only `Cut` and `Clear` behavior
- keep menu items visible if Leptos keeps them visible
- match Leptos action wiring exactly

### 2. Demo data loading path is not aligned

Leptos demo uses:

- `#[server] get_data_grid_rows()`
- `Resource::new(...)`
- loading state
- error state

Dioxus demo still uses local `initial_rows()` synchronously.

Required action:

- replace synchronous local initialization with the Dioxus equivalent of async data loading
- preserve the same visible UX states:
  - loading
  - loaded rows
  - error

Constraint:

- the mechanism will not be line-for-line identical because Leptos `Resource` and Dioxus async hooks differ
- the user-facing behavior must match

### 3. `use_data_grid_state` structure is behind the Leptos model

Leptos version uses a wrapper ref for click-outside handling.
Dioxus version still uses a mounted DOM element stored in a signal.

Required action:

- port Dioxus `use_data_grid_state` to the ref-based model if supported cleanly by the current hooks
- align naming and ownership with Leptos:
  - wrapper ref
  - copy signal
  - selection clearing on outside click

### 4. `ui/data_grid.rs` still lacks some structural parity

Leptos has:

- `VirtualizedGrid`
- `VirtualizedGridBody`
- `VirtualFor`
- more signal-oriented visibility plumbing
- `EditableCellContent` shaped around the Leptos edit context contract

Dioxus already has part of this, but not full structural parity.

Required action:

- add `VirtualFor` to Dioxus
- align `Grid`, `GridBody`, `GridRow`, header/cell visibility, and generic header helpers with the Leptos model
- make visibility/state wiring reactive in the places where Leptos relies on reactive reads

Note:

- this does not mean blindly copying Leptos syntax
- it means matching the same dataflow and rendering semantics in Dioxus

### 5. `multi_select` is behaviorally close but not structurally matched

Leptos `MultiSelect` reuses select primitives:

- `SelectGroup`
- `SelectItem`
- `SelectLabel`

Dioxus currently uses custom components to avoid invalid `SelectContext` coupling.

Required action:

- audit `select.rs` and `multi_select.rs`
- determine whether Dioxus can reuse the same primitive split without reintroducing the context bug
- if not, keep the Dioxus-safe wrappers but make their markup, styles, and interaction model match Leptos exactly

### 6. Integration assumptions must match the Leptos app shell

Leptos root app provides:

- global toaster
- scroll lock setup

Dioxus had local/demo-only gaps here and already needed fixes.

Required action:

- keep toaster at the Dioxus root
- ensure `ScrollLock` integration is initialized at the app level if the app architecture allows it
- keep defensive guards only where Dioxus web runtime otherwise hard-crashes

### 7. Reactive semantics must be reviewed systematically

One confirmed bug already came from using non-reactive reads in Dioxus where Leptos used reactive reads.

Required action:

- audit every render-time signal read in:
  - `use_cell_selection`
  - `use_drag_selection`
  - `use_data_grid_state`
  - `demo_data_grid`
  - `ui/data_grid`
- verify that render-time reads subscribe reactively
- keep non-reactive reads only in event-time helpers

## Execution Order

### Phase 1. Re-baseline to exact Leptos behavior

1. Remove Dioxus-only `Cut` and `Clear` mutations.
2. Reconfirm context menu behavior against Leptos:
   - `Copy`
   - hold-to-delete
   - context selection behavior
3. Reconfirm inline edit behavior against Leptos.

Exit condition:

- no Dioxus-only behavior remains in the demo

### Phase 2. Align shared state and integration

1. Port `use_data_grid_state` toward the Leptos shape.
2. Align wrapper ref / click-outside handling.
3. Recheck root-level toaster and scroll-lock integration.

Exit condition:

- Dioxus runtime wiring matches Leptos assumptions without local demo hacks

### Phase 3. Align grid primitives

1. Port missing `VirtualFor`.
2. Align visibility handling across:
   - pinned columns
   - non-pinned columns
   - header cells
   - body cells
3. Align `EditableCellContent` behavior and save/cancel flow.
4. Align generic header/grid helpers where Dioxus still differs materially.

Exit condition:

- Dioxus `ui/data_grid.rs` exposes the same conceptual primitives as Leptos

### Phase 4. Align demo loading flow

1. Replace `initial_rows()` bootstrap with async loading.
2. Add loading and error rendering matching Leptos.
3. Keep row editing, pinning, sorting, drag selection, and delete behavior working on loaded data.

Exit condition:

- demo page presents the same UX states as Leptos

### Phase 5. Final parity audit

1. Diff these files directly against Leptos and review remaining gaps:
   - `app_crates/registry/src/demos/demo_data_grid.rs`
   - `app_crates/registry/src/ui/data_grid.rs`
   - `app_crates/registry/src/ui/multi_select.rs`
   - `app_crates/registry/src/hooks/use_data_grid_state.rs`
   - `app_crates/registry/src/hooks/use_cell_selection.rs`
   - `app_crates/registry/src/hooks/use_drag_selection.rs`
2. Classify each remaining diff as:
   - required Dioxus adaptation
   - acceptable naming/typing difference
   - still missing parity
3. Close only the third category.

Exit condition:

- every remaining diff is either framework-mandated or semantically neutral

## Validation

### Compile checks

- `cargo check -p registry`
- `cargo check`

### Runtime checks on the demo page

- page opens without panic
- `View` multi-select opens and closes correctly
- drag multi-cell selection updates live
- right click selects the correct cell/range
- `Copy` works
- `Cut` and `Clear` match Leptos behavior exactly
- hold-to-delete deletes the correct rows
- inline edit saves and cancels correctly
- pinned columns stay aligned while scrolling
- sorting updates row order and row positioning correctly
- add-row still works
- loading and error states render correctly

## Definition Of Done

The work is done when:

- the Dioxus `data_grid` demo matches the Leptos demo in visible behavior
- shared grid primitives expose the same conceptual feature set
- remaining code differences are only framework-required adaptations
- no known runtime panic remains on the demo page
- compile checks pass

## Non-Goals

- adding new behavior not present in Leptos
- refactoring unrelated UI primitives
- touching unrelated work such as `assets/tailwind.css`
