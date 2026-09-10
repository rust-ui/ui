use dioxus::prelude::*;
use tw_merge::tw_merge;

use crate::hooks::use_press_hold::use_press_hold;
use crate::ui::button::{Button, ButtonSize, ButtonVariant};

/// A button that requires press-and-hold to activate.
/// Shows a progress indicator filling from left to right.
#[component]
pub fn ButtonAction(
    children: Element,
    #[props(into)] on_complete: EventHandler<()>,
    #[props(default = 2000)] duration_ms: u32,
    #[props(default = ButtonVariant::Destructive)] variant: ButtonVariant,
    #[props(default = ButtonSize::Default)] size: ButtonSize,
    #[props(into, optional)] class: Option<String>,
    #[props(optional)] disabled: bool,
) -> Element {
    let press_hold = use_press_hold(duration_ms, on_complete, disabled);

    let button_class = tw_merge!(
        "relative overflow-hidden select-none active:scale-[0.99] transition-transform",
        class.as_deref().unwrap_or("")
    );

    let progress_width = (press_hold.progress_signal)();
    let progress_style = format!(
        "position: absolute; left: 0; top: 0; bottom: 0; width: {:.1}%; background: rgba(0, 0, 0, 0.25); pointer-events: none; border-radius: inherit;",
        progress_width * 100.0
    );

    let wrapper_class = if disabled { "pointer-events-none opacity-50" } else { "" };

    let ph1 = press_hold.clone();
    let ph2 = press_hold.clone();
    let ph3 = press_hold.clone();
    let ph4 = press_hold;

    rsx! {
        span { class: "{wrapper_class}",
            Button {
                variant: variant,
                size: size,
                class: "{button_class}",
                onpointerdown: move |_| ph1.on_pointer_down(),
                onpointerup: move |_| ph2.on_pointer_up(),
                onpointerleave: move |_| ph3.on_pointer_up(),
                onpointercancel: move |_| ph4.on_pointer_up(),
                span { style: "{progress_style}" }
                span { class: "flex relative z-10 gap-2 items-center", {children} }
            }
        }
    }
}
