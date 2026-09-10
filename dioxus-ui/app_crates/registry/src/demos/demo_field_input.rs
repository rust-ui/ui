use dioxus::prelude::*;

use crate::ui::field::{Field, FieldDescription, FieldGroup, FieldLabel, FieldSet};
use crate::ui::input::Input;

#[component]
pub fn DemoFieldInput() -> Element {
    rsx! {
        FieldSet { class: "w-full max-w-xs",
            FieldGroup {
                Field {
                    FieldLabel { html_for: "username", "Username" }
                    Input { id: "username", placeholder: "Max Leiter" }
                    FieldDescription { "Choose a unique username for your account." }
                }
                Field {
                    FieldLabel { html_for: "password", "Password" }
                    FieldDescription { "Must be at least 8 characters long." }
                    Input { id: "password", r#type: crate::ui::input::InputType::Password, placeholder: "••••••••" }
                }
            }
        }
    }
}
