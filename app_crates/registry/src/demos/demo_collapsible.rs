use dioxus::prelude::*;

use crate::ui::badge::Badge;
use crate::ui::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};

#[component]
pub fn DemoCollapsible() -> Element {
    rsx! {
        div { class: "w-full max-w-sm",
            Collapsible {
                div { class: "flex items-center justify-between px-3 py-2",
                    div { class: "flex items-center gap-2",
                        Badge { "@dioxus-ui" }
                        span { class: "text-sm font-medium", "3 repositories" }
                    }
                    CollapsibleTrigger {
                        svg {
                            class: "size-4",
                            xmlns: "http://www.w3.org/2000/svg",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "m6 9 6 6 6-6" }
                        }
                    }
                }
                div { class: "rounded-md border px-3 py-2 text-sm font-mono", "dioxus-ui/button" }
                CollapsibleContent {
                    div { class: "flex flex-col gap-1 mt-1",
                        div { class: "rounded-md border px-3 py-2 text-sm font-mono", "dioxus-ui/card" }
                        div { class: "rounded-md border px-3 py-2 text-sm font-mono", "dioxus-ui/input" }
                    }
                }
            }
        }
    }
}
