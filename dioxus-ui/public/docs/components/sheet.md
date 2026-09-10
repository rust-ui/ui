+++
title = "Sheet"
description = "Rust/UI component that displays a sheet."
tags = ["navigation"]
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<StaticSheet />

## Installation

<StaticInstallSheet />

## Usage

```rust
use crate::ui::sheet::{
    Sheet, SheetTrigger, SheetContent, SheetSide,
    SheetHeader, SheetTitle, SheetDescription,
    SheetBody, SheetFooter, SheetClose,
};
```

```rust
rsx! {
    Sheet {
        SheetTrigger { "Open" }
        SheetContent { side: SheetSide::Right,
            SheetHeader {
                SheetTitle { "Title" }
                SheetDescription { "Description" }
            }
            SheetBody { "Content here." }
            SheetFooter {
                SheetClose { "Cancel" }
            }
        }
    }
}
```

## See Also

- [Dialog](/components/dialog)
- [Drawer](/components/drawer)
