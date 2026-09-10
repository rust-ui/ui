use dioxus::prelude::*;

use crate::ui::button::{Button, ButtonVariant};
use crate::ui::button_group::ButtonGroup;
use crate::ui::select::{Select, SelectContent, SelectGroup, SelectOption, SelectTrigger, SelectValue};

#[component]
pub fn DemoButtonGroupSelect() -> Element {
    rsx! {
        ButtonGroup {
            Select { default_value: "https",
                SelectTrigger {
                    SelectValue { placeholder: "Protocol" }
                }
                SelectContent {
                    SelectGroup {
                        SelectOption { value: "https", "https://" }
                        SelectOption { value: "http", "http://" }
                    }
                }
            }
            Button { variant: ButtonVariant::Outline, "Go" }
        }
    }
}
