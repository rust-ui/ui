use dioxus::prelude::*;

use crate::ui::checkbox::Checkbox;
use crate::ui::form::{FormDescription, FormGroup, FormLabel, FormSeparator, FormSet};

#[component]
pub fn DemoFormGroup() -> Element {
    rsx! {
        div { class: "w-full max-w-md",
            FormGroup {
                FormSet {
                    FormLabel { "Responses" }
                    FormDescription {
                        "Get notified when ChatGPT responds to requests that take time, like research or image generation."
                    }
                    FormGroup { data_name: "CheckboxGroup",
                        div { class: "flex flex-row gap-3 items-center w-full",
                            Checkbox { checked: true, disabled: true }
                            FormLabel { class: "font-normal", "Push notifications" }
                        }
                    }
                }
                FormSeparator {}
                FormSet {
                    FormLabel { "Tasks" }
                    FormDescription {
                        "Get notified when tasks you've created have updates. "
                        a { href: "#", "Manage tasks" }
                    }
                    FormGroup { data_name: "CheckboxGroup",
                        div { class: "flex flex-row gap-3 items-center w-full",
                            Checkbox { }
                            FormLabel { class: "font-normal", "Push notifications" }
                        }
                        div { class: "flex flex-row gap-3 items-center w-full",
                            Checkbox { }
                            FormLabel { class: "font-normal", "Email notifications" }
                        }
                    }
                }
            }
        }
    }
}
