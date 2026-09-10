use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::hooks::use_form::use_form;
use crate::ui::form::{
    Form, FormDescription, FormField, FormGroup, FormInput, FormLabel, FormLegend, FormLegendVariant, FormProvider,
    FormSet,
};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct FormData {
    street: String,
    city: String,
    zip: String,
}

#[component]
pub fn DemoFormFieldset() -> Element {
    let form = use_form::<FormData>();

    rsx! {
        FormProvider { form,
            Form { class: "max-w-md",
                FormSet {
                    FormLegend { variant: FormLegendVariant::Legend, "Address Information" }
                    FormDescription { "We need your address to deliver your order." }
                    FormGroup {
                        FormField { field: "street",
                            FormLabel { "Street Address" }
                            FormInput { placeholder: "123 Main St" }
                        }
                        div { class: "grid grid-cols-2 gap-4",
                            FormField { field: "city",
                                FormLabel { "City" }
                                FormInput { placeholder: "New York" }
                            }
                            FormField { field: "zip",
                                FormLabel { "Postal Code" }
                                FormInput { placeholder: "90502" }
                            }
                        }
                    }
                }
            }
        }
    }
}
