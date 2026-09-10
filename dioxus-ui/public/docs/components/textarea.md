+++
title = "Textarea"
description = "Rust/UI component that displays a textarea."
tags = ["input"]
is_new = false
image = "/images/thumbnails/textarea.webp"
image_dark = "/images/thumbnails/textarea-dark.webp"
+++

<StaticTextarea />

## Installation

<StaticInstallTextarea />

## Usage

```rust
use crate::ui::textarea::Textarea;
```

```rust
rsx! {
    Textarea { placeholder: "Type your message here..." }
}
```

## Examples

### Disabled

A disabled textarea prevents user input and shows reduced opacity.

## See Also

- [Input](/components/input)
- [Label](/components/label)
