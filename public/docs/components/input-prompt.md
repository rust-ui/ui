+++
title = "Input Prompt"
description = "A compound input component for AI chat interfaces — combines an auto-growing textarea with a footer toolbar and a submit button."
tags = ["input"]
is_new = false
image = "/images/thumbnails/input.webp"
image_dark = "/images/thumbnails/input-dark.webp"
+++




<StaticInputPrompt />

## Installation

<StaticInstallInputPrompt />

## Components

The InputPrompt component is composed of several subcomponents:

- **InputPrompt**: Main wrapper (built on `InputGroup`)
- **InputPromptTextarea**: Auto-growing textarea bound to an `RwSignal<String>`. Enter sends, Shift+Enter inserts a newline
- **InputPromptFooter**: Block-end addon row — tools on the left, submit on the right
- **InputPromptTools**: Left-side slot for action buttons (attach, mode toggle, etc.)
- **InputPromptSubmit**: Send button with reactive `disabled` signal

## Usage

```rust
use crate::components::ui::input_prompt::{
    InputPrompt, InputPromptFooter, InputPromptSubmit, InputPromptTextarea, InputPromptTools,
};
```

```rust
let value = RwSignal::new(String::new());

<InputPrompt>
    <InputPromptTextarea
        value=value
        placeholder="Ask anything..."
        on_submit=Callback::new(move |_: ()| {
            value.set(String::new());
        })
    />
    <InputPromptFooter>
        <InputPromptTools>
            // tools go here
        </InputPromptTools>
        <InputPromptSubmit
            disabled=Signal::derive(move || value.get().trim().is_empty())
        >
            <ArrowUp />
        </InputPromptSubmit>
    </InputPromptFooter>
</InputPrompt>
```

## Examples

### With Tools

Full chat prompt with an attach button, a wand action, and a reactive send button that disables while loading.

<StaticInputPromptWithTools />

## See Also

- [Input Group](/docs/components/input-group)
- [Button](/docs/components/button)
- [Textarea](/docs/components/textarea)
