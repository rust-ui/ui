use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use dioxus::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::closure::Closure;

/// Extra rows rendered above/below the viewport for smooth scrolling.
const BUFFER_ROWS: usize = 3;

/// State returned by `use_grid_virtual_scroll`.
#[derive(Clone, Copy)]
pub struct GridVirtualScrollState {
    /// First visible item index (inclusive).
    pub start_index: Memo<usize>,
    /// Last visible item index (exclusive).
    pub end_index: Memo<usize>,
    /// Total scrollable height (px) of the virtual grid.
    pub total_height: ReadSignal<usize>,
    /// Columns currently fitting the container width.
    pub columns: ReadSignal<usize>,
}

/// Hook for virtual scrolling a uniform, multi-column item grid (icon
/// pickers, emoji pickers, etc).
///
/// Unlike `use_virtual_scroll` (single-column
/// rows, e.g. a data grid), this measures the container width itself to
/// derive how many square `item_size`-px cells fit per row, then only
/// renders the row range visible in the viewport.
///
/// Sizing (columns, row math) is driven entirely by this hook so the caller
/// must apply the returned `columns` as `grid-template-columns` instead of
/// Tailwind responsive `grid-cols-*` classes — otherwise the actual CSS
/// column count and this hook's row math disagree.
///
/// # Arguments
/// * `container_element` - the scrollable grid container (set from `onmounted`)
/// * `total_items` - total item count backing the grid
/// * `item_size` - width/height (px) of one square cell, gap included
#[cfg_attr(not(target_arch = "wasm32"), allow(unused_variables))]
#[must_use]
pub fn use_grid_virtual_scroll(
    container_element: ReadSignal<Option<web_sys::Element>>,
    total_items: ReadSignal<usize>,
    item_size: usize,
) -> GridVirtualScrollState {
    let scroll_top_signal = use_signal(|| 0usize);
    let container_height_signal = use_signal(|| 600usize);
    let columns_signal = use_signal(|| 1usize);

    let is_mounted = Arc::new(AtomicBool::new(true));
    let is_mounted_for_cleanup = Arc::clone(&is_mounted);
    use_drop(move || {
        is_mounted_for_cleanup.store(false, Ordering::SeqCst);
    });

    // wasm32-only: `web_sys` FFI calls panic unconditionally on a native
    // (non-wasm) target such as iOS's `dx serve --platform ios`, even
    // behind `if let Some(...)` — see use_table_of_contents.rs.
    #[cfg(target_arch = "wasm32")]
    {
        let is_mounted_for_effect = Arc::clone(&is_mounted);
        let is_mounted_for_scroll = Arc::clone(&is_mounted);
        let is_mounted_for_resize = Arc::clone(&is_mounted);
        use_effect(move || {
        let Some(el) = container_element.peek().clone() else {
            return;
        };
        let el: web_sys::HtmlElement = match el.dyn_into() {
            Ok(e) => e,
            Err(_) => return,
        };

        let measure = move |el: &web_sys::HtmlElement| {
            let width = usize::try_from(el.client_width().max(0)).unwrap_or(0);
            (width + item_size).max(item_size) / item_size
        };

        if is_mounted_for_effect.load(Ordering::SeqCst) {
            let mut container_height_signal = container_height_signal;
            let mut columns_signal = columns_signal;
            container_height_signal.set(usize::try_from(el.client_height().max(0)).unwrap_or(0));
            columns_signal.set(measure(&el).max(1));
        }

        // iOS's native webview can report `client_width` 0 at `onmounted`
        // time (layout not settled yet), permanently locking the grid to 1
        // column since nothing else re-measures until the user scrolls or
        // resizes — re-measure a beat later once layout has caught up.
        let is_mounted_for_remeasure = Arc::clone(&is_mounted_for_effect);
        let el_for_remeasure = el.clone();
        let mut container_height_signal_for_remeasure = container_height_signal;
        let mut columns_signal_for_remeasure = columns_signal;
        spawn(async move {
            gloo_timers::future::TimeoutFuture::new(50).await;
            if !is_mounted_for_remeasure.load(Ordering::SeqCst) {
                return;
            }
            container_height_signal_for_remeasure
                .set(usize::try_from(el_for_remeasure.client_height().max(0)).unwrap_or(0));
            columns_signal_for_remeasure.set(measure(&el_for_remeasure).max(1));
        });

        let mut scroll_top_signal_clone = scroll_top_signal;
        let mut container_height_signal_clone = container_height_signal;
        let mut columns_signal_clone = columns_signal;
        let el_clone = el.clone();
        let is_mounted_for_scroll = Arc::clone(&is_mounted_for_scroll);
        let scroll_handler = Closure::wrap(Box::new(move || {
            if !is_mounted_for_scroll.load(Ordering::SeqCst) {
                return;
            }
            scroll_top_signal_clone.set(usize::try_from(el_clone.scroll_top().max(0)).unwrap_or(0));
            container_height_signal_clone.set(usize::try_from(el_clone.client_height().max(0)).unwrap_or(0));
            columns_signal_clone.set(measure(&el_clone).max(1));
        }) as Box<dyn FnMut()>);
        let _ = el.add_event_listener_with_callback("scroll", scroll_handler.as_ref().unchecked_ref());
        scroll_handler.forget();

        // Column count depends on container width, which only `scroll`
        // doesn't cover (e.g. device rotation) — also recompute on resize.
        let mut container_height_signal_for_resize = container_height_signal;
        let mut columns_signal_for_resize = columns_signal;
        let el_for_resize = el.clone();
        let is_mounted_for_resize = Arc::clone(&is_mounted_for_resize);
        let resize_handler = Closure::wrap(Box::new(move || {
            if !is_mounted_for_resize.load(Ordering::SeqCst) {
                return;
            }
            container_height_signal_for_resize.set(usize::try_from(el_for_resize.client_height().max(0)).unwrap_or(0));
            columns_signal_for_resize.set(measure(&el_for_resize).max(1));
        }) as Box<dyn FnMut()>);
        if let Some(window) = web_sys::window() {
            let _ = window.add_event_listener_with_callback("resize", resize_handler.as_ref().unchecked_ref());
        }
        resize_handler.forget();
        });
    }

    let start_index = use_memo(move || {
        let scroll_top = scroll_top_signal();
        let columns = columns_signal();
        let start_row = (scroll_top / item_size).saturating_sub(BUFFER_ROWS);
        start_row * columns
    });

    let end_index = use_memo(move || {
        let scroll_top = scroll_top_signal();
        let container_height = container_height_signal();
        let columns = columns_signal();
        let total = total_items();

        let visible_rows = (container_height / item_size) + 1;
        let start_row = scroll_top / item_size;
        let end_row = start_row + visible_rows + BUFFER_ROWS * 2;

        (end_row * columns).min(total)
    });

    let total_height = use_memo(move || {
        let columns = columns_signal();
        let total = total_items();
        total.div_ceil(columns) * item_size
    });

    GridVirtualScrollState {
        start_index,
        end_index,
        total_height: total_height.into(),
        columns: columns_signal.into(),
    }
}
