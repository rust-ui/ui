+++
title = "Pressable"
description = "Wrapper component that adds press feedback (scale effect) to any children."
tags = []
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<StaticPressable />

## Installation

<StaticInstallPressable />

## Usage

```rust
use crate::ui::pressable::Pressable;
```

```rust
rsx! {
    Pressable {
        Card { "Tap or click me" }
    }
}
```

## See Also

- [Button](/components/button)
- [Card](/components/card)
