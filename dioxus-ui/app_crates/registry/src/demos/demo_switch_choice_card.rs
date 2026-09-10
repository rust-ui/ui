use dioxus::prelude::*;

use crate::ui::field::{Field, FieldContent, FieldDescription, FieldGroup, FieldLabel, FieldTitle, FieldVariant};
use crate::ui::switch::Switch;

#[component]
pub fn DemoSwitchChoiceCard() -> Element {
    rsx! {
        FieldGroup { class: "w-full max-w-sm",
            FieldLabel {
                Field { variant: FieldVariant::Horizontal,
                    FieldContent {
                        FieldTitle { "Share across devices" }
                        FieldDescription {
                            "Focus is shared across devices, and turns off when you leave the app."
                        }
                    }
                    Switch { id: "switch-share" }
                }
            }
            FieldLabel {
                Field { variant: FieldVariant::Horizontal,
                    FieldContent {
                        FieldTitle { "Enable notifications" }
                        FieldDescription {
                            "Receive notifications when focus mode is enabled or disabled."
                        }
                    }
                    Switch { id: "switch-notifications", checked: true }
                }
            }
        }
    }
}
