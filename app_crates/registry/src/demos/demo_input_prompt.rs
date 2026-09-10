use dioxus::prelude::*;
use icons::ArrowUp;

use crate::ui::input_prompt::{InputPrompt, InputPromptFooter, InputPromptSubmit, InputPromptTextarea};

#[component]
pub fn DemoInputPrompt() -> Element {
    let mut value = use_signal(String::new);

    let on_submit = move |_| {
        let text = value.peek().trim().to_string();
        if !text.is_empty() {
            value.set(String::new());
        }
    };

    rsx! {
        div { class: "w-full max-w-lg",
            InputPrompt {
                InputPromptTextarea { value, placeholder: "Ask anything...", on_submit }
                InputPromptFooter {
                    span { class: "px-1 text-xs text-muted-foreground", "Shift+Enter for new line" }
                    InputPromptSubmit { disabled: value().trim().is_empty(),
                        ArrowUp { }
                    }
                }
            }
        }
    }
}
