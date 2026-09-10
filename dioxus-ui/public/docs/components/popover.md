+++
title = "Popover"
description = "Rust/UI component that displays rich content in a portal, triggered by a button."
tags = ["popover"]
is_new = false
image = "/images/thumbnails/popover.webp"
image_dark = "/images/thumbnails/popover-dark.webp"
+++

<StaticPopover />

## Installation

<StaticInstallPopover />

## Usage

```rust
use crate::ui::popover::{Popover, PopoverTrigger, PopoverContent, PopoverSide};
```

```rust
rsx! {
    Popover {
        PopoverTrigger { "Open" }
        PopoverContent {
            p { "Popover content here." }
        }
    }
}
```

## See Also

- [Hover Card](/components/hover-card)
- [Tooltip](/components/tooltip)
- [Dialog](/components/dialog)
