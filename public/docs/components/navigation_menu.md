+++
title = "Navigation Menu"
description = "Rust/UI component for site-level horizontal navigation with animated dropdown panels and hover-activated content."
tags = []
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++



<StaticNavigationMenu />




## Installation

<StaticInstallNavigationMenu />




## Components

The NavigationMenu component is composed of several subcomponents:

- **NavigationMenu**: Root `<nav>` wrapper that provides context and scopes interactions
- **NavigationMenuList**: Horizontal `<ul>` container for menu items
- **NavigationMenuItem**: Individual `<li>` item — wraps a Trigger+Content pair or a Link
- **NavigationMenuTrigger**: Hover button with an auto-rotating ChevronDown icon
- **NavigationMenuContent**: Dropdown panel revealed on trigger hover
- **NavigationMenuLink**: Styled anchor for direct navigation links

### Helper

Use `navigation_menu_trigger_style()` to apply trigger-equivalent styles to a `NavigationMenuLink`:

```rust
<NavigationMenuLink class=navigation_menu_trigger_style() href="/docs">
    "Documentation"
</NavigationMenuLink>
```



## Usage

```rust
use crate::components::ui::navigation_menu::{
    NavigationMenu,
    NavigationMenuContent,
    NavigationMenuItem,
    NavigationMenuLink,
    NavigationMenuList,
    NavigationMenuTrigger,
    navigation_menu_trigger_style,
};
```

```rust
<NavigationMenu>
    <NavigationMenuList>
        <NavigationMenuItem>
            <NavigationMenuTrigger>"Getting Started"</NavigationMenuTrigger>
            <NavigationMenuContent>
                <ul class="grid w-[400px] gap-3 p-4 md:grid-cols-2">
                    // ListItem components here
                </ul>
            </NavigationMenuContent>
        </NavigationMenuItem>

        <NavigationMenuItem>
            <NavigationMenuLink class=navigation_menu_trigger_style() href="/docs">
                "Documentation"
            </NavigationMenuLink>
        </NavigationMenuItem>
    </NavigationMenuList>
</NavigationMenu>
```




## Examples


### Complex

Expanded navigation with multiple dropdown panels and direct links. Demonstrates how to build a full site header navigation with grouped content sections.

<StaticNavigationMenuComplex />



### RTL

Navigation menu with Arabic links and dropdown panels. Trigger chevrons and submenus open in the correct RTL direction.

<StaticNavigationMenuRtl />

## See Also

- [Menubar](/docs/components/menubar)
- [Dropdown Menu](/docs/components/dropdown-menu)
- [Breadcrumb](/docs/components/breadcrumb)
