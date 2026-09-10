use dioxus::prelude::*;
use icons::{Moon, Sun};

use crate::components::hooks::use_theme_mode::use_theme_mode;
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};

#[component]
pub fn ThemeToggle() -> Element {
    let mut is_dark = use_theme_mode();

    rsx! {
        Button {
            variant: ButtonVariant::Ghost,
            size: ButtonSize::Icon,
            onclick: move |_| is_dark.toggle(),
            if is_dark() {
                Sun { class: "size-4" }
            } else {
                Moon { class: "size-4" }
            }
        }
    }
}
