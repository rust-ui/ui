use dioxus::prelude::*;

use crate::ui::field::{Field, FieldContent, FieldDescription, FieldLabel, FieldTitle, FieldVariant};
use crate::ui::radio_button::{RadioGroup, RadioGroupItem};

#[component]
pub fn DemoRadioChoiceCard() -> Element {
    let selected = use_signal(|| "pro".to_string());

    rsx! {
        RadioGroup { value: selected, class: "w-full max-w-sm",
            FieldLabel { r#for: "plan-plus",
                Field { variant: FieldVariant::Horizontal,
                    FieldContent {
                        FieldTitle { "Plus" }
                        FieldDescription { "For individuals and small teams." }
                    }
                    RadioGroupItem { value: "plus", id: "plan-plus" }
                }
            }
            FieldLabel { r#for: "plan-pro",
                Field { variant: FieldVariant::Horizontal,
                    FieldContent {
                        FieldTitle { "Pro" }
                        FieldDescription { "For growing businesses." }
                    }
                    RadioGroupItem { value: "pro", id: "plan-pro" }
                }
            }
            FieldLabel { r#for: "plan-enterprise",
                Field { variant: FieldVariant::Horizontal,
                    FieldContent {
                        FieldTitle { "Enterprise" }
                        FieldDescription { "For large teams and enterprises." }
                    }
                    RadioGroupItem { value: "enterprise", id: "plan-enterprise" }
                }
            }
        }
    }
}
