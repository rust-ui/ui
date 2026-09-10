+++
title = "Drawer"
description = "A Drawer for Rust. Inspired by the amazing work of Emil Kowalski."
tags = []
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<StaticDrawer />

## Installation

<StaticInstallDrawer />

## Usage

```rust
use crate::ui::drawer::{
    Drawer, DrawerTrigger, DrawerContent, DrawerHandle,
    DrawerHeader, DrawerTitle, DrawerDescription,
    DrawerBody, DrawerFooter, DrawerClose,
};
```

```rust
rsx! {
    Drawer {
        DrawerTrigger { "Open" }
        DrawerContent {
            DrawerHandle {}
            DrawerHeader {
                DrawerTitle { "Title" }
                DrawerDescription { "Description" }
            }
            DrawerBody { "Content here." }
            DrawerFooter {
                DrawerClose { "Cancel" }
            }
        }
    }
}
```

## See Also

- [Sheet](/components/sheet)
- [Dialog](/components/dialog)
