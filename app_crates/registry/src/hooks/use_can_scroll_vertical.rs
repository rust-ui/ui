use dioxus::prelude::*;

/// Returns (on_scroll_handler, can_scroll_up, can_scroll_down).
/// Attach on_scroll to the scrollable element's onscroll event.
pub fn use_can_scroll_vertical() -> (impl Fn(Event<ScrollData>) + Clone, ReadSignal<bool>, ReadSignal<bool>) {
    let can_up = use_signal(|| false);
    let can_down = use_signal(|| false);

    let on_scroll = move |_ev: Event<ScrollData>| {
        let scroll_top = _ev.scroll_top();
        let scroll_height = _ev.scroll_height() as f64;
        let client_height = _ev.client_height() as f64;
        *can_up.write_unchecked() = scroll_top > 0.0;
        *can_down.write_unchecked() = scroll_top < scroll_height - client_height - 1.0;
    };

    (on_scroll, can_up.into(), can_down.into())
}
