use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use dioxus::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

/// Buffer rows to render above/below viewport for smooth scrolling
const BUFFER_ROWS: usize = 5;
const ROW_HEIGHT: usize = 36;

/// State returned by `use_virtual_scroll` hook.
#[derive(Clone, Copy)]
pub struct VirtualScrollState {
    /// First visible row index
    pub start_index: Memo<usize>,
    /// Last visible row index (exclusive)
    pub end_index: Memo<usize>,
    /// Total height of the virtual container in pixels
    pub total_height: ReadSignal<usize>,
}

/// Get the virtual scroll context from a parent VirtualizedGrid.
/// Returns None if used outside of a VirtualizedGrid.
pub fn use_virtual_scroll_context() -> Option<VirtualScrollState> {
    Some(consume_context::<VirtualScrollState>())
}

/// Hook for virtual scrolling in data grids.
///
/// Only renders rows that are visible in the viewport plus a small buffer,
/// dramatically improving performance for large datasets.
///
/// # Arguments
/// * `container_element` - Signal for the scrollable container element (set from onmounted)
/// * `total_rows` - Signal containing the total number of rows
///
/// # Returns
/// * `VirtualScrollState` with start/end indices and total height
pub fn use_virtual_scroll(
    container_element: ReadSignal<Option<web_sys::Element>>,
    total_rows: ReadSignal<usize>,
) -> VirtualScrollState {
    let scroll_top_signal = use_signal(|| 0usize);
    let container_height_signal = use_signal(|| 600usize); // Default height

    // Track if component is still mounted to prevent accessing disposed signals
    let is_mounted = Arc::new(AtomicBool::new(true));
    let is_mounted_for_cleanup = Arc::clone(&is_mounted);

    // Cleanup: mark as unmounted when component is disposed
    use_drop(move || {
        is_mounted_for_cleanup.store(false, Ordering::SeqCst);
    });

    // Set up scroll listener on mount
    let is_mounted_for_effect = Arc::clone(&is_mounted);
    let is_mounted_for_scroll = Arc::clone(&is_mounted);
    use_effect(move || {
        let Some(el) = container_element.peek().clone() else { return };
        let el: web_sys::HtmlElement = match el.dyn_into() {
            Ok(e) => e,
            Err(_) => return,
        };

        // Update container height immediately
        if is_mounted_for_effect.load(Ordering::SeqCst) {
            container_height_signal.clone().set(el.client_height().max(0) as usize);
        }

        // Set up scroll listener with mounted check
        let is_mounted_for_handler = Arc::clone(&is_mounted_for_scroll);
        let mut scroll_top_signal_clone = scroll_top_signal;
        let mut container_height_signal_clone = container_height_signal;
        let el_clone = el.clone();
        let scroll_handler = Closure::wrap(Box::new(move || {
            // Check if still mounted before accessing signals
            if !is_mounted_for_handler.load(Ordering::SeqCst) {
                return;
            }
            scroll_top_signal_clone.set(el_clone.scroll_top().max(0) as usize);
            container_height_signal_clone.set(el_clone.client_height().max(0) as usize);
        }) as Box<dyn FnMut()>);

        let _ = el.add_event_listener_with_callback("scroll", scroll_handler.as_ref().unchecked_ref());

        // Keep the closure alive - it will check is_mounted before accessing signals
        scroll_handler.forget();
    });

    let start_index = use_memo(move || {
        let scroll_top = scroll_top_signal();
        let start = scroll_top / ROW_HEIGHT;
        start.saturating_sub(BUFFER_ROWS)
    });

    let end_index = use_memo(move || {
        let scroll_top = scroll_top_signal();
        let container_height = container_height_signal();
        let total = total_rows();

        let visible_rows = (container_height / ROW_HEIGHT) + 1;
        let start = scroll_top / ROW_HEIGHT;
        let end = start + visible_rows + BUFFER_ROWS * 2;

        end.min(total)
    });

    let total_height = use_memo(move || total_rows() * ROW_HEIGHT);

    VirtualScrollState { start_index, end_index, total_height: total_height.into() }
}
