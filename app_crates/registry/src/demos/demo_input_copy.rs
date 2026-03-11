use icons::{Check, Copy};
use leptos::prelude::*;

use crate::hooks::use_copy_clipboard::use_copy_clipboard;
use crate::ui::button::{Button, ButtonVariant};
use crate::ui::input::Input;

#[component]
pub fn DemoInputCopy() -> impl IntoView {
    let url_signal = RwSignal::new("https://rust-ui.com/docs/components/input");
    let (copy_to_clipboard, copied_signal) = use_copy_clipboard(None);

    let handle_copy = move |_| {
        copy_to_clipboard(url_signal.get());
    };

    view! {
        <div class="flex gap-2">
            <Input prop:value=move || url_signal().to_string() attr:readonly=true class="flex-1" />

            <Button variant=ButtonVariant::Outline on:click=handle_copy>
                {move || {
                    if copied_signal.get() { view! { <Check /> }.into_any() } else { view! { <Copy /> }.into_any() }
                }}
            </Button>
        </div>
    }
}
