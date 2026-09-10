+++
title = "Animate Group"
description = "A wrapper for animations, made with Tailwind CSS. Works seamlessly with any children component."
tags = ["animation"]
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<StaticAnimateGroup />

## Installation

<StaticInstallAnimateGroup />

## Usage

```rust
use crate::ui::animate::{AnimateGroup, AnimateGroupItem, AnimateVariant};
```

```rust
rsx! {
    AnimateGroup {
        AnimateGroupItem { variant: AnimateVariant::FadeUp, "First item" }
        AnimateGroupItem { variant: AnimateVariant::FadeUp, delay_ms: 150, "Second item" }
    }
}
```

## Examples

### Staggered Group

Animated group with staggered children entering in sequence. Useful for hero sections, onboarding content, and layered reveals.

<StaticAnimateGroup />

## See Also

- [Animate](/components/animate)
- [Button](/components/button)
