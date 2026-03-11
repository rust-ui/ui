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

```bash
Coming soon
```




## Components

The AnimateGroup component is composed of several subcomponents:

- **AnimateGroup**: Main wrapper component for staggered animations
- **AnimateGroupItem**: Individual animated item with delay support



## Usage

```rust
use crate::components::ui::animate::{AnimateGroup, AnimateGroupItem, AnimateVariant};
```

```rust
<AnimateGroup>
    <AnimateGroupItem variant=AnimateVariant::FadeUp delay_ms=0>
        <p>"First item"</p>
    </AnimateGroupItem>
    <AnimateGroupItem variant=AnimateVariant::FadeUp delay_ms=100>
        <p>"Second item"</p>
    </AnimateGroupItem>
    <AnimateGroupItem variant=AnimateVariant::FadeUp delay_ms=200>
        <p>"Third item"</p>
    </AnimateGroupItem>
</AnimateGroup>
```


## See Also

- [Animate](/docs/components/animate)
