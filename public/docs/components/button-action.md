+++
title = "Button Action"
description = "A button that requires press-and-hold to activate, showing a progress indicator."
is_new = true
image = "/images/thumbnails/button.webp"
image_dark = "/images/thumbnails/button-dark.webp"
+++




<StaticButtonAction />




## Installation

<StaticInstallButtonAction />



## Usage

```rust
use crate::registry::ui::button_action::ButtonAction;
```

```rust
<ButtonAction on_complete=on_complete>
    "Hold to Delete"
</ButtonAction>
```


## See Also

- [Button](/docs/components/button)
- [Button Group](/docs/components/button-group)
- [Pressable](/docs/components/pressable)
