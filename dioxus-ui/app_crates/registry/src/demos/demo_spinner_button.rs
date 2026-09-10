use dioxus::prelude::*;

use crate::ui::button::{Button, ButtonVariant};
use crate::ui::spinner::SpinnerCircle;

#[component]
pub fn DemoSpinnerButton() -> Element {
    rsx! {
        div { class: "flex flex-col gap-4 items-center",
            Button { disabled: true,
                SpinnerCircle {}
                span { "Loading..." }
            }
            Button { variant: ButtonVariant::Outline, disabled: true,
                SpinnerCircle {}
                span { "Please wait" }
            }
            Button { variant: ButtonVariant::Secondary, disabled: true,
                SpinnerCircle {}
                span { "Processing" }
            }
        }
    }
}
