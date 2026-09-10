use dioxus::prelude::*;

use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::textarea::Textarea;

#[component]
pub fn DemoTextareaRtl() -> Element {
    rsx! {
        DirectionProvider { dir: Direction::Rtl, class: "w-full max-w-sm",
            div {
                Textarea { placeholder: "اكتب رسالتك هنا." }
            }
        }
    }
}
