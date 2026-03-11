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



## Components

The Popover component is composed of several subcomponents:

- **Popover**: Main wrapper component managing open/close state
- **PopoverTrigger**: Button or element that opens the popover
- **PopoverContent**: Floating container for popover content
- **PopoverTitle**: Primary heading text for the popover
- **PopoverDescription**: Secondary descriptive text
- **PopoverAlign**: Enum for alignment (Start/Center/End/StartOuter/EndOuter)



## Usage

```rust
use crate::components::ui::popover::{
    Popover,
    PopoverAlign,
    PopoverContent,
    PopoverDescription,
    PopoverTitle,
    PopoverTrigger,
};
```

```rust
<Popover align=PopoverAlign::Center>
    <PopoverTrigger>"Open Popover"</PopoverTrigger>
    <PopoverContent>
        <PopoverTitle>"Popover Title"</PopoverTitle>
        <PopoverDescription>"Popover Description"</PopoverDescription>
    </PopoverContent>
</Popover>
```




## Examples

### Popover Aligned Start

Start-aligned popover that anchors to the left edge of the trigger [Button](/docs/components/button) element. This example demonstrates intelligent repositioning behavior with scroll detection, ensuring popovers remain visible and accessible in Leptos applications using dynamic positioning algorithms.

<StaticPopoverStart />


### Popover Aligned End

End-aligned popover that anchors to the right edge of the trigger [Button](/docs/components/button) element. This example showcases automatic viewport boundary detection in Leptos, adjusting popover position dynamically to prevent overflow and maintain optimal user experience.

<StaticPopoverEnd />


### Popover Aligned StartOuter

StartOuter-aligned popover that positions completely outside the left boundary of the trigger [Button](/docs/components/button). This example demonstrates advanced popover positioning in Rust where the popover's right edge aligns with the trigger's left edge for expanded layout options.

<StaticPopoverStartOuter />


### Popover Aligned EndOuter

EndOuter-aligned popover that positions completely outside the right boundary of the trigger [Button](/docs/components/button). This example shows flexible positioning patterns in Leptos where the popover's left edge aligns with the trigger's right edge for versatile UI compositions.

<StaticPopoverEndOuter />


### RTL

Popover with Arabic title and description. Content alignment and internal padding follow the RTL reading direction.

<StaticPopoverRtl />

## See Also

- [Tooltip](/docs/components/tooltip)
- [Dialog](/docs/components/dialog)
- [Dropdown Menu](/docs/components/dropdown-menu)
