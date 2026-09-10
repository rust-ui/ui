use dioxus::prelude::*;

use crate::ui::field::{Field, FieldDescription, FieldGroup, FieldLabel, FieldLegend, FieldSet};
use crate::ui::input::Input;

#[component]
pub fn DemoFieldFieldset() -> Element {
    rsx! {
        FieldSet { class: "w-full max-w-sm",
            FieldLegend { "Address Information" }
            FieldDescription { "We need your address to deliver your order." }
            FieldGroup {
                Field {
                    FieldLabel { html_for: "street", "Street Address" }
                    Input { id: "street", placeholder: "123 Main St" }
                }
                div { class: "grid grid-cols-2 gap-4",
                    Field {
                        FieldLabel { html_for: "city", "City" }
                        Input { id: "city", placeholder: "New York" }
                    }
                    Field {
                        FieldLabel { html_for: "zip", "Postal Code" }
                        Input { id: "zip", placeholder: "90502" }
                    }
                }
            }
        }
    }
}
