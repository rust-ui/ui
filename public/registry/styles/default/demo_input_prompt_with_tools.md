---
title: "Demo Input Prompt With Tools"
name: "demo_input_prompt_with_tools"
cargo_dependencies: ["icons/leptos"]
registry_dependencies: ["input_group", "input_prompt"]
type: "components:demos"
path: "demos/demo_input_prompt_with_tools.rs"
---

# Demo Input Prompt With Tools

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_input_prompt_with_tools
```

## Component Code

```rust
use icons::{ArrowUp, Paperclip, WandSparkles};
use leptos::prelude::*;

use crate::components::ui::input_group::{InputGroupButton, InputGroupButtonSize};
use crate::components::ui::input_prompt::{
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
```
