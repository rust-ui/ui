use dioxus::prelude::*;
use icons::{ArrowUp, Paperclip, WandSparkles};

use crate::ui::input_group::{InputGroupButton, InputGroupButtonSize};
use crate::ui::input_prompt::{
    InputPrompt, InputPromptFooter, InputPromptSubmit, InputPromptTextarea, InputPromptTools,
};

#[component]
pub fn DemoInputPromptWithTools() -> Element {
    let mut value = use_signal(String::new);
    let mut is_loading = use_signal(|| false);

    let on_submit = move |_| {
        let text = value.peek().trim().to_string();
        if text.is_empty() || is_loading() {
            return;
        }
        value.set(String::new());
        is_loading.set(true);
        // simulate async response via web_sys timeout
        if let Some(window) = web_sys::window() {
            use wasm_bindgen::JsCast;
            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                wasm_bindgen::closure::Closure::once_into_js(move || {
                    is_loading.set(false);
                })
                .unchecked_ref(),
                1500,
            );
        }
    };

    let is_submit_disabled = value().trim().is_empty() || is_loading();

    rsx! {
        div { class: "w-full max-w-lg",
            InputPrompt {
                InputPromptTextarea { value, placeholder: "Ask a question...", on_submit }
                InputPromptFooter {
                    InputPromptTools {
                        InputGroupButton { size: InputGroupButtonSize::IconXs,
                            Paperclip { }
                        }
                        InputGroupButton { size: InputGroupButtonSize::IconXs,
                            WandSparkles { }
                        }
                    }
                    InputPromptSubmit { disabled: is_submit_disabled,
                        ArrowUp { }
                    }
                }
            }
        }
    }
}
