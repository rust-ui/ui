use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::hooks::use_form::use_form;
use crate::ui::form::{Form, FormDescription, FormField, FormGroup, FormInput, FormLabel, FormProvider, FormSet};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct FormData {
    username: String,
    password: String,
}

#[component]
pub fn DemoForm() -> Element {
    let form = use_form::<FormData>();

    rsx! {
        FormProvider { form,
            Form { class: "max-w-md",
                FormSet {
                    FormGroup {
                        FormField { field: "username",
                            FormLabel { "Username" }
                            FormInput { placeholder: "Max Wells" }
                            FormDescription { "Choose a unique username for your account." }
                        }
                        FormField { field: "password",
                            FormLabel { "Password" }
                            FormDescription { "Must be at least 8 characters long." }
                            FormInput { r#type: "password", placeholder: "********" }
                        }
                    }
                }
            }
        }
    }
}
