use dioxus::prelude::*;

use crate::ui::badge::Badge;
use crate::ui::direction_provider::{Direction, DirectionProvider};

#[component]
pub fn DemoBadgeRtl() -> Element {
    rsx! {
        DirectionProvider { dir: Direction::Rtl, class: "max-w-fit",
            Badge { "افتراضي" }
        }
    }
}
