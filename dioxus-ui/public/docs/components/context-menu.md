+++
title = "Context Menu"
description = "Rust/UI component that displays a context menu on right-click."
tags = []
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<StaticContextMenu />

## Installation

<StaticInstallContextMenu />

## Usage

```rust
use crate::ui::context_menu::{ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuTrigger};
```

```rust
rsx! {
    DemoContextMenu {}
}
```

## Examples

### Basic Context Menu

Right-click menu for quick actions on a target element.

<StaticContextMenu />

### Action Menu

Context menu including an action that requires an additional hold-to-confirm step.

<StaticContextMenuAction />

### RTL

Context menu rendered in a right-to-left layout.

<StaticContextMenuRtl />

## See Also

- [Dropdown Menu](/components/dropdown_menu)
- [Command](/components/command)
