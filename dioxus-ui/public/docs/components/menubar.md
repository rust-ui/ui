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

## Usage

```rust
rsx! {
    DemoMenubar {}
}
```

## Examples

### Basic Menubar

Application-style top menubar with nested actions.

<StaticMenubar />

### RTL

Menubar layout in a right-to-left interface.

<StaticMenubarRtl />

## See Also

- [Dropdown Menu](/components/dropdown_menu)
- [Navigation Menu](/components/navigation_menu)
