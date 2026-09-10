use dioxus::prelude::*;

use crate::ui::button::Button;
use crate::ui::direction_provider::{Direction, DirectionProvider};

#[component]
pub fn DemoButtonRtl() -> Element {
    rsx! {
        DirectionProvider { dir: Direction::Rtl, class: "max-w-fit",
            Button { "زر" }
        }
    }
}
