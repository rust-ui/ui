use dioxus::prelude::*;
use tw_merge::tw_merge;

use crate::ui::button::{Button, ButtonSize};
use crate::ui::input_group::{InputGroup, InputGroupAddon, InputGroupAddonAlign};

/* ========================================================== */
/*                     ✨ CLX COMPONENTS ✨                   */
/* ========================================================== */

#[component]
pub fn InputPromptTools(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("flex items-center gap-1", class.as_deref().unwrap_or(""));
    rsx! { div { "data-name": "InputPromptTools", class: "{merged}", {children} } }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

/// Outer wrapper — InputGroup with overflow clipping.
#[component]
pub fn InputPrompt(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("overflow-hidden", class.as_deref().unwrap_or(""));
    rsx! { InputGroup { class: merged, {children} } }
}

/// Auto-growing textarea bound to a Signal<String>.
/// Enter (without Shift) fires on_submit.
#[component]
pub fn InputPromptTextarea(
    value: Signal<String>,
    #[props(into, optional)] placeholder: Option<String>,
    #[props(into, optional)] class: Option<String>,
    #[props(optional)] on_submit: Option<EventHandler<()>>,
) -> Element {
    let placeholder = placeholder.unwrap_or_else(|| "Write a message...".to_string());
    let merged = tw_merge!(
        "flex-1 resize-none rounded-none border-0 bg-transparent py-3 px-3 \
         field-sizing-content min-h-[52px] max-h-48 text-sm shadow-none \
         focus-visible:ring-0 dark:bg-transparent placeholder:text-muted-foreground",
        class.as_deref().unwrap_or("")
    );

    rsx! {
        textarea {
            "data-slot": "input-group-control",
            class: "{merged}",
            placeholder: "{placeholder}",
            value: "{value()}",
            oninput: move |ev| value.set(ev.value()),
            onkeydown: move |ev| {
                if ev.key() == Key::Enter && !ev.modifiers().shift() {
                    ev.prevent_default();
                    if let Some(cb) = on_submit {
                        cb.call(());
                    }
                }
            }
        }
    }
}

/// Block-end footer row. Holds tools on the left, submit on the right.
#[component]
pub fn InputPromptFooter(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("border-t px-2 py-2 justify-between gap-1", class.as_deref().unwrap_or(""));
    rsx! {
        InputGroupAddon { align: InputGroupAddonAlign::BlockEnd, class: merged,
            {children}
        }
    }
}

/// Send button. Pass disabled=true to disable when the textarea is empty.
#[component]
pub fn InputPromptSubmit(
    #[props(optional)] disabled: bool,
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let merged = tw_merge!("size-8 rounded-full", class.as_deref().unwrap_or(""));

    rsx! {
        Button { size: ButtonSize::Icon, class: merged, button_type: "button", disabled: disabled,
            {children}
        }
    }
}
