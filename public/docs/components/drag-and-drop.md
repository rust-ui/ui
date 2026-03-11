+++
title = "Drag and Drop"
description = "Rust/UI component that allows users to drag and drop elements."
tags = []
is_new = true
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++


<StaticDragAndDrop />











## Installation

<StaticInstallDragAndDrop />



## Components

The Drag and Drop component is composed of several subcomponents:

- **Draggable**: Main wrapper component managing drag state
- **DraggableZone**: Drop zone container for draggable items
- **DraggableItem**: Individual draggable item element



## Usage

```rust
use crate::components::ui::drag_and_drop::{
    Draggable,
    DraggableItem,
    DraggableZone,
};
```

```rust
<Draggable>
    <DraggableZone>
        <DraggableItem text="Item 1" />
        <DraggableItem text="Item 2" />
        <DraggableItem text="Item 3" />
    </DraggableZone>
</Draggable>
```


## See Also

- [Dropzone](/docs/components/dropzone)
