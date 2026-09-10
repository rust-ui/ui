use app_config::{SeoMeta, SiteConfig};
use dioxus::prelude::*;
use registry::ui::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};

use crate::__registry__::demos_sidenav::{SIDENAV_ITEMS, SidenavItem};
use crate::__registry__::sidenav_hooks::SIDENAV_HOOKS_ITEMS;

#[component]
pub fn DocsComponentsIndexPage() -> Element {
    rsx! {
        DocsIndexPage {
            title: "Dioxus Components",
            description: "Beautiful Rust UI components for Dioxus applications. Copy-and-paste components to quickly build modern fullstack web apps.",
            canonical_url: format!("{}/docs/components", SiteConfig::BASE_URL),
            base_url: "/docs/components",
            items: SIDENAV_ITEMS,
        }
    }
}

#[component]
pub fn DocsHooksIndexPage() -> Element {
    rsx! {
        DocsIndexPage {
            title: "Dioxus Hooks",
            description: "Reusable Rust UI hooks for Dioxus applications. A collection of custom hooks for building fullstack web apps.",
            canonical_url: format!("{}/docs/hooks", SiteConfig::BASE_URL),
            base_url: "/docs/hooks",
            items: SIDENAV_HOOKS_ITEMS,
        }
    }
}

#[component]
fn DocsIndexPage(
    title: &'static str,
    description: &'static str,
    canonical_url: String,
    base_url: &'static str,
    items: &'static [SidenavItem],
) -> Element {
    let page_title = format!("{title} · Rust UI Components | {}", SiteConfig::TITLE);

    rsx! {
        SeoMeta {
            title: page_title,
            description: description.to_string(),
            canonical_url: canonical_url,
        }

        div { class: "flex flex-col gap-6 py-6 w-full sm:px-4 lg:max-w-[1100px]",
            div { class: "max-w-3xl max-sm:text-center text-pretty",
                h1 { class: "text-3xl font-bold tracking-tight lg:text-4xl text-pretty", "{title}" }
                p { class: "mt-2 mb-8 text-lg text-muted-foreground text-balance", "{description}" }
            }

            div { class: "grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3",
                for item in items {
                    a { href: format!("{}/{}", base_url, item.slug),
                        Card { class: "transition-all duration-200 hover:shadow-lg hover:scale-101",
                            CardContent {
                                if !item.image.is_empty() {
                                    img { src: item.image, alt: item.label, class: "w-full rounded-md dark:hidden" }
                                }
                                if !item.image_dark.is_empty() {
                                    img { src: item.image_dark, alt: item.label, class: "hidden w-full rounded-md dark:block" }
                                }

                                CardHeader { class: "px-0",
                                    CardTitle { class: "text-lg", "{item.label}" }
                                }

                                CardDescription { class: "line-clamp-2", "{item.description}" }
                            }
                        }
                    }
                }
            }

            SuggestComponentsCta {}
        }
    }
}

#[component]
fn SuggestComponentsCta() -> Element {
    rsx! {
        div { class: "py-16 text-center border-t",
            h2 { class: "mb-6 text-2xl font-semibold text-foreground",
                "Didn't find what you were looking for?"
            }
            a {
                href: "https://github.com/rust-ui/labs/discussions/categories/suggestions",
                target: "_blank",
                rel: "noopener noreferrer",
                class: "inline-flex justify-center items-center py-2 px-6 text-sm font-medium rounded-md shadow transition-colors focus-visible:ring-1 focus-visible:outline-none bg-primary text-primary-foreground hover:bg-primary/90 focus-visible:ring-ring",
                "Suggest component"
            }
        }
    }
}
