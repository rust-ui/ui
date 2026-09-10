use dioxus::prelude::*;
use tw_merge::tw_merge;

#[component]
pub fn SwitchLabel(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("text-sm font-medium", class.as_deref().unwrap_or(""));
    rsx! { span { class: "{merged}", {children} } }
}

#[component]
pub fn Switch(
    #[props(into, optional)] id: Option<String>,
    #[props(default = false)] checked: bool,
    #[props(into, optional)] aria_label: Option<String>,
    #[props(into, optional)] class: Option<String>,
) -> Element {
    let mut is_checked = use_signal(|| checked);
    let state = if is_checked() { "checked" } else { "unchecked" };

    let track_class = tw_merge!(
        "inline-flex h-6 w-11 shrink-0 cursor-pointer items-center rounded-full border-2 border-transparent transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:bg-primary data-[state=unchecked]:bg-input",
        class.as_deref().unwrap_or("")
    );

    rsx! {
        button {
            "data-name": "Switch",
            id: id.as_deref().unwrap_or(""),
            r#type: "button",
            role: "switch",
            aria_checked: "{is_checked()}",
            aria_label: aria_label.as_deref().unwrap_or("Toggle switch"),
            "data-state": "{state}",
            class: "{track_class}",
            onclick: move |_| is_checked.set(!is_checked()),
            span {
                "data-state": "{state}",
                class: "block rounded-full ring-0 shadow-lg transition-transform pointer-events-none size-5 bg-background data-[state=checked]:translate-x-5 data-[state=unchecked]:translate-x-0",
            }
        }
    }
}
