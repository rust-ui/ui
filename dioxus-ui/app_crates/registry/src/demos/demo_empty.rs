use dioxus::prelude::*;

use crate::ui::button::{Button, ButtonVariant};
use crate::ui::empty::{Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyMediaVariant, EmptyTitle};

#[component]
pub fn DemoEmpty() -> Element {
    rsx! {
        div { class: "w-full max-w-md",
            Empty {
                EmptyHeader {
                    EmptyMedia { variant: EmptyMediaVariant::Icon,
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            class: "size-6",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "1.5",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "M2 7a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V7Z" }
                            path { d: "M16 3v4M8 3v4" }
                            path { d: "M3 11h18" }
                        }
                    }
                    EmptyTitle { "No Projects Yet" }
                    EmptyDescription {
                        "You haven't created any projects yet. Get started by creating your first project."
                    }
                }
                EmptyContent {
                    Button { "Create Project" }
                    Button { variant: ButtonVariant::Outline, "Import" }
                }
            }
        }
    }
}
