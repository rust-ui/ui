use dioxus::prelude::*;

use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::progress::Progress;

#[component]
pub fn DemoProgressRtl() -> Element {
    rsx! {
        DirectionProvider { dir: Direction::Rtl, class: "w-full max-w-sm",
            div { class: "flex flex-col gap-4 w-full max-w-sm",
                p { class: "text-sm text-muted-foreground", "تقدم التحميل" }
                Progress { value: 60.0 }
            }
        }
    }
}
