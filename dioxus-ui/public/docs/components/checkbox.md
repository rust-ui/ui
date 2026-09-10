+++
title = "Checkbox"
description = "Rust/UI component that displays a control that allows the user to toggle between checked and not checked."
tags = ["utils"]
is_new = false
image = "/images/thumbnails/checkbox.webp"
image_dark = "/images/thumbnails/checkbox-dark.webp"
+++

<StaticCheckbox />

## Installation

<StaticInstallCheckbox />

## Usage

```rust
use crate::ui::checkbox::Checkbox;
```

```rust
rsx! {
    Checkbox {}
}
```

## Examples

### With Label

Combine the checkbox with a label for accessible form fields.

### Disabled

A disabled checkbox prevents user interaction while maintaining its visual state.

## See Also

- [Switch](/components/switch)
- [Label](/components/label)
