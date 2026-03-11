+++
title = "Dropdown Menu"
description = "Rust/UI component that displays a dropdown menu."
tags = ["dropdown"]
is_new = false
image = "/images/thumbnails/dropdown.webp"
image_dark = "/images/thumbnails/dropdown-dark.webp"
+++


<StaticDropdownMenu />







## Installation

<StaticInstallDropdownMenu />



## Components

The DropdownMenu component is composed of several subcomponents:

- **DropdownMenu**: Main wrapper component managing open/close state
- **DropdownMenuTrigger**: Button or element that opens the menu
- **DropdownMenuContent**: Dropdown container for menu items
- **DropdownMenuLabel**: Label text for menu sections
- **DropdownMenuGroup**: Groups related menu items together
- **DropdownMenuItem**: Individual menu item container
- **DropdownMenuAction**: Clickable action within a menu item



## Usage

You can use the `DropdownMenu` component in combination with the [Separator](/docs/components/separator) component.

```rust
use crate::components::ui::dropdown_menu::{
    DropdownMenu,
    DropdownMenuAction,
    DropdownMenuContent,
    DropdownMenuGroup,
    DropdownMenuItem,
    DropdownMenuLabel,
    DropdownMenuTrigger,
};
```

```rust
<DropdownMenu>
    <DropdownMenuTrigger>"Open Menu"</DropdownMenuTrigger>
    <DropdownMenuContent>
        <DropdownMenuLabel>"Menu Label"</DropdownMenuLabel>
        <DropdownMenuGroup>
            <DropdownMenuItem>
                <DropdownMenuAction>"Action 1"</DropdownMenuAction>
            </DropdownMenuItem>
            <DropdownMenuItem>
                <DropdownMenuAction>"Action 2"</DropdownMenuAction>
            </DropdownMenuItem>
        </DropdownMenuGroup>
    </DropdownMenuContent>
</DropdownMenu>
```




## Examples


### DropdownMenu Start

Start-aligned dropdown menu with left edge alignment to the trigger button. This example demonstrates precise positioning in Leptos for creating predictable dropdown behavior that maintains visual consistency and improves navigation in Rust applications.

<StaticDropdownMenuStart />


### DropdownMenu End

End-aligned dropdown menu with right edge alignment to the trigger button. This example showcases flexible dropdown positioning in Leptos for right-aligned navigation menus and contextual actions in Rust applications.

<StaticDropdownMenuEnd />


### DropdownMenu StartOuter

StartOuter-aligned dropdown menu positioned completely outside the left trigger boundary. This example demonstrates advanced positioning patterns in Leptos where dropdowns extend beyond trigger elements for expanded menu layouts in Rust applications.

<StaticDropdownMenuStartOuter />


### DropdownMenu EndOuter

EndOuter-aligned dropdown menu positioned completely outside the right trigger boundary. This example shows versatile dropdown positioning in Leptos for creating mega menus and expanded navigation interfaces in Rust applications.

<StaticDropdownMenuEndOuter />


### DropdownMenu User

User profile dropdown menu with account information and action items. This example demonstrates how to build user menus in Leptos with [Avatar](/docs/components/avatar) display, profile links, and logout functionality for authentication workflows in Rust applications.

<StaticDropdownMenuUser />


### DropdownMenu User Icon

Compact user dropdown menu triggered by an icon button for space-efficient navigation. This example showcases icon-based user menus in Leptos with condensed layouts perfect for mobile-responsive designs in Rust applications.

<StaticDropdownMenuUserIcon />


### DropdownMenu Select

Dropdown menu functioning as a select component with selected value display on trigger. This example demonstrates custom select behavior in Leptos using dropdown menus for building stylized selection interfaces with full control in Rust applications.

<StaticDropdownMenuSelect />


### DropdownMenu Radio

Dropdown menu with radio group selection for mutually exclusive options. This example demonstrates using DropdownMenuRadioGroup in Leptos for building sort controls, filter selections, and other single-choice interfaces with visual feedback in Rust applications.

<StaticDropdownMenuRadio />


### DropdownMenu Destructive

Dropdown menu with a destructive action item styled in red to signal irreversible operations. Use `DropdownMenuActionVariant::Destructive` on `DropdownMenuAction` to mark delete or remove actions.

<StaticDropdownMenuDestructive />



### RTL

Dropdown menu with Arabic items and nested submenus. Sub-menu indicators and flyout positions mirror for right-to-left layouts.

<StaticDropdownMenuRtl />

## See Also

- [Context Menu](/docs/components/context-menu)
- [Popover](/docs/components/popover)
- [Select](/docs/components/select)
