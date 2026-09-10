use dioxus::prelude::*;

const LEPTOS_URL: &str = "https://rust-ui.com";
const LEPTOS_LOGO: &str = "/images/logos/leptos.svg";

#[component]
pub fn LeptosLink() -> Element {
    rsx! {
        a {
            href: LEPTOS_URL,
            target: "_blank",
            rel: "noopener noreferrer",
            "aria-label": "Leptos",
            class: "inline-flex items-center text-sm transition-colors text-muted-foreground hover:text-foreground",
            img {
                src: LEPTOS_LOGO,
                alt: "Leptos",
                class: "size-5",
                "aria-hidden": "true",
            }
        }
    }
}
