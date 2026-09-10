use dioxus::prelude::*;

use crate::Route;
use crate::utils::assets::{LOGO_SQUARE_DARK, LOGO_SQUARE_LIGHT};

#[component]
pub fn NavDesktop() -> Element {
    let route = use_route::<Route>();
    let is_get_started = match &route {
        Route::ComponentPage { name } => {
            matches!(
                name.as_str(),
                "introduction" | "installation" | "cli" | "icons" | "figma" | "changelog" | "rtl"
            )
        }
        _ => false,
    };
    let components_class =
        if matches!(route, Route::DocsComponentsIndexPage {} | Route::ComponentPage { .. }) && !is_get_started {
            "inline-flex items-center py-1.5 px-2.5 text-sm rounded-md hover:bg-accent bg-accent"
        } else {
            "inline-flex items-center py-1.5 px-2.5 text-sm rounded-md hover:bg-accent"
        };
    let hooks_class = if matches!(route, Route::DocsHooksIndexPage {} | Route::HookPage { .. }) {
        "inline-flex items-center py-1.5 px-2.5 text-sm rounded-md hover:bg-accent bg-accent"
    } else {
        "inline-flex items-center py-1.5 px-2.5 text-sm rounded-md hover:bg-accent"
    };

    rsx! {
        div { class: "hidden gap-0 items-center md:flex",
            Link {
                class: "inline-flex items-center py-1.5 px-2.5 text-sm rounded-md hover:bg-accent",
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
            Link {
                class: "{components_class}",
                to: Route::DocsComponentsIndexPage {},
                "Components"
            }
            Link {
                class: "{hooks_class}",
                to: Route::DocsHooksIndexPage {},
                "Hooks"
            }
            Link {
                class: "inline-flex items-center py-1.5 px-2.5 text-sm rounded-md hover:bg-accent",
                to: Route::PageIcons {},
                "Icons"
            }
            Link {
                class: "inline-flex items-center py-1.5 px-2.5 text-sm rounded-md hover:bg-accent",
                to: Route::LoginBlocks {},
                "Blocks"
            }
            Link {
                class: "inline-flex items-center py-1.5 px-2.5 text-sm rounded-md hover:bg-accent",
                to: Route::AreaChartPage {},
                "Charts"
            }
        }
    }
}
