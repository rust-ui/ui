use dioxus::prelude::*;

const LOGO_SRC: &str = "/icons/logo.png";
const SITE_URL: &str = "https://dioxus.rust-ui.com";
const SITE_LABEL: &str = "dioxus.rust-ui.com";
const APP_NAME: &str = "Dioxus Fullstack";
const APP_DESCRIPTION: &str = "Cross-platform starter — web, mobile, desktop from one codebase. Built with ";

#[component]
pub fn HomePage() -> Element {
    rsx! {
        div { class: "flex flex-col justify-center items-center px-6 h-full text-center",
            img {
                src: LOGO_SRC,
                alt: "Logo",
                class: "mb-6 rounded-2xl size-20",
            }
            h1 { class: "text-2xl font-bold tracking-tight mb-3", {APP_NAME} }
            p { class: "text-muted-foreground text-sm max-w-xs",
                {APP_DESCRIPTION}
                a {
                    href: SITE_URL,
                    target: "_blank",
                    class: "underline underline-offset-2 hover:text-foreground transition-colors",
                    {SITE_LABEL}
                }
                "."
            }
        }
    }
}
