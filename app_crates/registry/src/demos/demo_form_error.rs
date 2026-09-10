use dioxus::prelude::*;

use crate::ui::form::{FormError, FormFieldWrapper, FormLabel};
use crate::ui::input::Input;

#[component]
pub fn DemoFormError() -> Element {
    rsx! {
        div { class: "w-full max-w-md",
            FormFieldWrapper { data_invalid: "true",
                FormLabel { html_for: "email", "Email" }
                Input { id: "email", r#type: crate::ui::input::InputType::Email }
                FormError { "Enter a valid email address." }
            }
        }
    }
}
