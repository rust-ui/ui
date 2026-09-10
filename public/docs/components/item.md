+++
title = "Item"
description = "A flexible container component for displaying list items with media, content, and actions."
tags = []
is_new = false
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
- **ItemMedia**: Container for avatar, icon, or image
- **ItemContent**: Content wrapper for title and description
- **ItemTitle**: Primary heading text for the item
- **ItemDescription**: Secondary descriptive text
- **ItemActions**: Container for action buttons or menus
- **ItemHeader**: Full-width header row for metadata
- **ItemFooter**: Full-width footer row for tags, dates, or secondary actions
- **ItemSeparator**: Divider line between items in a group

## Usage

```rust
use registry::ui::item::{
    Item, ItemActions, ItemContent, ItemDescription, ItemMedia, ItemTitle, ItemVariant,
};
```

```rust
rsx! {
    Item { variant: ItemVariant::Outline,
        ItemContent {
            ItemTitle { "Item Title" }
            ItemDescription { "Item description text." }
        }
        ItemActions {
            Button { "Action" }
        }
    }
}
```

## Examples

### Variants

<StaticItemVariants />

### Dropdown Menu

<StaticItemDropdownMenu />

### Item Group

<StaticItemGroup />

### Image Media

<StaticItemMediaImage />

### File Upload List

<StaticItemFileUpload />

### RTL

<StaticItemRtl />

## See Also

- [Card](/components/card)
- [Avatar](/components/avatar)
- [Badge](/components/badge)
