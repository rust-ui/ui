+++
title = "Drag and Drop"
description = "Rust/UI component that allows users to drag and drop elements."
tags = []
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<StaticDragAndDrop />

## Installation

<StaticInstallDragAndDrop />

## Usage

```rust
use crate::ui::drag_and_drop::{Draggable, DraggableItem, DraggableZone};
```

```rust
rsx! {
    DemoDragAndDrop {}
}
```

## Examples

### Sortable Items

Simple drag-and-drop sorting between visual items.

<StaticDragAndDrop />

## See Also

- [Item](/components/item)
- [Card](/components/card)
