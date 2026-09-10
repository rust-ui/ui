use dioxus::prelude::*;

use crate::app::Route;
use crate::components::layout::reload_button::ReloadButton;
use crate::components::layout::theme_toggle::ThemeToggle;
use crate::domain::home::routes::HomeRoutes;
use crate::domain::item::routing::ItemRoutes;
use crate::domain::settings::routes::SettingsRoutes;

#[component]
pub fn Header() -> Element {
    rsx! {
        // Mobile: reload + theme toggle absolute-positioned with safe-area offset for iOS notch
        div { class: "absolute right-8 sm:hidden top-[calc(env(safe-area-inset-top)+0.625rem)] z-100",
            ReloadButton {}
        }
        div { class: "absolute right-4 sm:hidden top-[calc(env(safe-area-inset-top)+1rem)] z-100",
            ThemeToggle {}
        }

        // Desktop header
        header { class: "hidden sm:flex h-14 items-center border-b px-6 gap-6",
            Link {
                to: Route::Home {},
                class: "font-semibold text-foreground leading-none",
                "Dioxus Fullstack"
            }

            nav { class: "flex items-center gap-4 text-sm text-muted-foreground",
                Link {
                    to: Route::Home {},
                    active_class: "text-foreground font-medium",
                    class: "hover:text-foreground transition-colors",
                    {HomeRoutes::LABEL}
                }
                Link {
                    to: Route::ItemList {},
                    active_class: "text-foreground font-medium",
                    class: "hover:text-foreground transition-colors",
                    {ItemRoutes::LABEL}
                }
                Link {
                    to: Route::Settings {},
                    active_class: "text-foreground font-medium",
                    class: "hover:text-foreground transition-colors",
                    {SettingsRoutes::LABEL}
                }
            }

            div { class: "ml-auto", ThemeToggle {} }
        }
    }
}
