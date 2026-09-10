+++
title = "Dialog"
description = "Rust/UI component that displays a modal dialog that the user can interact with."
tags = ["dialog"]
is_new = false
image = "/images/thumbnails/dialog.webp"
image_dark = "/images/thumbnails/dialog-dark.webp"
+++

<StaticDialog />

## Installation

<StaticInstallDialog />

## Usage

```rust
use crate::ui::dialog::{
    Dialog, DialogTrigger, DialogContent, DialogBody,
    DialogHeader, DialogTitle, DialogDescription,
    DialogFooter, DialogClose,
};
```

```rust
let mut open = use_signal(|| false);

rsx! {
    Dialog { open: open,
        DialogTrigger {
            onclick: move |_| open.set(true),
            "Open"
        }
        DialogContent { open: open,
            DialogBody {
                DialogHeader {
                    DialogTitle { "Edit profile" }
                    DialogDescription { "Make your changes below." }
                }
                // ... content ...
                DialogFooter {
                    DialogClose { open: open, "Cancel" }
                    Button { onclick: move |_| open.set(false), "Save" }
                }
            }
        }
    }
}
```

## Scrollable

<StaticDialogScrollable />

## See Also

- [Alert Dialog](/components/alert-dialog)
- [Collapsible](/components/collapsible)
