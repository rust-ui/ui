use dioxus::prelude::*;

#[component]
pub fn AppWrapper(children: Element) -> Element {
    rsx! {
        // Empty touchstart enables CSS :active on iOS
        div {
            class: "flex flex-col h-full bottom__safe",
            ontouchstart: |_| {},
            {children}
        }
    }
}
