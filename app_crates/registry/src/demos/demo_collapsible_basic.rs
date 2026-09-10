use dioxus::prelude::*;

use crate::ui::card::{Card, CardContent};
use crate::ui::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};

#[component]
pub fn DemoCollapsibleBasic() -> Element {
    rsx! {
        Card { class: "mx-auto w-full max-w-sm",
            CardContent {
                Collapsible { class: "rounded-md data-[state=open]:bg-muted",
                    CollapsibleTrigger { class: "flex gap-2 items-center py-2 px-3 w-full text-sm font-medium rounded-md transition-colors group hover:bg-accent hover:text-accent-foreground",
                        "Product details"
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            class: "ml-auto transition-transform duration-300 size-4 group-data-[state=open]:rotate-180",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "m6 9 6 6 6-6" }
                        }
                    }
                    CollapsibleContent { class: "flex flex-col gap-2 items-start p-2.5 pt-0 text-sm",
                        p { "This panel can be expanded or collapsed to reveal additional content." }
                    }
                }
            }
        }
    }
}
