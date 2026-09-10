use dioxus::prelude::*;
use icons::{Minus, Plus};

use crate::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::ui::button_group::{ButtonGroup, ButtonGroupOrientation};

#[component]
pub fn DemoButtonGroupIcon() -> Element {
    rsx! {
        ButtonGroup { orientation: ButtonGroupOrientation::Vertical,
            Button { variant: ButtonVariant::Outline, size: ButtonSize::Icon,
                Plus {}
            }
            Button { variant: ButtonVariant::Outline, size: ButtonSize::Icon,
                Minus {}
            }
        }
    }
}
