+++
title = "Radio Button"
description = "Rust/UI component that displays a set of checkable buttons where only one can be selected at a time."
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<StaticRadioButton />

## Installation

<StaticInstallRadioButton />

## Usage

```rust
use crate::ui::radio_button::{RadioGroup, RadioGroupItem};
```

```rust
rsx! {
    RadioGroup { value,
        div { class: "flex gap-3 items-center",
            RadioGroupItem { value: "option-a", id: "option-a" }
            Label { html_for: "option-a", "Option A" }
        }
    }
}
```

## See Also

- [Label](/components/label)
- [Checkbox](/components/checkbox)
