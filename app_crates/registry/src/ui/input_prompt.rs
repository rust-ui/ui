use leptos::prelude::*;
use leptos_ui::clx;
use tw_merge::tw_merge;

use crate::ui::button::{Button, ButtonSize};
use crate::ui::input_group::{InputGroup, InputGroupAddon, InputGroupAddonAlign};

mod components {
    use super::*;
    clx! { InputPromptTools, div, "flex items-center gap-1" }
}

pub use components::*;

/// Outer wrapper — InputGroup with overflow clipping.
#[component]
pub fn InputPrompt(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    let merged_class = tw_merge!("overflow-hidden", class);
    view! { <InputGroup class=merged_class>{children()}</InputGroup> }
}

/// Auto-growing textarea bound to an RwSignal<String>.
/// Enter (without Shift) fires on_submit.
#[component]
pub fn InputPromptTextarea(
    #[prop(into)] value: RwSignal<String>,
    #[prop(into, optional)] placeholder: Option<String>,
    #[prop(into, optional)] class: String,
    #[prop(optional)] on_submit: Option<Callback<()>>,
) -> impl IntoView {
    let placeholder = placeholder.unwrap_or_else(|| "Write a message...".to_string());
    let merged_class = tw_merge!(
        "flex-1 resize-none rounded-none border-0 bg-transparent py-3 px-3 \
         field-sizing-content min-h-[52px] max-h-48 text-sm shadow-none \
         focus-visible:ring-0 dark:bg-transparent placeholder:text-muted-foreground",
        class
    );

    view! {
        <textarea
            data-slot="input-group-control"
            class=merged_class
            placeholder=placeholder
            prop:value=move || value.get()
            on:input=move |ev| value.set(event_target_value(&ev))
            on:keydown=move |ev| {
                if ev.key() == "Enter" && !ev.shift_key() {
                    ev.prevent_default();
                    if let Some(cb) = on_submit {
                        cb.run(());
                    }
                }
            }
        />
    }
}

/// Block-end footer row. Holds tools on the left, submit on the right.
#[component]
pub fn InputPromptFooter(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    let merged_class = tw_merge!("border-t px-2 py-2 justify-between gap-1", class);
    view! {
        <InputGroupAddon align=InputGroupAddonAlign::BlockEnd class=merged_class>
            {children()}
        </InputGroupAddon>
    }
}

/// Send button. Pass disabled=Signal::derive(move || value.get().trim().is_empty())
/// to disable when the textarea is empty.
#[component]
pub fn InputPromptSubmit(
    #[prop(into, optional)] disabled: Option<Signal<bool>>,
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    let is_disabled = disabled.unwrap_or_else(|| Signal::derive(|| false));
    let merged_class = tw_merge!("size-8 rounded-full", class);

    view! {
        <Button size=ButtonSize::Icon class=merged_class attr:r#type="button" attr:disabled=move || is_disabled.get()>
            {children()}
        </Button>
    }
}
