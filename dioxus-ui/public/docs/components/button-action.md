+++
title = "Button Action"
description = "A button that requires press-and-hold to activate, showing a progress indicator."
is_new = false
image = "/images/thumbnails/button.webp"
image_dark = "/images/thumbnails/button-dark.webp"
+++

<StaticButtonAction />

## Installation

<StaticInstallButtonAction />

## Usage

```rust
use crate::ui::button_action::ButtonAction;
```

```rust
rsx! {
    ButtonAction { on_complete: move |_| {}, "Hold to confirm" }
}
```

## Examples

### Hold To Confirm

Requires the user to hold the button before the action completes. Useful for destructive actions where an accidental click would be too risky.

<StaticButtonAction />

## See Also

- [Button](/components/button)
- [Pressable](/components/pressable)
