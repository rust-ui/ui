use dioxus::prelude::*;

use crate::ui::auto_form::AutoForm;
use crate::ui::button::Button;

#[component]
pub fn DemoAutoForm() -> Element {
    rsx! {
        div { class: "flex flex-col gap-8 my-8 w-full max-w-4xl md:flex-row",
            div { class: "flex-1",
                AutoForm {
                    Button { button_type: "submit", "Submit" }
                }
            }

            div { class: "flex-1",
                h3 { class: "mb-2 text-sm font-medium whitespace-nowrap text-muted-foreground",
                    "Live Form Data"
                }
                pre { class: "overflow-auto p-4 max-h-80 font-mono text-sm rounded-lg bg-muted",
                    "Form submission data will appear here."
                }
            }
        }
    }
}
