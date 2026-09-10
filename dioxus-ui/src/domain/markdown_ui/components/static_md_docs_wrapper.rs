use dioxus::prelude::*;

#[component]
pub fn StaticMdDocsWrapper(children: Element) -> Element {
    rsx! {
        div { class: "p-2 w-full rounded-md border",
            {children}
        }
    }
}
