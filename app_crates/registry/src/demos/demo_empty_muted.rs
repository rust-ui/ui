use dioxus::prelude::*;

use crate::ui::empty::{Empty, EmptyDescription, EmptyHeader, EmptyTitle};

#[component]
pub fn DemoEmptyMuted() -> Element {
    rsx! {
        div { class: "w-full max-w-md",
            Empty {
                EmptyHeader {
                    EmptyTitle { "No results" }
                    EmptyDescription { "Try adjusting your search or filters." }
                }
            }
        }
    }
}
