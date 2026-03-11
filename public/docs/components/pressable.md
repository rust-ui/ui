+++
title = "Pressable"
description = "Wrapper component that adds press feedback (scale effect) to any children."
tags = []
is_new = true
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<StaticPressable />

Wrapper component that adds press feedback with a subtle scale effect to any children. This is especially useful for non-button elements like [Card](/docs/components/card) or custom clickable containers, where the CSS `:active` pseudo-class does not work reliably on mobile devices. Use Pressable to provide tactile feedback and improve perceived interactivity in your Leptos applications.




## Installation

<StaticInstallPressable />




## Usage

```rust
use crate::components::ui::pressable::Pressable;
```

```rust
<Pressable>
    <Card class="p-4">
        "Press me!"
    </Card>
</Pressable>
```


## See Also

- [Button](/docs/components/button)
- [Button Action](/docs/components/button-action)
