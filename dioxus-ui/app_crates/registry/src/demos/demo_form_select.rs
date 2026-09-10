use dioxus::prelude::*;

use crate::ui::form::{FormContent, FormDescription, FormLabel};
use crate::ui::select::{Select, SelectContent, SelectGroup, SelectOption, SelectTrigger, SelectValue};

const DEPARTMENTS: [&str; 8] =
    ["Engineering", "Design", "Marketing", "Sales", "Customer Support", "Human Resources", "Finance", "Operations"];

#[component]
pub fn DemoFormSelect() -> Element {
    rsx! {
        div { class: "w-full max-w-md",
            FormContent {
                FormLabel { "Department" }
                Select {
                    SelectTrigger {
                        SelectValue { placeholder: "Choose department" }
                    }

                    SelectContent {
                        SelectGroup {
                            for dept in DEPARTMENTS {
                                SelectOption { value: dept, "{dept}" }
                            }
                        }
                    }
                }
                FormDescription { "Select your department or area of work." }
            }
        }
    }
}
