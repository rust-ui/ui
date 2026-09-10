use dioxus::prelude::*;
use tw_merge::tw_merge;

#[component]
pub fn RadioGroup(#[props(into, optional)] class: Option<String>, value: Signal<String>, children: Element) -> Element {
    provide_context(value);
    let merged = tw_merge!("flex flex-col gap-3", class.as_deref().unwrap_or(""));
    rsx! {
        div { "data-name": "RadioGroup", class: "{merged}", role: "radiogroup", {children} }
    }
}

#[component]
pub fn RadioGroupItem(
    #[props(into, optional)] class: Option<String>,
    #[props(into)] value: String,
    #[props(into, optional)] id: Option<String>,
    #[props(default = false)] disabled: bool,
) -> Element {
    let mut selected = use_context::<Signal<String>>();
    let is_checked = selected() == value;
    let state = if is_checked { "checked" } else { "unchecked" };

    let radio_class = tw_merge!(
        "aspect-square size-4 shrink-0 rounded-full border border-input shadow-xs transition-colors",
        "focus:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2",
        "disabled:cursor-not-allowed disabled:opacity-50",
        "data-[state=checked]:border-primary",
        class.as_deref().unwrap_or("")
    );

    rsx! {
        button {
            "data-name": "RadioGroupItem",
            r#type: "button",
            role: "radio",
            id: id.as_deref().unwrap_or(""),
            class: "{radio_class}",
            "aria-checked": "{is_checked}",
            "data-state": "{state}",
            disabled: disabled,
            onclick: move |_| {
                if !disabled {
                    selected.set(value.clone());
                }
            },
            span { class: "flex justify-center items-center",
                if is_checked {
                    span { class: "rounded-full size-2.5 bg-primary" }
                }
            }
        }
    }
}
