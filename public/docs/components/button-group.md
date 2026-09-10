+++
title = "Button Group"
description = "A component that groups multiple buttons together with shared borders and styling."
tags = ["button"]
is_new = false
image = "/images/thumbnails/button.webp"
image_dark = "/images/thumbnails/button-dark.webp"
+++

# Button Group

## Installation

<StaticInstallButtonGroup />

## Usage

```rust
use crate::ui::button_group::ButtonGroup;
use crate::ui::button::{Button, ButtonVariant};
```

```rust
rsx! {
    ButtonGroup {
        Button { variant: ButtonVariant::Outline, "First" }
        Button { variant: ButtonVariant::Outline, "Second" }
        Button { variant: ButtonVariant::Outline, "Third" }
    }
}
```

## Examples

### With Separator

<StaticButtonGroupSeparator />

### With Icons

<StaticButtonGroupIcon />

### Sizes

<StaticButtonGroupSizes />

### With Input

<StaticButtonGroupInput />

### RTL

<StaticButtonGroupRtl />

## See Also

- [Button](/components/button)
- [Separator](/components/separator)
