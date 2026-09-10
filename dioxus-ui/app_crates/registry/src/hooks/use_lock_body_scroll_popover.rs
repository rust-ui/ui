use dioxus::prelude::*;

/// Hook to lock/unlock body scroll while preserving scroll position.
///
/// This prevents the page from scrolling when selects or modals are open,
/// maintaining the current scroll position by using fixed positioning with
/// a negative top offset.
///
/// When unlocking, it delays the restoration by 100ms to allow select
/// closing animations to complete, preventing visual flashing during the
/// transition where the page would briefly jump to the top.
///
/// # Arguments
/// * `initial_locked` - Whether the body scroll should be initially locked
///
/// # Returns
/// A reactive signal that controls the lock state - set to `true` to lock,
/// `false` to unlock with delayed restoration
pub fn use_lock_body_scroll_popover(initial_locked: bool) -> Signal<bool> {
    let locked_signal = use_signal(|| initial_locked);
    let mut scroll_position_signal = use_signal(|| 0.0_f64);

    use_effect(move || {
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;

            let Some(body) = web_sys::window().and_then(|w| w.document()).and_then(|d| d.body()) else {
                return;
            };
            let window = web_sys::window().unwrap();

            if locked_signal() {
                let current_scroll = window.scroll_y().unwrap_or(0.0);
                scroll_position_signal.set(current_scroll);

                let Some(inner_width) = window.inner_width().ok().and_then(|w| w.as_f64()) else {
                    return;
                };
                let scrollbar_width = inner_width - body.client_width() as f64;
                let style = body.style();

                let _ = style.set_property("position", "fixed");
                let _ = style.set_property("top", &format!("-{current_scroll}px"));
                let _ = style.set_property("width", "100%");
                let _ = style.set_property("overflow", "hidden");

                if scrollbar_width > 0.0 {
                    let _ = style.set_property("padding-right", &format!("{scrollbar_width}px"));
                }
            } else {
                let stored_position = scroll_position_signal();
                let body_clone = body.clone();
                let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                    wasm_bindgen::closure::Closure::once_into_js(move || {
                        let style = body_clone.style();
                        for prop in ["position", "top", "width", "overflow", "padding-right"] {
                            let _ = style.remove_property(prop);
                        }
                        if let Some(w) = web_sys::window() {
                            w.scroll_to_with_x_and_y(0.0, stored_position);
                        }
                    })
                    .unchecked_ref(),
                    100,
                );
            }
        }
    });

    locked_signal
}
