use leptos::prelude::*;

pub fn use_lock_body_scroll(initial_locked: bool) -> RwSignal<bool> {
    let locked_signal = RwSignal::new(initial_locked);

    Effect::new(move |_| {
        if let Some(body) = window().document().and_then(|d| d.body()) {
            let overflow = if locked_signal.get() { "hidden" } else { "" };
            let _ = body.style().set_property("overflow", overflow);
        }
    });

    locked_signal
}
