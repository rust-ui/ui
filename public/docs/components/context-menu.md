+++
title = "Context Menu"
description = "Rust/UI component that displays a context menu on right-click."
tags = []
is_new = true
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++


<StaticContextMenu />




## Installation

<StaticInstallContextMenu />



## Components

The ContextMenu component is composed of several subcomponents:

- **ContextMenu**: Main wrapper component managing open/close state
- **ContextMenuTrigger**: Area that triggers the context menu on right-click
- **ContextMenuContent**: Dropdown container for menu items
- **ContextMenuLabel**: Label text for menu sections
- **ContextMenuGroup**: Groups related menu items together
- **ContextMenuItem**: Individual menu item container
- **ContextMenuAction**: Clickable action within a menu item
- **ContextMenuSub**: Submenu wrapper component
- **ContextMenuSubTrigger**: Item that opens a submenu on hover
- **ContextMenuSubContent**: Submenu dropdown container



## Usage

You can use the `ContextMenu` component in combination with the [Separator](/docs/components/separator) component.

```rust
use crate::components::ui::context_menu::{
    ContextMenu,
    ContextMenuAction,
    ContextMenuContent,
    ContextMenuGroup,
    ContextMenuItem,
    ContextMenuLabel,
    ContextMenuSub,
    ContextMenuSubContent,
    ContextMenuSubTrigger,
    ContextMenuTrigger,
};
```

```rust
<ContextMenu>
    <ContextMenuTrigger>"Right click here"</ContextMenuTrigger>
    <ContextMenuContent>
        <ContextMenuLabel>"Actions"</ContextMenuLabel>
        <ContextMenuGroup>
            <ContextMenuItem>
                <ContextMenuAction>"Action 1"</ContextMenuAction>
            </ContextMenuItem>
            <ContextMenuItem>
                <ContextMenuAction>"Action 2"</ContextMenuAction>
            </ContextMenuItem>
        </ContextMenuGroup>
    </ContextMenuContent>
</ContextMenu>
```



## Examples

### With Press-and-Hold Action

A context menu with a press-and-hold confirmation action. The user must hold the button to confirm the action, preventing accidental clicks.

<StaticContextMenuAction />


### RTL

Context menu with Arabic items and nested submenus. Sub-menu indicators and flyout directions mirror for RTL.

<StaticContextMenuRtl />

## See Also

- [Dropdown Menu](/docs/components/dropdown-menu)
