use dioxus::prelude::*;
use icons::{House, ScrollText, Settings};

use crate::app::Route;
use crate::components::hooks::use_is_current_path::use_is_current_path;
use crate::components::ui::bottom_nav::{BottomNav, BottomNavButton, BottomNavGrid, BottomNavLabel};
use crate::domain::home::routes::HomeRoutes;
use crate::domain::item::routing::ItemRoutes;
use crate::domain::settings::routes::SettingsRoutes;

#[component]
pub fn AppBottomNav() -> Element {
    let is_current = use_is_current_path();
    let navigator = use_navigator();

    rsx! {
        BottomNav {
            BottomNavGrid {
                BottomNavButton {
                    active: is_current("/"),
                    onclick: move |_| { navigator.push(Route::Home {}); },
                    House { class: "size-5" }
                    BottomNavLabel { {HomeRoutes::LABEL} }
                }
                BottomNavButton {
                    active: is_current("/items"),
                    onclick: move |_| { navigator.push(Route::ItemList {}); },
                    ScrollText { class: "size-5" }
                    BottomNavLabel { {ItemRoutes::LABEL} }
                }
                BottomNavButton {
                    active: is_current("/settings"),
                    onclick: move |_| { navigator.push(Route::Settings {}); },
                    Settings { class: "size-5" }
                    BottomNavLabel { {SettingsRoutes::LABEL} }
                }
            }
        }
    }
}
