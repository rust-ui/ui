use dioxus::prelude::*;

use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::scroll_area::ScrollArea;
use crate::ui::separator::Separator;

#[component]
pub fn DemoScrollAreaRtl() -> Element {
    let tags = (0..=50_u32)
        .rev()
        .map(|i| format!("v1.2.0-beta.{i}"))
        .collect::<Vec<_>>();

    rsx! {
        DirectionProvider { dir: Direction::Rtl, class: "max-w-fit",
            ScrollArea { class: "w-48 h-72 rounded-md border",
                div { class: "p-4",
                    h4 { class: "mb-4 text-sm font-medium leading-none", "الإصدارات" }
                    for tag in tags {
                        div { class: "text-sm", "{tag}" }
                        Separator { class: "my-2" }
                    }
                }
            }
        }
    }
}
