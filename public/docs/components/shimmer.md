+++
title = "Shimmer"
description = "Auto-adapting skeleton loader that mirrors your DOM structure."
tags = []
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<StaticShimmer />

## Installation

<StaticInstallShimmer />

## Usage

```rust
use crate::ui::shimmer::Shimmer;
```

```rust
rsx! {
    Shimmer { loading: ReadSignal::new(loading),
        Card { "Content that shimmers while loading" }
    }
}
```

## See Also

- [Skeleton](/components/skeleton)
- [Spinner](/components/spinner)
