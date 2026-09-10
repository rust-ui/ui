use dioxus::prelude::*;

use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::popover::{Popover, PopoverContent, PopoverDescription, PopoverTitle, PopoverTrigger};

#[component]
pub fn DemoPopoverRtl() -> Element {
    rsx! {
        DirectionProvider { dir: Direction::Rtl, class: "max-w-fit",
            Popover {
                PopoverTrigger { "فتح النافذة المنبثقة" }

                PopoverContent { class: "w-[300px]",
                    PopoverTitle { "نافذة منبثقة تجريبية" }

                    PopoverDescription {
                        "نافذة منبثقة تفاعلية تتكيف موضعها أثناء التمرير. جرّب التمرير لرؤية التمركز الذكي."
                    }
                }
            }
        }
    }
}
