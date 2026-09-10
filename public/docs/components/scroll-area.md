+++
title = "Scroll Area"
description = "Rust/UI component that provides custom scrolling functionality with cross-browser styling."
tags = ["utils"]
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<StaticScrollArea />

## Installation

<StaticInstallScrollArea />

## Usage

```rust
use crate::ui::scroll_area::ScrollArea;
```

```rust
rsx! {
    ScrollArea { class: "h-72",
        div { "Content..." }
    }
}
```

## Examples

### Horizontal

<StaticScrollAreaHorizontal />

### RTL

<StaticScrollAreaRtl />

## See Also

- [Separator](/components/separator)
- [Card](/components/card)
