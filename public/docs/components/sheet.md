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




## Components

The Sheet component is composed of several subcomponents:

- **Sheet**: Main wrapper component managing open/close state
- **SheetTrigger**: Button or element that opens the sheet
- **SheetContent**: Slide-in panel container with direction support; `show_close_button=false` hides the X button
- **SheetHeader**: Header section wrapping title and description
- **SheetTitle**: Primary heading text for the sheet
- **SheetDescription**: Secondary descriptive text
- **SheetBody**: Main content area for the sheet
- **SheetFooter**: Footer section for action buttons
- **SheetClose**: Button that closes the sheet
- **SheetDirection**: Enum for slide direction (Top/Bottom/Left/Right)



## Usage

```rust
use crate::components::ui::sheet::{
    Sheet,
    SheetClose,
    SheetContent,
    SheetDescription,
    SheetDirection,
    SheetFooter,
    SheetHeader,
    SheetTitle,
    SheetTrigger,
};
```

```rust
<Sheet>
    <SheetTrigger>"Open Sheet"</SheetTrigger>
    <SheetContent direction=SheetDirection::Right>
        <SheetHeader>
            <SheetTitle>"Sheet Title"</SheetTitle>
            <SheetDescription>"Sheet Description"</SheetDescription>
        </SheetHeader>
        <SheetFooter>
            <SheetClose variant=ButtonVariant::Outline>"Cancel"</SheetClose>
            <Button>"Save changes"</Button>
        </SheetFooter>
    </SheetContent>
</Sheet>
```





## Examples

### Directions

Sheet component that slides in from different directions including top, bottom, left, and right. This example demonstrates how to implement directional sheet animations in Leptos for creating versatile [Button](/docs/components/button) triggered drawer patterns and side panels in Rust applications.

<StaticSheetDirections />


### No Close Button

Hide the default X button by passing `show_close_button=false` to `SheetContent`. Use this when you want full control over dismissal via your own footer actions or the backdrop/ESC key.

<StaticSheetNoCloseButton />


### Experimental

Experimental sheet features with advanced positioning and animation behaviors. This example showcases cutting-edge sheet component capabilities in Leptos including nested sheets and custom transition effects for modern web applications.

<StaticSheetExperimental />


### RTL

Side sheet with Arabic content. Header, description, and footer buttons align to the right-to-left reading direction.

<StaticSheetRtl />

## See Also

- [Dialog](/docs/components/dialog)
- [Drawer](/docs/components/drawer)
