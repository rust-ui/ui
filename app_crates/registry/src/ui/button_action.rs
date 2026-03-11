use leptos::prelude::*;
use tw_merge::tw_merge;

use crate::hooks::use_press_hold::use_press_hold;
use crate::ui::button::{Button, ButtonSize, ButtonVariant};

/// A button that requires press-and-hold to activate.
/// Shows a progress indicator filling from left to right.
#[component]
pub fn ButtonAction(
    children: Children,
    #[prop(into)] on_complete: Callback<()>,
    #[prop(optional, default = 2000)] duration_ms: u32,
    #[prop(optional, default = ButtonVariant::Destructive)] variant: ButtonVariant,
    #[prop(optional, default = ButtonSize::Default)] size: ButtonSize,
    #[prop(into, optional)] class: String,
    #[prop(into, optional)] disabled: Signal<bool>,
) -> impl IntoView {
    let press_hold = use_press_hold(duration_ms, on_complete, disabled);

    let button_class =
        tw_merge!("relative overflow-hidden select-none active:scale-[0.99] transition-transform", class);

    let progress_style = move || {
        let width_percent = press_hold.progress_signal.get() * 100.0;
        format!(
            "position: absolute; left: 0; top: 0; bottom: 0; width: {width_percent:.1}%; background: rgba(0, 0, 0, 0.25); pointer-events: none; border-radius: inherit;"
        )
    };

    let wrapper_class = move || {
        if disabled.get() { "pointer-events-none opacity-50" } else { "" }
    };

    let ph1 = press_hold.clone();
    let ph2 = press_hold.clone();
    let ph3 = press_hold.clone();
    let ph4 = press_hold;

    view! {
        <span class=wrapper_class>
            <Button
                variant=variant
                size=size
                class=button_class
                on:pointerdown=move |_| ph1.on_pointer_down()
                on:pointerup=move |_| ph2.on_pointer_up()
                on:pointerleave=move |_| ph3.on_pointer_up()
                on:pointercancel=move |_| ph4.on_pointer_up()
            >
                <span style=progress_style></span>
                <span class="flex relative z-10 gap-2 items-center">{children()}</span>
            </Button>
        </span>
    }
}
