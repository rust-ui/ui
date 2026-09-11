# PLAN_ICONS_GRID_VIRTUAL_SCROLL.md

Icons page (`src/routes/page_icons.rs`) virtual-scroll fix. Status:
**not confirmed working** — clippy clean is not proof, screenshot showed a
broken 1-column grid after the first pass. Whoever picks this up must run the
app and look at it, not trust the compiler.

## Context

~1500 icons (`ALL_ICONS`) rendered in one `.map()` made the grid slow to
build/diff on iOS's webview (VDOM cost). Fix: virtualize with a new hook,
`app_crates/registry/src/hooks/use_grid_virtual_scroll.rs`, reusing the
pattern from `use_virtual_scroll.rs` / `data_grid.rs`'s `VirtualizedGrid`.

## What's been done

1. New hook `use_grid_virtual_scroll` (measures container width, derives
   `columns`, exposes `start_index`/`end_index`/`total_height`/`columns`).
   Registered in `hooks/mod.rs`.
2. `page_icons.rs` wired to it: `onmounted` on the scroll container sets
   `grid_element`, grid renders only `filtered_icons()[start..end]` inside an
   absolutely-positioned inner div (`translateY` by row offset), outer spacer
   div holds `total_height` for correct scrollbar size. `grid-template-columns`
   driven by `columns()`, not Tailwind `grid-cols-*` (the hook's row math and
   the actual CSS column count must agree, see hook's doc comment).
3. **Bug #1 (screenshot)**: grid rendered as a single column. Root cause:
   `client_width` measured once at mount, before iOS webview layout settled →
   `columns` locked to 1 forever (nothing re-measures except scroll/resize).
   Attempted fix: `gloo_timers::future::TimeoutFuture::new(50)` re-measure
   after mount, inside the effect.
4. **Bug #2 (found while fixing #1, not yet triggered/observed)**: the whole
   hook used raw `web_sys` FFI calls (`client_width`, `add_event_listener`,
   `web_sys::window()`) with no `#[cfg(target_arch = "wasm32")]` gate. Per
   `15cd0b0` (this repo, same day): on iOS, `dx serve --platform ios` compiles
   **natively** (`aarch64-apple-ios`, not wasm32), and wasm-bindgen's
   non-wasm32 stub **panics unconditionally** the instant any such call
   executes — even inside `if let Some(...)`. Gated the entire effect behind
   `#[cfg(target_arch = "wasm32")]`; on native the hook silently keeps its
   signal defaults (`columns = 1`, `container_height = 600`).
5. **Bug #3 (build error, caught after #2)**: `error: lifetime may not live
   long enough` on the `spawn(async move { ... })` re-measure block. Cause:
   `measure` closure (`|el: &web_sys::HtmlElement| ...`) used `item_size` by
   reference only (Rust 2021 disjoint capture — doesn't need ownership, so it
   borrows), tying `measure` to the enclosing `use_effect` closure's stack
   frame instead of `'static`. `spawn` requires a `'static` future, so moving
   `measure` into the async block failed to compile. Fix: `let measure = move
   |el: &web_sys::HtmlElement| { ... }` — force it to own `item_size`.
6. `cargo build -p registry` and `cargo clippy -p registry --no-deps` both
   clean for the default (wasm) target and `--target aarch64-apple-ios`.

## What is NOT verified yet — do this before calling it done

Clippy passing proves the code type-checks per target. It proves nothing
about runtime behavior (that's exactly how bug #1 shipped with a clean
`cargo check` in the first pass). Check yourself:

1. Run the app in a real browser (`dx serve` web target, or whatever this
   repo's `run` skill launches) and open `/icons`.
2. Confirm the grid is multi-column (5+ columns at desktop width), not 1.
3. Resize the window / rotate: column count should update.
4. Scroll to the bottom of the icon list: last row must be reachable, no
   dead space or cut-off (check `total_height` math: `total.div_ceil(columns)
   * item_size` — if `columns` is stale when `total_height` is computed you
   get a wrong scroll range).
5. Type a search query: grid should re-flow to the filtered set, not still
   show stale positions from the unfiltered set.
6. Click an icon: drawer still opens with the right icon (unrelated to
   virtualization, but the click handler moved when the DOM structure
   changed — confirm the `DrawerTrigger` click-eval hack still finds its
   target).
7. If a real device or simulator is available: run on iOS specifically
   (`aarch64-apple-ios` target, not just wasm) — this is the target the
   `#[cfg(target_arch = "wasm32")]` gate affects, and on iOS the grid will
   render at fixed `columns = 1` / `container_height = 600` defaults (no
   virtualization at all, since the effect never runs there) — decide if that
   degraded-but-safe behavior (correctness over virtualization) is acceptable
   or whether iOS needs a native measurement path.
8. Only after visually confirming 2-6 (and deciding on 7) is this plan done.
   Delete this file at that point — it's a debugging aid, not permanent docs.

## Reference facts

- `ICON_ITEM_SIZE = 72`: icon button is `size-16` (64px, fixed regardless of
  the size selector) + `gap-2` (8px). If the button's fixed size class ever
  changes, this constant must change with it.
- `use_grid_virtual_scroll` caller contract: use its `columns` for
  `grid-template-columns`, never Tailwind responsive `grid-cols-*` — the two
  will disagree and break the row math silently.
- See `[[feedback_ios_websys_gating]]` (auto-memory) and `15cd0b0` for the
  wasm32-gating pattern; any new hook touching `web_sys`/`js_sys` needs the
  same audit before it's assumed iOS-safe.
