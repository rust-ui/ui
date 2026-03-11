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



## Components

The Dialog component is composed of several subcomponents:

- **Dialog**: Main wrapper component managing open/close state
- **DialogTrigger**: Button or element that opens the dialog
- **DialogContent**: Modal container with backdrop overlay
- **DialogBody**: Main content wrapper inside the modal
- **DialogHeader**: Header section for title and description
- **DialogTitle**: Primary heading text for the dialog
- **DialogDescription**: Secondary descriptive text
- **DialogFooter**: Footer section for action buttons
- **DialogClose**: Button that closes the dialog
- **DialogAction**: Primary action button (e.g., confirm)



## Usage

You can use the `Dialog` component in combination with the [Button](/docs/components/button), [Input](/docs/components/input), [Label](/docs/components/label), and [ScrollArea](/docs/components/scroll-area) components.

```rust
use crate::components::ui::dialog::{
    Dialog,
    DialogAction,
    DialogBody,
    DialogClose,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
    DialogTrigger,
};
```

```rust
<Dialog>
    <DialogTrigger>"Open Dialog"</DialogTrigger>
    <DialogContent>
        <DialogBody>
            <DialogHeader>
                <DialogTitle>"Dialog Title"</DialogTitle>
                <DialogDescription>"Dialog Description"</DialogDescription>
            </DialogHeader>
            <DialogFooter>
                <DialogClose>"Cancel"</DialogClose>
                <DialogAction>"Confirm"</DialogAction>
            </DialogFooter>
        </DialogBody>
    </DialogContent>
</Dialog>
```




## Examples

### Scrollable Dialog

Modal dialog with [ScrollArea](/docs/components/scroll-area) overflow content for displaying lengthy text and forms. This example demonstrates how to implement accessible scrollable dialogs in Leptos with proper ARIA attributes and keyboard navigation support for improved usability.

<StaticDialogScrollable />


### Sticky Footer

Dialog with a scrollable body and a footer that stays pinned at the bottom. The middle content scrolls independently using `-mx-6 px-6` to bleed to the dialog edges, while `DialogFooter` remains fixed — ideal for long forms or multi-step flows.

<StaticDialogStickyFooter />


### RTL

Edit-profile dialog with Arabic labels and inputs. Layout, label alignment, and footer button order all adapt to RTL.

<StaticDialogRtl />

## See Also

- [Alert Dialog](/docs/components/alert-dialog)
- [Drawer](/docs/components/drawer)
- [Sheet](/docs/components/sheet)
- [Popover](/docs/components/popover)
