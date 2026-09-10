use dioxus::prelude::*;

use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::input::Input;
use crate::ui::label::Label;

#[component]
pub fn DemoLabelRtl() -> Element {
    rsx! {
        DirectionProvider { dir: Direction::Rtl, class: "w-full max-w-sm",
            div { class: "flex flex-col gap-2",
                Label { html_for: "rtl-label-input", "اسم المستخدم" }
                Input { id: "rtl-label-input", placeholder: "أدخل اسم المستخدم" }
            }
        }
    }
}
