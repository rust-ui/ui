use dioxus::prelude::*;
use tw_merge::tw_merge;

/// Adds press feedback (scale) to children — handles mobile where :active doesn't fire on divs.
#[component]
pub fn Pressable(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let mut is_pressed = use_signal(|| false);

    let merged =
        tw_merge!("transition-transform", if is_pressed() { "scale-95" } else { "" }, class.as_deref().unwrap_or(""));

    rsx! {
        div {
            class: "{merged}",
            onpointerdown: move |_| is_pressed.set(true),
            onpointerup: move |_| is_pressed.set(false),
            onpointerleave: move |_| is_pressed.set(false),
            onpointercancel: move |_| is_pressed.set(false),
            {children}
        }
    }
}
