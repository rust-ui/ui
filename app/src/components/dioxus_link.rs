use leptos::prelude::*;

const DIOXUS_URL: &str = "https://dioxus.rust-ui.com";
const DIOXUS_LOGO: &str = "/images/logos/dioxus.png";

#[component]
pub fn DioxusLink() -> impl IntoView {
    view! {
        <a
            href=DIOXUS_URL
            target="_blank"
            rel="noopener noreferrer"
            aria-label="Dioxus"
            class="inline-flex items-center text-sm transition-colors text-muted-foreground hover:text-foreground"
        >
            <img src=DIOXUS_LOGO alt="Dioxus" class="size-5" aria-hidden="true" />
        </a>
    }
}
