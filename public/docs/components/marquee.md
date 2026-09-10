+++
title = "Marquee"
description = "Rust/UI component that displays an infinite scrolling component that can be used to display text, images, or videos."
tags = []
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<StaticMarquee />

## Installation

<StaticInstallMarquee />

## Usage

```rust
use crate::ui::marquee::{Marquee, MarqueeRow, MarqueeWrapper};
```

```rust
rsx! {
    MarqueeWrapper {
        Marquee {
            MarqueeRow {
                // Items repeat for seamless scroll
                div { "Card 1" }
                div { "Card 2" }
            }
        }
    }
}
```

## See Also

- [Mask](/components/mask)
- [Card](/components/card)
