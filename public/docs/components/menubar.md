+++
title = "Menubar"
description = "Rust/UI component that displays a horizontal menu bar with dropdown menus, submenus, checkboxes, and radio items."
tags = []
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++



<StaticMenubar />




## Installation

<StaticInstallMenubar />



## Components

The Menubar component is composed of several subcomponents:

- **Menubar**: Main horizontal menu bar wrapper
- **MenubarMenu**: Individual menu entry within the bar
- **MenubarTrigger**: Clickable label that opens a menu's dropdown
- **MenubarContent**: Dropdown container for menu items
- **MenubarGroup**: Groups related menu items together
- **MenubarLabel**: Section label within a dropdown
- **MenubarItem**: Individual clickable menu item
- **MenubarShortcut**: Keyboard shortcut hint displayed on the right
- **MenubarSeparator**: Horizontal divider between groups
- **MenubarCheckboxItem**: Menu item with a toggle checkbox
- **MenubarRadioGroup**: Group of mutually exclusive radio items
- **MenubarRadioItem**: Single radio option within a `MenubarRadioGroup`
- **MenubarSub**: Submenu wrapper
- **MenubarSubTrigger**: Item that opens a submenu on hover
- **MenubarSubContent**: Submenu dropdown container
- **MenubarSubItem**: Individual item inside a submenu



## Usage

```rust
use crate::components::ui::menubar::{
    Menubar,
    MenubarContent,
    MenubarGroup,
    MenubarItem,
    MenubarMenu,
    MenubarSeparator,
    MenubarShortcut,
    MenubarTrigger,
};
```

```rust
<Menubar>
    <MenubarMenu>
        <MenubarTrigger>"File"</MenubarTrigger>
        <MenubarContent>
            <MenubarGroup>
                <MenubarItem>
                    "New Tab"
                    <MenubarShortcut>"⌘T"</MenubarShortcut>
                </MenubarItem>
                <MenubarItem>
                    "New Window"
                    <MenubarShortcut>"⌘N"</MenubarShortcut>
                </MenubarItem>
            </MenubarGroup>
            <MenubarSeparator />
            <MenubarGroup>
                <MenubarItem>
                    "Print..."
                    <MenubarShortcut>"⌘P"</MenubarShortcut>
                </MenubarItem>
            </MenubarGroup>
        </MenubarContent>
    </MenubarMenu>
</Menubar>
```



## Examples

### RTL

Menubar with Arabic labels and nested submenus. Sub-menu indicators and flyout directions mirror for RTL.

<StaticMenubarRtl />



## See Also

- [Dropdown Menu](/docs/components/dropdown-menu)
- [Context Menu](/docs/components/context-menu)
