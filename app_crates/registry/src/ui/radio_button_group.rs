use dioxus::prelude::*;
use tw_merge::tw_merge;

const BASE: &str = "flex flex-wrap justify-center mt-2";

#[component]
pub fn RadioButtonText(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let c = tw_merge!(
        "block cursor-pointer bg-transparent text-primary px-3 py-1.5 relative ml-px shadow-[0_0_0_1px_#b5bfd9] tracking-wider text-center transition-colors duration-500",
        class.as_deref().unwrap_or("")
    );
    rsx! { span { "data-name": "RadioButtonText", class: "{c}", {children} } }
}

#[component]
pub fn RadioButtonGroup(children: Element) -> Element {
    let button_group_class =
        tw_merge!(BASE, "[&>label:first-child>span]:rounded-l-md [&>label:last-child>span]:rounded-r-md");
    rsx! {
        fieldset { class: "{BASE}",
            div { class: "{button_group_class}", role: "radio-button-group", {children} }
        }
    }
}

#[component]
pub fn RadioButton(#[props(optional)] checked: bool, children: Element) -> Element {
    rsx! {
        label {
            input {
                r#type: "radio",
                name: "radio",
                class: "radio__button focus:outline-0 focus:border-input/60",
                checked: checked,
            }
            {children}
        }
    }
}
