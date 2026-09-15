use dioxus::prelude::*;
use icons::{Plus, Smartphone};
use registry::ui::button::{Button, ButtonSize};
use registry::ui::separator::{Separator, SeparatorOrientation};
use registry::ui::theme_toggle::ThemeToggle;

use crate::components::command_search_docs::CommandSearchDocs;
use crate::components::github_stars::GithubStars;
use crate::components::leptos_link::LeptosLink;
use crate::components::navigation::nav_desktop::NavDesktop;
use crate::components::navigation::nav_mobile::NavMobile;

const TESTFLIGHT_URL: &str = "https://testflight.apple.com/join/DF1hC7bQ";

#[component]
pub fn HeaderDocs() -> Element {
    rsx! {
        // TODO: temporary, remove once iOS TestFlight beta is out of public testing.
        a {
            href: TESTFLIGHT_URL,
            target: "_blank",
            rel: "noopener noreferrer",
            class: "flex sticky top-0 z-50 gap-1.5 justify-center items-center py-2 px-4 w-full text-sm text-center hover:underline bg-warning-light text-warning-dark underline-offset-2",
            Smartphone { class: "size-4 shrink-0" }
            "Rust/UI iOS beta is live! Join on TestFlight →"
        }
        header { class: "sticky top-9 z-50 w-full border-b border-border/40 backdrop-blur supports-[backdrop-filter]:bg-background/60",
            div { class: "container flex justify-between items-center px-3 md:px-6 h-14",
                nav { class: "flex flex-1 justify-between items-center",
                    div {
                        NavMobile {}
                        NavDesktop {}
                    }
                    div { class: "flex gap-2 items-center min-w-0",
                        CommandSearchDocs {}
                        Separator {
                            orientation: SeparatorOrientation::Vertical,
                            class: "hidden ml-2 h-4 lg:block",
                        }
                        LeptosLink {}
                        GithubStars {}
                        Separator {
                            orientation: SeparatorOrientation::Vertical,
                            class: "hidden h-4 lg:block",
                        }
                        ThemeToggle {}
                        Separator {
                            orientation: SeparatorOrientation::Vertical,
                            class: "hidden h-4 lg:block",
                        }
                        Button {
                            href: "/create",
                            size: ButtonSize::Sm,
                            class: "hidden rounded-xl md:inline-flex",
                            Plus {}
                            "New"
                        }
                    }
                }
            }
        }
    }
}
