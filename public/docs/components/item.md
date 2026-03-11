+++
title = "Item"
description = "A flexible container component for displaying list items with media, content, and actions."
tags = []
is_new = true
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++




<StaticItem />





## Installation

<StaticInstallItem />



## Components

The Item component is composed of several subcomponents:

- **Item**: Main wrapper component with variant styling
- **ItemGroup**: List container for grouping multiple items
- **ItemMedia**: Container for avatar, icon, or image — supports `Default`, `Icon`, and `Image` variants
- **ItemContent**: Content wrapper for title and description
- **ItemTitle**: Primary heading text for the item
- **ItemDescription**: Secondary descriptive text
- **ItemActions**: Container for action buttons or menus
- **ItemHeader**: Full-width header row (badge, timestamp, metadata)
- **ItemFooter**: Full-width footer row (tags, date, secondary actions)
- **ItemSeparator**: Divider line between items in a group



## Usage

You can use the `Item` component in combination with the [Button](/docs/components/button), [Avatar](/docs/components/avatar) and [DropdownMenu](/docs/components/dropdown-menu) components.

```rust
use crate::components::ui::item::{Item, ItemContent, ItemTitle, ItemDescription, ItemActions, ItemMedia};
```

```rust
<Item variant=ItemVariant::Outline>
    <ItemContent>
        <ItemTitle>"Item Title"</ItemTitle>
        <ItemDescription>"Item description text."</ItemDescription>
    </ItemContent>
</Item>
```




## Examples

### Variants

Item component with multiple visual style variants including default, outline, and muted options. This example demonstrates how to create consistent list item styling in Leptos with different visual emphasis levels for building flexible UI layouts in Rust applications.

<StaticItemVariants />


### Dropdown Menu

Item component integration with dropdown menus for rich content selection interfaces. This example shows how to combine Item and [DropdownMenu](/docs/components/dropdown-menu) components in Leptos to build sophisticated selection menus with [Avatar](/docs/components/avatar) media, descriptions, and actions in Rust applications.

<StaticItemDropdownMenu />


### Item Group

Group multiple items in a list container with separators and full-width header rows for metadata like category badges and timestamps.

<StaticItemGroup />


### Image Media

Use `ItemMedia` with `variant=ItemMediaVariant::Image` for object-cover image thumbnails, and `ItemFooter` for a full-width row of tags or dates beneath the content.

<StaticItemMediaImage />


### File Upload List

`ItemSize::Xs` items with `ItemMedia variant=Icon`, a `Progress` bar, and a time-remaining label — demonstrates the compact upload queue pattern.

<StaticItemFileUpload />


### RTL

Item components with Arabic titles and descriptions. Icon slots, content, and action areas all align to the right-to-left reading direction.

<StaticItemRtl />

## See Also

- [Card](/docs/components/card)
- [Avatar](/docs/components/avatar)
- [Badge](/docs/components/badge)
- [Checkbox](/docs/components/checkbox)
