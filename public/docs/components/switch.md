+++
title = "Switch"
description = "Rust/UI component that displays a control that allows the user to toggle between checked and not checked."
tags = ["utils"]
is_new = false
image = "/images/thumbnails/switch.webp"
image_dark = "/images/thumbnails/switch-dark.webp"
+++

<StaticSwitch />

## Installation

<StaticInstallSwitch />

## Usage

```rust
use crate::ui::switch::Switch;
```

```rust
rsx! {
    Switch {}
}
```

## Examples

### With Label

Pair a switch with a label for a clear, accessible toggle control.

## See Also

- [Checkbox](/components/checkbox)
- [Label](/components/label)
