+++
title = "Label"
description = "Rust/UI component that displays a label for an input field."
tags = []
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<StaticLabel />

## Installation

<StaticInstallLabel />

## Usage

```rust
use crate::ui::label::Label;
```

```rust
rsx! {
    Label { html_for: "email", "Email address" }
}
```

## Examples

### With Input

Pair a label with an input for accessible form fields. The `html_for` prop links the label to the input via its `id`.

## See Also

- [Input](/components/input)
- [Checkbox](/components/checkbox)
