+++
title = "Hover Card"
description = "Rust/UI component that displays rich content in a floating card when hovering over a trigger element."
tags = []
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<StaticHoverCard />

## Installation

<StaticInstallHoverCard />

## Usage

```rust
use crate::ui::hover_card::{HoverCard, HoverCardTrigger, HoverCardContent};
```

```rust
rsx! {
    HoverCard {
        HoverCardTrigger {
            Button { variant: ButtonVariant::Link, "@username" }
        }
        HoverCardContent {
            p { "Card content" }
        }
    }
}
```

## Examples

### RTL

<StaticHoverCardRtl />

## See Also

- [Button](/components/button)
- [Avatar](/components/avatar)
