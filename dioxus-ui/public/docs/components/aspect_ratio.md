+++
title = "Aspect Ratio"
description = "Displays content within a desired ratio."
tags = []
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<StaticAspectRatio />

## Installation

<StaticInstallAspectRatio />

## Usage

```rust
use crate::ui::aspect_ratio::AspectRatio;
```

```rust
rsx! {
    AspectRatio { ratio: 16.0 / 9.0,
        img { src: "...", alt: "..." }
    }
}
```

## See Also

- [Card](/components/card)
