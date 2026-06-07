use icons::ArrowUp;
use leptos::prelude::*;

use crate::ui::input_prompt::{InputPrompt, InputPromptFooter, InputPromptSubmit, InputPromptTextarea};

#[component]
pub fn DemoInputPrompt() -> impl IntoView {
    let value = RwSignal::new(String::new());

    let on_submit = Callback::new(move |_: ()| {
        let text = value.get_untracked().trim().to_string();
        if !text.is_empty() {
            value.set(String::new());
        }
    });

    view! {
        <div class="w-full max-w-lg">
            <InputPrompt>
                <InputPromptTextarea value=value placeholder="Ask anything..." on_submit=on_submit />
                <InputPromptFooter>
                    <span class="px-1 text-xs text-muted-foreground">"Shift+Enter for new line"</span>
                    <InputPromptSubmit disabled=Signal::derive(move || value.get().trim().is_empty())>
                        <ArrowUp />
                    </InputPromptSubmit>
                </InputPromptFooter>
            </InputPrompt>
        </div>
    }
}
