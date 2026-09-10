use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::hooks::use_form::use_form;
use crate::ui::form::{Form, FormError, FormField, FormGroup, FormInput, FormLabel, FormProvider, FormSet};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct FormData {
    name: String,
    age: String,
    email: String,
}

#[component]
pub fn DemoFormValidation() -> Element {
    let form = use_form::<FormData>();

    rsx! {
        FormProvider { form,
            Form { class: "max-w-md",
                FormSet {
                    FormGroup {
                        FormField { field: "name",
                            FormLabel { "Name" }
                            FormInput { placeholder: "John Doe" }
                            FormError {}
                        }
                    }
                    FormGroup {
                        FormField { field: "age",
                            FormLabel { "Age" }
                            FormInput { r#type: "number", placeholder: "25" }
                            FormError {}
                        }
                    }
                    FormGroup {
                        FormField { field: "email",
                            FormLabel { "Email" }
                            FormInput { r#type: "email", placeholder: "example@email.com" }
                            FormError {}
                        }
                    }
                }
            }
        }
    }
}
