---
title: "Demo Input Prompt"
name: "demo_input_prompt"
cargo_dependencies: ["icons/leptos"]
registry_dependencies: ["input_prompt"]
type: "components:demos"
path: "demos/demo_input_prompt.rs"
---

# Demo Input Prompt

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_input_prompt
```

## Component Code

```rust
use icons::ArrowUp;
use leptos::prelude::*;

use crate::components::ui::input_prompt::{InputPrompt, InputPromptFooter, InputPromptSubmit, InputPromptTextarea};

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
```
