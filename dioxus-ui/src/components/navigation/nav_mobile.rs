use dioxus::prelude::*;

use crate::__registry__::demos_sidenav::SIDENAV_ITEMS;
use crate::__registry__::sidenav_hooks::SIDENAV_HOOKS_ITEMS;
use crate::Route;

#[component]
pub fn NavMobile() -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        div { class: "md:hidden",
            button {
                class: "inline-flex items-center justify-center p-2 rounded-md text-muted-foreground hover:bg-accent",
                "aria-label": "Open Menu",
                onclick: move |_| open.set(!open()),
                svg {
                    stroke_width: "1.5",
                    view_box: "0 0 24 24",
                    fill: "none",
                    xmlns: "http://www.w3.org/2000/svg",
                    class: "size-5",
                    path { d: "M3 5H11", stroke: "currentColor", stroke_width: "1.5", stroke_linecap: "round", stroke_linejoin: "round" }
                    path { d: "M3 12H16", stroke: "currentColor", stroke_width: "1.5", stroke_linecap: "round", stroke_linejoin: "round" }
                    path { d: "M3 19H21", stroke: "currentColor", stroke_width: "1.5", stroke_linecap: "round", stroke_linejoin: "round" }
                }
            }

            if open() {
                div { class: "absolute top-14 left-0 right-0 z-50 bg-background border-b border-border p-4 max-h-[calc(100vh-3.5rem)] overflow-y-auto",
                    div { class: "flex flex-col gap-4",
                        Link {
                            class: "inline-flex items-center py-1.5 px-2.5 text-sm rounded-md hover:bg-accent w-fit",
                            to: Route::Home {},
                            onclick: move |_| open.set(false),
                            img {
                                src: "/icons/logo-dark-square-48.webp",
                                alt: "Logo Rust/UI",
                                class: "hidden dark:block size-6",
                            }
                            img {
                                src: "/icons/logo-light-square-48.webp",
                                alt: "Logo Rust/UI",
                                class: "dark:hidden size-6",
                            }
                        }
                        div { class: "flex flex-col gap-1",
                            h2 { class: "text-xl", "Get Started" }
                            MobileNavLink { label: "Introduction", to: Route::ComponentPage { name: "introduction".to_string() }, onclose: move |_| open.set(false) }
                            MobileNavLink { label: "Installation", to: Route::ComponentPage { name: "installation".to_string() }, onclose: move |_| open.set(false) }
                            MobileNavLink { label: "CLI", to: Route::ComponentPage { name: "cli".to_string() }, onclose: move |_| open.set(false) }
                            MobileNavLink { label: "Icons", to: Route::PageIcons {}, onclose: move |_| open.set(false) }
                            MobileNavLink { label: "Blocks", to: Route::LoginBlocks {}, onclose: move |_| open.set(false) }
                            MobileNavLink { label: "Charts", to: Route::AreaChartPage {}, onclose: move |_| open.set(false) }
                        }
                        div { class: "flex flex-col gap-1",
                            h2 { class: "text-xl", "Components" }
                            for item in SIDENAV_ITEMS {
                                MobileNavLink { label: item.label, to: Route::ComponentPage { name: item.slug.to_string() }, onclose: move |_| open.set(false) }
                            }
                        }
                        div { class: "flex flex-col gap-1",
                            h2 { class: "text-xl", "Hooks" }
                            for item in SIDENAV_HOOKS_ITEMS {
                                MobileNavLink { label: item.label, to: Route::HookPage { name: item.slug.to_string() }, onclose: move |_| open.set(false) }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn MobileNavLink(label: &'static str, to: Route, onclose: EventHandler<MouseEvent>) -> Element {
    rsx! {
        Link {
            class: "py-2 px-3 text-sm rounded-md hover:bg-accent",
            to: to,
            onclick: move |evt| onclose.call(evt),
            "{label}"
        }
    }
}
