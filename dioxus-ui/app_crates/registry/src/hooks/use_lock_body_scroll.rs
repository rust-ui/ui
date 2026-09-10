use dioxus::prelude::*;

pub fn use_lock_body_scroll(initial_locked: bool) -> Signal<bool> {
    let locked_signal = use_signal(|| initial_locked);

    use_effect(move || {
        if let Some(body) = web_sys::window().and_then(|w| w.document()).and_then(|d| d.body()) {
            let overflow = if locked_signal() { "hidden" } else { "" };
            let _ = body.style().set_property("overflow", overflow);
        }
    });

    locked_signal
}
