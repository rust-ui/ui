+++
title = "Field"
description = "Rust/UI components for composing accessible form fields with labels, descriptions, and error messages."
tags = ["input"]
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<StaticField />

## Installation

<StaticInstallField />

## Usage

```rust
use crate::ui::field::{Field, FieldGroup, FieldLabel, FieldDescription, FieldError};
```

```rust
rsx! {
    FieldGroup {
        Field {
            FieldLabel { html_for: "email", "Email" }
            Input { id: "email", placeholder: "you@example.com" }
            FieldDescription { "We'll never share your email." }
        }
    }
}
```

## Examples

### RTL

<StaticFieldRtl />

## See Also

- [Input](/components/input)
- [Label](/components/label)
- [Checkbox](/components/checkbox)
