use dioxus::prelude::*;
use icons::{Bold, Italic, Underline};

use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::toggle_group::{ToggleGroup, ToggleGroupItem};

#[component]
pub fn DemoToggleGroupRtl() -> Element {
    let mut bold = use_signal(|| false);
    let mut italic = use_signal(|| false);
    let mut underline = use_signal(|| false);

    rsx! {
        DirectionProvider { dir: Direction::Rtl, class: "max-w-fit",
            ToggleGroup {
                ToggleGroupItem { title: "غامق", pressed: bold(), onclick: move |_| bold.set(!bold()),
                    Bold {}
                }
                ToggleGroupItem { title: "مائل", pressed: italic(), onclick: move |_| italic.set(!italic()),
                    Italic {}
                }
                ToggleGroupItem { title: "تسطير", pressed: underline(), onclick: move |_| underline.set(!underline()),
                    Underline {}
                }
            }
        }
    }
}
