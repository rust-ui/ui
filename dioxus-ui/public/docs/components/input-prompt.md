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

## Usage

```rust
use crate::ui::input_prompt::InputPrompt;
```

```rust
rsx! {
    DemoInputPrompt {}
}
```

## Examples

### Basic Prompt

Prompt input surface for short text requests.

<StaticInputPrompt />

### Prompt With Tools

Prompt input combined with auxiliary actions or tools.

<StaticInputPromptWithTools />

## See Also

- [Input Group](/components/input_group)
- [Button](/components/button)
