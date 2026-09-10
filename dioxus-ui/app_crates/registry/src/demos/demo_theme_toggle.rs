use dioxus::prelude::*;

use crate::ui::theme_toggle::ThemeToggle;

#[component]
pub fn DemoThemeToggle() -> Element {
    rsx! {
        div { class: "flex items-center gap-4",
            ThemeToggle {}
            p { class: "text-sm text-muted-foreground", "Click to toggle light / dark mode" }
        }
    }
}
