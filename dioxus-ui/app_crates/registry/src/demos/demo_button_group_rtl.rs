use dioxus::prelude::*;

use crate::ui::button::{Button, ButtonVariant};
use crate::ui::button_group::ButtonGroup;
use crate::ui::direction_provider::{Direction, DirectionProvider};

#[component]
pub fn DemoButtonGroupRtl() -> Element {
    rsx! {
        DirectionProvider { dir: Direction::Rtl, class: "max-w-fit",
            ButtonGroup {
                Button { variant: ButtonVariant::Outline, "الأول" }
                Button { variant: ButtonVariant::Outline, "الثاني" }
                Button { variant: ButtonVariant::Outline, "الثالث" }
            }
        }
    }
}
