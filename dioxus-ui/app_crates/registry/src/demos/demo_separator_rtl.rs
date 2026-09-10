use dioxus::prelude::*;

use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::separator::{Separator, SeparatorOrientation};

#[component]
pub fn DemoSeparatorRtl() -> Element {
    rsx! {
        DirectionProvider { dir: Direction::Rtl, class: "w-full max-w-sm",
            div { class: "flex flex-col gap-4",
                h3 { class: "text-2xl font-bold text-pretty", "Rust/UI" }
                p { "مكتبة مكونات واجهة مستخدم مفتوحة المصدر 🦀." }
                Separator {}
                div { class: "flex gap-4 items-center h-5",
                    p { "المدونة" }
                    Separator { orientation: SeparatorOrientation::Vertical }
                    p { "التوثيق" }
                    Separator { orientation: SeparatorOrientation::Vertical }
                    p { "المصدر" }
                }
            }
        }
    }
}
