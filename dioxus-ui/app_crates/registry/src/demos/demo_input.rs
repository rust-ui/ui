use dioxus::prelude::*;

use crate::ui::input::{Input, InputType};

#[component]
pub fn DemoInput() -> Element {
    let mut value = use_signal(String::new);

    rsx! {
        div { class: "space-y-4 w-full max-w-lg",
            Input { placeholder: "Default input" }
            Input { r#type: InputType::Email, placeholder: "Email input" }
            Input { r#type: InputType::Password, placeholder: "Password input" }
            Input {
                class: "border-2 border-purple-500 focus:border-purple-700",
                placeholder: "Custom styled input",
            }
            Input { r#type: InputType::Number, placeholder: "Number input" }
            div { class: "pt-4 border-t",
                p { class: "mb-2 text-sm text-muted-foreground", "Two-way binding:" }
                Input {
                    placeholder: "Type here...",
                    oninput: move |e: FormEvent| value.set(e.value()),
                }
                if !value().is_empty() {
                    p { class: "mt-2 text-sm", "Value: {value}" }
                }
            }
        }
    }
}
