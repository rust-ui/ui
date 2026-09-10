+++
title = "Radio Button Group"
description = "Rust/UI component that displays a group of radio buttons."
tags = []
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<StaticRadioButtonGroup />

## Installation

<StaticInstallRadioButtonGroup />

## Usage

```rust
use crate::ui::radio_button_group::{RadioButtonGroup, RadioButton, RadioButtonText};
```

```rust
rsx! {
    RadioButtonGroup {
        RadioButton { checked: true, RadioButtonText { "Option A" } }
        RadioButton { RadioButtonText { "Option B" } }
        RadioButton { RadioButtonText { "Option C" } }
    }
}
```

## Examples

### RTL

<StaticRadioButtonGroupRtl />

## See Also

- [Radio Button](/components/radio-button)
- [Checkbox](/components/checkbox)
