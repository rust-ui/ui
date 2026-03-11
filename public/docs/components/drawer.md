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



## Components

The Drawer component is composed of several subcomponents:

- **Drawer**: Main wrapper component managing open/close state
- **DrawerTrigger**: Button or element that opens the drawer
- **DrawerContent**: Slide-in panel container with position support
- **DrawerHandle**: Draggable handle for gesture-based closing
- **DrawerBody**: Main content wrapper inside the drawer
- **DrawerTitle**: Primary heading text for the drawer
- **DrawerDescription**: Secondary descriptive text
- **DrawerClose**: Button that closes the drawer
- **DrawerPosition**: Enum for drawer position (Top/Bottom/Left/Right)
- **DrawerVariant**: Enum for visual style variants



## Usage

```rust
use crate::components::ui::drawer::{
    Drawer,
    DrawerBody,
    DrawerClose,
    DrawerContent,
    DrawerDescription,
    DrawerHandle,
    DrawerPosition,
    DrawerTitle,
    DrawerTrigger,
    DrawerVariant,
};
```

```rust
<Drawer>
    <DrawerTrigger>"Open Drawer"</DrawerTrigger>
    <DrawerContent>
        <DrawerHandle />
        <DrawerBody>
            <DrawerTitle>"Drawer Title"</DrawerTitle>
            <DrawerDescription>"Drawer description goes here."</DrawerDescription>
            <DrawerClose>"Close"</DrawerClose>
        </DrawerBody>
    </DrawerContent>
</Drawer>
```



## Examples

### Family

Nested drawer components demonstrating parent-child drawer relationships. This example showcases advanced drawer composition patterns in Leptos for building multi-level navigation and complex modal workflows in Rust applications.

<StaticDrawerFamily />

### Focus

Drawer with automatic focus management and keyboard navigation support. This example demonstrates accessibility best practices in Leptos for managing focus states and ensuring keyboard users can navigate drawer content efficiently.

<StaticDrawerFocus />

### Non Dismissable

Modal drawer that prevents dismissal until explicit user action is taken. This example shows how to create critical action drawers in Leptos that require user confirmation before closing for important workflows in Rust applications.

<StaticDrawerNonDismissable />

### Scrollable

Drawer with [ScrollArea](/docs/components/scroll-area) content area for displaying lengthy information. This example demonstrates overflow handling in Leptos drawers with proper scroll behavior and visual indicators for better content accessibility in Rust applications.

<StaticDrawerScrollable />

### Side

Side-positioned drawer that slides in from left or right viewport edges. This example showcases directional drawer positioning in Leptos for creating navigation panels and contextual sidebars in Rust web applications.

<StaticDrawerSide />

### Side Floating

Floating side drawer with elevated styling and shadow effects. This example demonstrates advanced drawer styling in Leptos with floating panels that overlay content while maintaining visual hierarchy in Rust applications.

<StaticDrawerSideFloating />

### Side Scrollable

Side drawer with internal [ScrollArea](/docs/components/scroll-area) content for extended navigation menus. This example combines directional positioning with scroll behavior in Leptos for building comprehensive sidebar interfaces in Rust applications.

<StaticDrawerSideScrollable />

### Responsive Dialog

Opens as a bottom Drawer on mobile and switches to a centered Dialog on desktop (≥ 768px). Uses CSS (`md:hidden` / `hidden md:block`) — no JS, no flash on reload.

<StaticDrawerDialog />

### RTL

Drawer with Arabic content. Handle and body text align to the right, following the RTL reading direction.

<StaticDrawerRtl />

## See Also

- [Dialog](/docs/components/dialog)
- [Sheet](/docs/components/sheet)
- [Alert Dialog](/docs/components/alert-dialog)
