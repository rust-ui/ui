use leptos::html::Nav;
use leptos::prelude::*;

/// Hook for detecting scroll state of a horizontally scrollable element
///
/// Returns a tuple of (update_fn, show_left_signal, show_right_signal) where:
/// - `update_fn`: Function to call on scroll events to update fade states
/// - `show_left_signal`: ReadSignal<bool> indicating if left fade should be visible
/// - `show_right_signal`: ReadSignal<bool> indicating if right fade should be visible
pub fn use_can_scroll(node_ref: NodeRef<Nav>) -> (impl Fn() + Clone, ReadSignal<bool>, ReadSignal<bool>) {
    let show_left_fade_signal = RwSignal::new(false);
    let show_right_fade_signal = RwSignal::new(true);

    let update_fades = move || {
        if let Some(element) = node_ref.get() {
            let scroll_left = element.scroll_left();
            let scroll_width = element.scroll_width();
            let client_width = element.client_width();

            show_left_fade_signal.set(scroll_left > 0);
            show_right_fade_signal.set(scroll_left < scroll_width - client_width - 1);
        }
    };

    (update_fades, show_left_fade_signal.read_only(), show_right_fade_signal.read_only())
}
