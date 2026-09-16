use dioxus::prelude::*;

use crate::domain::test::editor::demo_editor::DemoEditor;

#[component]
pub fn TestPage() -> Element {
    rsx! {
        div { class: "flex flex-col gap-6",
            div { class: "flex flex-col gap-2",
                h2 { class: "text-xl font-semibold tracking-tight", "Rich text editor" }
                p { class: "text-sm text-muted-foreground",
                    "Composable contenteditable editor demo with formatting and lists."
                }
                DemoEditor {}
            }
        }
    }
}
