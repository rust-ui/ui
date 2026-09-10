use app_routes::ChartRoutes;
use dioxus::prelude::*;

#[component]
pub fn ChartsHero() -> Element {
    rsx! {
        div { class: "flex flex-col gap-10",
            ChartsHeader {}
            ChartsNavigation {}
        }
    }
}

#[component]
fn ChartsNavigation() -> Element {
    rsx! {
        div { class: "relative lg:max-w-none max-w-[600px]",
            nav {
                "data-name": "__ChartsNav",
                class: "overflow-x-auto relative [-webkit-overflow-scrolling:touch] [scrollbar-width:none] [-ms-overflow-style:none] [&::-webkit-scrollbar]:hidden [touch-action:pan-x]",
                div { class: "flex gap-0 items-center",
                    for route in ChartRoutes::ALL.iter().copied() {
                        a {
                            href: route.to_route(),
                            class: "flex justify-center items-center px-4 h-7 text-base font-medium text-center whitespace-nowrap shrink-0",
                            {route.to_title()}
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ChartsHeader() -> Element {
    rsx! {
        div { class: "flex flex-col gap-2 items-center py-8 text-center md:py-16 lg:pt-20 xl:gap-4",
            h1 { class: "max-w-2xl text-4xl font-semibold tracking-tight lg:font-semibold xl:text-5xl xl:tracking-tighter text-primary leading-tighter text-balance lg:leading-[1.1]",
                "Dioxus Rust UI Charts & Graphs"
            }
            p { class: "max-w-3xl text-base sm:text-lg text-foreground text-balance",
                "Beautiful Dioxus Rust UI chart components. Ready-to-use charts and graphs for modern web apps - copy and paste into your Dioxus project."
            }
        }
    }
}
