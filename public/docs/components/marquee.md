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






## Components

The Marquee component is composed of several subcomponents:

- **MarqueeWrapper**: Outer wrapper with overflow handling
- **Marquee**: Main animation container
- **MarqueeRow**: Row of scrolling content items



## Usage

```rust
use crate::components::ui::marquee::{Marquee, MarqueeRow, MarqueeWrapper};
```

```rust
<MarqueeWrapper>
    <Marquee>
        <MarqueeRow>
            <span>"Item 1"</span>
            <span>"Item 2"</span>
            <span>"Item 3"</span>
        </MarqueeRow>
        <MarqueeRow>
            <span>"Item 1"</span>
            <span>"Item 2"</span>
            <span>"Item 3"</span>
        </MarqueeRow>
    </Marquee>
</MarqueeWrapper>
```


## See Also

- [Animate](/docs/components/animate)
- [Card](/docs/components/card)
