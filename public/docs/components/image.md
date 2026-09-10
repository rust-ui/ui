+++
title = "Image"
description = "An image component with lazy loading and responsive support."
+++

<StaticImage />

## Installation

<StaticInstallImage />

## Usage

```rust
use crate::ui::image::Image;
```

```rust
rsx! {
    Image {
        src: "https://example.com/photo.jpg",
        alt: "A photo",
        width: 400,
        height: 300,
        class: "rounded-lg",
    }
}
```

## See Also

- [AspectRatio](/components/aspect-ratio)
- [Card](/components/card)
