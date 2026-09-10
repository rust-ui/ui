use dioxus::prelude::*;

use crate::ui::checkbox::Checkbox;
use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::label::Label;

#[component]
pub fn DemoCheckboxRtl() -> Element {
    let mut item1 = use_signal(|| true);
    let mut item2 = use_signal(|| false);

    rsx! {
        DirectionProvider { dir: Direction::Rtl, class: "w-full max-w-sm",
            div { class: "flex flex-col gap-4",
                div { class: "flex gap-2 items-center",
                    Checkbox {
                        checked: item1(),
                        on_checked_change: move |v| item1.set(v),
                    }
                    Label { "قبول الشروط والأحكام" }
                }
                div { class: "flex gap-2 items-center",
                    Checkbox {
                        checked: item2(),
                        on_checked_change: move |v| item2.set(v),
                    }
                    Label { "الاشتراك في النشرة البريدية" }
                }
                div { class: "flex gap-2 items-center",
                    Checkbox { checked: true, disabled: true }
                    Label { "معطّل (محدد)" }
                }
                div { class: "flex gap-2 items-center",
                    Checkbox { disabled: true }
                    Label { "معطّل" }
                }
            }
        }
    }
}
