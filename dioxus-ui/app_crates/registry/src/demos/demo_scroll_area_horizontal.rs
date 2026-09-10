use dioxus::prelude::*;

use crate::ui::scroll_area::ScrollArea;

#[component]
pub fn DemoScrollAreaHorizontal() -> Element {
    let images = (1..=5_u32).map(|i| format!("Image {i}")).collect::<Vec<_>>();

    rsx! {
        ScrollArea { class: "w-96 whitespace-nowrap rounded-md border",
            div { class: "flex gap-4 p-4 w-max",
                for label in images {
                    div { class: "shrink-0",
                        div { class: "overflow-hidden rounded-md",
                            div { class: "flex justify-center items-center text-sm h-[200px] w-[150px] bg-muted text-muted-foreground",
                                "{label}"
                            }
                        }
                    }
                }
            }
        }
    }
}
