+++
title = "Animate"
description = "A wrapper for animations, made with Tailwind CSS. Works seamlessly with any children component."
tags = ["animation"]
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<StaticAnimate />

## Installation

<StaticInstallAnimate />

## Usage

```rust
use crate::ui::animate::{Animate, AnimateHoverVariant, AnimateVariant};
```

```rust
rsx! {
    Animate { hover_variant: AnimateHoverVariant::Wobble,
        span { "Hover me" }
    }
}
```

## Examples

### Staggered Group

```rust
use crate::ui::animate::{AnimateGroup, AnimateGroupItem, AnimateVariant};

rsx! {
    AnimateGroup {
        AnimateGroupItem { variant: AnimateVariant::FadeUp, delay_ms: 0, "First" }
        AnimateGroupItem { variant: AnimateVariant::FadeUp, delay_ms: 100, "Second" }
        AnimateGroupItem { variant: AnimateVariant::FadeUp, delay_ms: 200, "Third" }
    }
}
```

## See Also

- [Badge](/components/badge)
- [Card](/components/card)
