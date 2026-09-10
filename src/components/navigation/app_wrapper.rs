use dioxus::prelude::*;

#[component]
pub fn AppWrapper(children: Element) -> Element {
    rsx! {
        div { class: "flex flex-col h-full bottom__safe", {children} }
    }
}
