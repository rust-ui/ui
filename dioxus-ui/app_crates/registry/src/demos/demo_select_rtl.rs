use dioxus::prelude::*;

use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::select::{Select, SelectContent, SelectGroup, SelectOption, SelectTrigger, SelectValue};

const OPTIONS: [(&str, &str); 3] = [("components", "المكونات"), ("extensions", "الإضافات"), ("icons", "الأيقونات")];

#[component]
pub fn DemoSelectRtl() -> Element {
    rsx! {
        DirectionProvider { dir: Direction::Rtl, class: "max-w-fit",
            Select {
                SelectTrigger { class: "w-[180px]",
                    SelectValue { placeholder: "اختر خياراً" }
                }

                SelectContent {
                    SelectGroup {
                        for (value, label) in OPTIONS {
                            SelectOption { value, "{label}" }
                        }
                    }
                }
            }
        }
    }
}
