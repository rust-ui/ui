+++
title = "Avatar"
description = "Rust/UI component that displays an avatar with image and fallback support."
tags = []
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<StaticAvatar />

## Installation

<StaticInstallAvatar />

## Usage

```rust
use crate::ui::avatar::{Avatar, AvatarImage, AvatarFallback};
```

```rust
rsx! {
    Avatar {
        AvatarImage { src: "https://github.com/shadcn.png", alt: "@shadcn" }
        AvatarFallback { "CN" }
    }
}
```

## Examples

### Fallback

When the image fails to load or no `src` is provided, the fallback content is displayed instead.

### Sizes

Avatar supports three size variants: `Sm`, `Default`, and `Lg`.


### Group Count with Icon

<StaticAvatarGroupCountIcon />

## See Also

- [Badge](/components/badge)
- [Skeleton](/components/skeleton)
