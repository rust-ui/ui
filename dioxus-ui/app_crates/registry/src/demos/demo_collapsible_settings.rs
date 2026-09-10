use dioxus::prelude::*;
use icons::{Maximize, Minimize};

use crate::ui::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};
use crate::ui::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};
use crate::ui::input::Input;

#[component]
pub fn DemoCollapsibleSettings() -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        Card { class: "mx-auto w-full max-w-xs",
            CardHeader {
                CardTitle { "Radius" }
                CardDescription { "Set the corner radius of the element." }
            }
            CardContent {
                Collapsible { class: "flex gap-2 items-start",
                    div { class: "grid grid-cols-2 gap-2 w-full",
                        div { class: "flex flex-col gap-1",
                            label { class: "sr-only", r#for: "radius-tl", "Top Left" }
                            Input { id: "radius-tl", placeholder: "0", value: "0" }
                        }
                        div { class: "flex flex-col gap-1",
                            label { class: "sr-only", r#for: "radius-tr", "Top Right" }
                            Input { id: "radius-tr", placeholder: "0", value: "0" }
                        }
                        CollapsibleContent { class: "col-span-full grid gap-2 grid-cols-2",
                            div { class: "flex flex-col gap-1",
                                label { class: "sr-only", r#for: "radius-bl", "Bottom Left" }
                                Input { id: "radius-bl", placeholder: "0", value: "0" }
                            }
                            div { class: "flex flex-col gap-1",
                                label { class: "sr-only", r#for: "radius-br", "Bottom Right" }
                                Input { id: "radius-br", placeholder: "0", value: "0" }
                            }
                        }
                    }
                    CollapsibleTrigger { class: "inline-flex justify-center items-center rounded-md border transition-colors size-9 border-input shrink-0 hover:bg-accent hover:text-accent-foreground",
                        onclick: move |_| open.set(!open()),
                        if open() {
                            Minimize { class: "size-4" }
                        } else {
                            Maximize { class: "size-4" }
                        }
                    }
                }
            }
        }
    }
}
