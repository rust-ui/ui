use app_routes::BlockRoutes;
use dioxus::prelude::*;
use strum::IntoEnumIterator;

#[component]
pub fn BlocksHero() -> Element {
    rsx! {
        div { class: "flex flex-col gap-10",
            BlocksHeader {}
            BlocksNavigation {}
        }
    }
}

#[component]
fn BlocksNavigation() -> Element {
    rsx! {
        div { class: "relative lg:max-w-none max-w-[600px]",
            nav {
                "data-name": "__BlocksNav",
                class: "overflow-x-auto relative [-webkit-overflow-scrolling:touch] [scrollbar-width:none] [-ms-overflow-style:none] [&::-webkit-scrollbar]:hidden [touch-action:pan-x]",
                div { class: "flex gap-4 items-center",
                    for route in BlockRoutes::iter() {
                        a {
                            href: route.to_route(),
                            class: "flex justify-center items-center px-4 h-7 text-base font-medium text-center whitespace-nowrap shrink-0 capitalize",
                            {route.to_string()}
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn BlocksHeader() -> Element {
    rsx! {
        div { class: "flex flex-col gap-2 items-center py-8 text-center md:py-16 lg:py-20 xl:gap-4",
            h1 { class: "max-w-2xl text-4xl font-semibold tracking-tight lg:font-semibold xl:text-5xl xl:tracking-tighter text-primary leading-tighter text-balance lg:leading-[1.1]",
                "Dioxus Rust UI Blocks & Components"
            }
            p { class: "max-w-3xl text-base sm:text-lg text-foreground text-balance",
                "Beautiful Rust UI components for Dioxus applications. Ready-to-use blocks and components for modern web apps - copy and paste into your project."
            }
        }
    }
}
