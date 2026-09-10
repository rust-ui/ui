+++
title = "Input Phone"
description = "Rust/UI component that displays a phone number input with country code selector and automatic formatting."
tags = ["input"]
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<StaticInputPhone />

## Installation

<StaticInstallInputPhone />

## Usage

```rust
use crate::ui::input_phone::InputPhone;
```

```rust
rsx! {
    InputPhone {}
}
```

## Examples

### Basic Phone Input

Phone number input with country picker and formatted display.

<StaticInputPhone />

### Disabled

Disabled state for read-only or unavailable phone input workflows.

<StaticInputPhoneDisabled />

## See Also

- [Input](/components/input)
- [Popover](/components/popover)
