use icons::{ArrowUp, Paperclip, WandSparkles};
use leptos::prelude::*;

use crate::ui::input_group::{InputGroupButton, InputGroupButtonSize};
use crate::ui::input_prompt::{
    InputPrompt, InputPromptFooter, InputPromptSubmit, InputPromptTextarea, InputPromptTools,
};

#[component]
pub fn DemoInputPromptWithTools() -> impl IntoView {
    let value = RwSignal::new(String::new());
    let is_loading = RwSignal::new(false);

    let on_submit = Callback::new(move |_: ()| {
        let text = value.get_untracked().trim().to_string();
        if text.is_empty() || is_loading.get_untracked() {
            return;
        }
        value.set(String::new());
        is_loading.set(true);
        // simulate async response
        set_timeout(move || is_loading.set(false), std::time::Duration::from_millis(1500));
    });

    let is_submit_disabled = Signal::derive(move || value.get().trim().is_empty() || is_loading.get());

    view! {
        <div class="w-full max-w-lg">
            <InputPrompt>
                <InputPromptTextarea value=value placeholder="Ask a question..." on_submit=on_submit />
                <InputPromptFooter>
                    <InputPromptTools>
                        <InputGroupButton size=InputGroupButtonSize::IconXs>
                            <Paperclip />
                        </InputGroupButton>
                        <InputGroupButton size=InputGroupButtonSize::IconXs>
                            <WandSparkles />
                        </InputGroupButton>
                    </InputPromptTools>
                    <InputPromptSubmit disabled=is_submit_disabled>
                        <ArrowUp />
                    </InputPromptSubmit>
                </InputPromptFooter>
            </InputPrompt>
        </div>
    }
}
