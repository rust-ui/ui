+++
title = "Alert Dialog"
description = "Rust/UI component that displays a modal dialog that interrupts the user with important content and expects a response."
tags = ["dialog"]
is_new = false
image = "/images/thumbnails/dialog.webp"
image_dark = "/images/thumbnails/dialog-dark.webp"
+++


<StaticAlertDialog />











## Installation

<StaticInstallAlertDialog />



## Components

The AlertDialog component is composed of several subcomponents:

- **AlertDialog**: Main wrapper component managing open/close state
- **AlertDialogTrigger**: Button or element that opens the alert dialog
- **AlertDialogContent**: Modal container with backdrop overlay
- **AlertDialogBody**: Main content wrapper inside the modal
- **AlertDialogHeader**: Header section for title and description
- **AlertDialogTitle**: Primary heading text for the alert
- **AlertDialogDescription**: Secondary descriptive text explaining the action
- **AlertDialogFooter**: Footer section for action buttons
- **AlertDialogClose**: Button that cancels and closes the dialog



## Usage

You can use the `AlertDialog` component in combination with the [Button](/docs/components/button) component.

```rust
use crate::components::ui::alert_dialog::{
    AlertDialog,
    AlertDialogBody,
    AlertDialogClose,
    AlertDialogContent,
    AlertDialogDescription,
    AlertDialogFooter,
    AlertDialogHeader,
    AlertDialogTitle,
    AlertDialogTrigger,
};
use crate::components::ui::button::Button;
```

```rust
<AlertDialog>
    <AlertDialogTrigger>"Open Alert"</AlertDialogTrigger>
    <AlertDialogContent>
        <AlertDialogBody>
            <AlertDialogHeader>
                <AlertDialogTitle>"Are you sure?"</AlertDialogTitle>
                <AlertDialogDescription>
                    "This action cannot be undone."
                </AlertDialogDescription>
            </AlertDialogHeader>
            <AlertDialogFooter>
                <AlertDialogClose>"Cancel"</AlertDialogClose>
                <Button attr:r#type="submit">"Continue"</Button>
            </AlertDialogFooter>
        </AlertDialogBody>
    </AlertDialogContent>
</AlertDialog>
```


## Examples

### With Media

Icon displayed above the title and description — use a `div` with `bg-muted` inside `AlertDialogHeader` before the title.

<StaticAlertDialogMedia />

### Small

Compact narrow dialog with centered content and side-by-side footer buttons — pass `class="w-[300px]"` to `AlertDialogContent` and `class="flex-row"` to `AlertDialogFooter`.

<StaticAlertDialogSmall />

### Small with Media

Combines compact layout with a centered icon above the content.

<StaticAlertDialogSmallMedia />



### RTL

Confirmation dialog with Arabic text. Button order and text alignment reflect the RTL layout direction.

<StaticAlertDialogRtl />

## See Also

- [Dialog](/docs/components/dialog)
- [Drawer](/docs/components/drawer)
- [Alert](/docs/components/alert)
