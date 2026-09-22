use dioxus::prelude::*;
use icons::ExternalLink;
use registry::ui::navigation_menu::{
    NavigationMenu, NavigationMenuContent, NavigationMenuItem, NavigationMenuLink, NavigationMenuList,
    NavigationMenuTrigger,
};

use crate::Route;
use crate::utils::assets::{LEPTOS_LOGO, LOGO_SQUARE_DARK, LOGO_SQUARE_LIGHT, RUSTIFY_LOGO};

const LEPTOS_URL: &str = "https://leptos.rust-ui.com";
const RUSTIFY_URL: &str = "https://www.rustify.rs";

const NAV_LINK_CLASS: &str =
    "inline-flex items-center px-2.5 py-1.5 h-8 text-sm font-medium rounded-md transition-colors hover:bg-accent group";
// text-shadow trick: a same-colored duplicate of the label sits one line-height below the
// visible text. On hover the span translates up by that same offset, so the shadow copy
// slides into view in place of the original, an always-on-hand "slide up" effect from a
// single span (no duplicated markup needed).
const NAV_LINK_TEXT_CLASS: &str = "inline-block transition-transform duration-300 ease-out group-hover:-translate-y-[1.2em] [text-shadow:0_1.2em_0_var(--foreground)]";

#[component]
pub fn NavDesktop() -> Element {
    let route = use_route::<Route>();
    let is_get_started = match &route {
        Route::ComponentPage { name } => {
            matches!(name.as_str(), "introduction" | "installation" | "cli" | "icons" | "figma" | "changelog" | "rtl")
        }
        _ => false,
    };
    let components_text_class =
        if matches!(route, Route::DocsComponentsIndexPage {} | Route::ComponentPage { .. }) && !is_get_started {
            "text-foreground"
        } else {
            "text-muted-foreground"
        };
    let hooks_text_class = if matches!(route, Route::DocsHooksIndexPage {} | Route::HookPage { .. }) {
        "text-foreground"
    } else {
        "text-muted-foreground"
    };

    rsx! {
        div { class: "hidden gap-0 items-center md:flex",
            Link {
                class: "inline-flex items-center py-1.5 px-1 rounded-md",
                to: Route::Home {},
                img {
                    src: LOGO_SQUARE_DARK,
                    alt: "Logo Rust/UI",
                    class: "hidden dark:block size-6",
                }
                img {
                    src: LOGO_SQUARE_LIGHT,
                    alt: "Logo Rust/UI",
                    class: "dark:hidden size-6",
                }
            }
            NavigationMenu { class: "relative z-auto max-w-none flex-none",
                NavigationMenuList { class: "gap-0",
                    NavigationMenuItem {
                        NavigationMenuTrigger {
                            class: "gap-1 px-1.5 h-8 text-base font-medium bg-transparent border-none shadow-none hover:bg-transparent hover:text-foreground data-[state=open]:bg-transparent",
                            "Rust/UI"
                        }
                        NavigationMenuContent { class: "md:w-screen p-4", is_full_width: true, should_blur: true,
                            div { class: "flex gap-6 mx-auto max-w-7xl",
                                div { class: "flex flex-col gap-2 w-[320px]",
                                    span { class: "text-xs font-medium text-muted-foreground", "Latest" }
                                    NavigationMenuLink {
                                        href: "/",
                                        class: "flex relative flex-col gap-2 p-3 w-full rounded-md border hover:bg-accent hover:text-accent-foreground",
                                        div { class: "flex justify-center items-center rounded-md size-9 bg-muted",
                                            img {
                                                src: LOGO_SQUARE_DARK,
                                                alt: "Logo Rust/UI",
                                                class: "hidden dark:block size-5",
                                            }
                                            img {
                                                src: LOGO_SQUARE_LIGHT,
                                                alt: "Logo Rust/UI",
                                                class: "dark:hidden size-5",
                                            }
                                        }
                                        span { class: "text-sm font-medium", "Rust/UI" }
                                        span { class: "text-xs text-muted-foreground",
                                            "Reusable components for Dioxus and Rust fullstack apps"
                                        }
                                        ExternalLink { class: "absolute top-3 right-3 size-3.5 text-muted-foreground" }
                                    }
                                }
                                div { class: "flex flex-col gap-2 min-w-[120px]",
                                    span { class: "text-xs font-medium text-muted-foreground", "Ecosystem" }
                                    NavigationMenuLink {
                                        href: RUSTIFY_URL,
                                        target: "_blank",
                                        rel: "noopener noreferrer",
                                        class: "gap-1.5 px-2 py-1.5 w-full rounded-sm group hover:bg-accent hover:text-accent-foreground",
                                        img { src: RUSTIFY_LOGO, alt: "Rustify", class: "size-4 rounded-full" }
                                        "Rustify"
                                        ExternalLink { class: "size-3 text-muted-foreground transition-transform duration-200 group-hover:translate-x-0.5 group-hover:-translate-y-0.5" }
                                    }
                                    NavigationMenuLink {
                                        href: LEPTOS_URL,
                                        target: "_blank",
                                        rel: "noopener noreferrer",
                                        class: "gap-1.5 px-2 py-1.5 w-full rounded-sm group hover:bg-accent hover:text-accent-foreground",
                                        img { src: LEPTOS_LOGO, alt: "Leptos", class: "size-4 rounded-full" }
                                        "Leptos UI"
                                        ExternalLink { class: "size-3 text-muted-foreground transition-transform duration-200 group-hover:translate-x-0.5 group-hover:-translate-y-0.5" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            span { "aria-hidden": "true", class: "mx-1 select-none text-muted-foreground/50", "/" }
            Link { class: NAV_LINK_CLASS, to: Route::DocsComponentsIndexPage {},
                span { class: "overflow-hidden inline-block h-[1.2em] leading-[1.2em]",
                    span { class: "{NAV_LINK_TEXT_CLASS} {components_text_class}", "Components" }
                }
            }
            Link { class: NAV_LINK_CLASS, to: Route::DocsHooksIndexPage {},
                span { class: "overflow-hidden inline-block h-[1.2em] leading-[1.2em]",
                    span { class: "{NAV_LINK_TEXT_CLASS} {hooks_text_class}", "Hooks" }
                }
            }
            Link { class: NAV_LINK_CLASS, to: Route::PageIcons {},
                span { class: "overflow-hidden inline-block h-[1.2em] leading-[1.2em]",
                    span { class: "{NAV_LINK_TEXT_CLASS} text-muted-foreground", "Icons" }
                }
            }
            Link { class: NAV_LINK_CLASS, to: Route::LoginBlocks {},
                span { class: "overflow-hidden inline-block h-[1.2em] leading-[1.2em]",
                    span { class: "{NAV_LINK_TEXT_CLASS} text-muted-foreground", "Blocks" }
                }
            }
            Link { class: NAV_LINK_CLASS, to: Route::AreaChartPage {},
                span { class: "overflow-hidden inline-block h-[1.2em] leading-[1.2em]",
                    span { class: "{NAV_LINK_TEXT_CLASS} text-muted-foreground", "Charts" }
                }
            }
        }
    }
}
