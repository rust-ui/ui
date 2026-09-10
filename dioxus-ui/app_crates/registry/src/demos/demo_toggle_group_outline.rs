use dioxus::prelude::*;
use icons::{Bold, Italic, Underline};

use crate::ui::toggle_group::{ToggleGroup, ToggleGroupItem, ToggleGroupVariant};

#[component]
pub fn DemoToggleGroupOutline() -> Element {
    let mut bold = use_signal(|| false);
    let mut italic = use_signal(|| false);
    let mut underline = use_signal(|| false);

    rsx! {
        ToggleGroup { variant: ToggleGroupVariant::Outline,
            ToggleGroupItem { title: "Bold", pressed: bold(), onclick: move |_| bold.set(!bold()),
                Bold {}
            }
            ToggleGroupItem { title: "Italic", pressed: italic(), onclick: move |_| italic.set(!italic()),
                Italic {}
            }
            ToggleGroupItem { title: "Underline", pressed: underline(), onclick: move |_| underline.set(!underline()),
                Underline {}
            }
        }
    }
}
