use leptos::prelude::*;
use wasm_bindgen::JsCast;

/// Hook to lock/unlock body scroll and prevent background interactions for dialogs.
///
/// This prevents the page from scrolling when dialogs are open and makes
/// background elements non-interactive by setting pointer-events: none on #target__dialog_lock_body,
/// while keeping the dialog interactive by setting pointer-events: auto on dialog elements,
/// maintaining the current scroll position by using fixed positioning with
/// a negative top offset.
///
/// When unlocking, it delays the restoration by 100ms to allow dialog
/// closing animations to complete, preventing visual flashing during the
/// transition where the page would briefly jump to the top.
///
/// # Arguments
/// * `initial_locked` - Whether the body scroll should be initially locked
///
/// # Returns
/// A reactive signal that controls the lock state - set to `true` to lock,
/// `false` to unlock with delayed restoration
pub fn use_lock_body_scroll_dialog(initial_locked: bool) -> RwSignal<bool> {
    const TARGET_DIALOG_LOCK_BODY: &str = "#target__dialog_lock_body";

    let locked_signal = RwSignal::new(initial_locked);
    let scroll_position_signal = RwSignal::new(0.0);

    Effect::new(move |_| {
        let Some(document) = window().document() else { return };
        let Some(body) = document.body() else { return };

        if locked_signal.get() {
            // Store current scroll position
            scroll_position_signal.set(window().scroll_y().unwrap_or(0.0));

            // Calculate scrollbar width for compensation
            let Some(inner_width) = window().inner_width().ok().and_then(|w| w.as_f64()) else {
                return;
            };
            let scrollbar_width = inner_width - body.client_width() as f64;

            // Apply body lock styles
            let style = body.style();
            let _ = style.set_property("position", "fixed");
            let _ = style.set_property("top", &format!("-{}px", scroll_position_signal.get()));
            let _ = style.set_property("width", "100%");
            let _ = style.set_property("overflow", "hidden");

            if scrollbar_width > 0.0 {
                let _ = style.set_property("padding-right", &format!("{scrollbar_width}px"));
            }

            // Manage pointer events
            set_pointer_events(&document, TARGET_DIALOG_LOCK_BODY, "none");
            set_dialog_pointer_events(&document, "auto");
        } else {
            // Delayed unlock to allow closing animations
            let stored_position = scroll_position_signal.get();
            set_timeout(
                move || {
                    // Remove body lock styles
                    let style = body.style();
                    for prop in ["position", "top", "width", "overflow", "padding-right"] {
                        let _ = style.remove_property(prop);
                    }
                    window().scroll_to_with_x_and_y(0.0, stored_position);

                    // Restore pointer events
                    set_pointer_events(&document, TARGET_DIALOG_LOCK_BODY, "");
                },
                std::time::Duration::from_millis(100),
            );
        }
    });

    locked_signal
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

/// Used twice to set/unset pointer-events when dialog open/closes.
fn set_pointer_events(document: &web_sys::Document, selector: &str, value: &str) {
    if let Ok(Some(element)) = document.query_selector(selector)
        && let Some(html_element) = element.dyn_ref::<web_sys::HtmlElement>()
    {
        if value.is_empty() {
            let _ = html_element.style().remove_property("pointer-events");
        } else {
            let _ = html_element.style().set_property("pointer-events", value);
        }
    }
}

/// When dialog is open, set the pointer-events only to the dialog.
fn set_dialog_pointer_events(document: &web_sys::Document, value: &str) {
    if let Ok(elements) = document.query_selector_all("[data-target='target__dialog']") {
        for i in 0..elements.length() {
            if let Some(element) = elements.item(i)
                && let Some(html_element) = element.dyn_ref::<web_sys::HtmlElement>()
            {
                let _ = html_element.style().set_property("pointer-events", value);
            }
        }
    }
}
