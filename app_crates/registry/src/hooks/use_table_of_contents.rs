#![cfg_attr(
    not(target_arch = "wasm32"),
    allow(dead_code, reason = "DOM scroll-spy helpers are only executable in the wasm browser target")
)]

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use dioxus::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::closure::Closure;

/// Distance (px) below the top of the viewport a heading must cross before
/// it's considered "active" for scroll-spy purposes.
const SCROLL_OFFSET: f64 = 100.0;

/// State returned by `use_table_of_contents`.
#[derive(Clone, Copy)]
pub struct TableOfContentsState {
    /// Anchor id of the heading currently in view, if any.
    pub active_anchor: ReadSignal<Option<String>>,
}

/// Syncs `window.location.hash` to `id` via `history.replaceState` (no navigation/reload).
fn sync_hash(id: &str) {
    let Some(window) = web_sys::window() else { return };
    let Ok(history) = window.history() else { return };
    let hash = format!("#{id}");
    let _ = history.replace_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(&hash));
}

fn cache_heading_positions(
    document: &web_sys::Document,
    window: &web_sys::Window,
    anchors: &[String],
) -> Vec<(String, f64)> {
    let scroll_y = window.scroll_y().unwrap_or(0.0);
    anchors
        .iter()
        .filter_map(|id| {
            document
                .get_element_by_id(id)
                .map(|el| (id.clone(), el.get_bounding_client_rect().top() + scroll_y))
        })
        .collect()
}

/// Hook driving a table-of-contents scroll-spy, porting `table_of_contents.js`.
///
/// Caches heading positions (recached on resize), tracks which heading is
/// currently in view as the page scrolls, and keeps `window.location.hash`
/// in sync via `history.replaceState` (no navigation/reload).
///
/// # Arguments
/// * `anchors` - heading anchor ids, in document order (matches the TOC items)
#[cfg_attr(not(target_arch = "wasm32"), allow(unused_variables))]
pub fn use_table_of_contents(anchors: &[String]) -> TableOfContentsState {
    let anchors = anchors.to_vec();
    let active_anchor = use_signal(|| None::<String>);
    let positions = use_signal(Vec::<(String, f64)>::new);

    let is_mounted = Arc::new(AtomicBool::new(true));
    let is_mounted_cleanup = Arc::clone(&is_mounted);
    use_drop(move || {
        is_mounted_cleanup.store(false, Ordering::SeqCst);
    });

    #[cfg(target_arch = "wasm32")]
    use_effect(move || {
        let Some(window) = web_sys::window() else { return };
        let Some(document) = window.document() else { return };
        let mut positions = positions;

        positions.set(cache_heading_positions(&document, &window, &anchors));

        // Recompute active heading on scroll.
        let is_mounted_for_scroll = Arc::clone(&is_mounted);
        let mut active_anchor_for_scroll = active_anchor;
        let positions_for_scroll = positions;
        let window_for_scroll = window.clone();
        let scroll_handler = Closure::wrap(Box::new(move || {
            if !is_mounted_for_scroll.load(Ordering::SeqCst) {
                return;
            }
            let scroll_pos = window_for_scroll.scroll_y().unwrap_or(0.0) + SCROLL_OFFSET;
            let current = positions_for_scroll
                .peek()
                .iter()
                .rfind(|(_, top)| scroll_pos >= *top)
                .map(|(id, _)| id.clone());

            if active_anchor_for_scroll.peek().as_deref() != current.as_deref() {
                if let Some(id) = &current {
                    sync_hash(id);
                }
                active_anchor_for_scroll.set(current);
            }
        }) as Box<dyn FnMut()>);
        let _ = window.add_event_listener_with_callback("scroll", scroll_handler.as_ref().unchecked_ref());
        scroll_handler.forget();

        // Recache heading positions on resize (layout may have shifted).
        let anchors_for_resize = anchors.clone();
        let document_for_resize = document.clone();
        let window_for_resize = window.clone();
        let mut positions_for_resize = positions;
        let resize_handler = Closure::wrap(Box::new(move || {
            positions_for_resize.set(cache_heading_positions(
                &document_for_resize,
                &window_for_resize,
                &anchors_for_resize,
            ));
        }) as Box<dyn FnMut()>);
        let _ = window.add_event_listener_with_callback("resize", resize_handler.as_ref().unchecked_ref());
        resize_handler.forget();
    });

    TableOfContentsState {
        active_anchor: active_anchor.into(),
    }
}
