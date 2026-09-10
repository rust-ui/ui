use dioxus::prelude::*;
use icons::{AlignCenter, AlignLeft, AlignRight};

use crate::ui::toggle_group::{ToggleGroup, ToggleGroupItem, ToggleGroupOrientation};

#[component]
pub fn DemoToggleGroupVertical() -> Element {
    let mut align = use_signal(|| "left");

    rsx! {
        ToggleGroup { orientation: ToggleGroupOrientation::Vertical,
            ToggleGroupItem { title: "Align Left", pressed: align() == "left", onclick: move |_| align.set("left"),
                AlignLeft {}
                "Left"
            }
            ToggleGroupItem { title: "Align Center", pressed: align() == "center", onclick: move |_| align.set("center"),
                AlignCenter {}
                "Center"
            }
            ToggleGroupItem { title: "Align Right", pressed: align() == "right", onclick: move |_| align.set("right"),
                AlignRight {}
                "Right"
            }
        }
    }
}
