+++
title = "Dropzone"
description = "Rust/UI component that allows you to drop files onto it."
tags = ["utils"]
is_new = false
image = "/images/thumbnails/dropzone.webp"
image_dark = "/images/thumbnails/dropzone-dark.webp"
+++

<StaticDropzoneToggle />

## Installation

<StaticInstallDropzone />

## Usage

```rust
use crate::ui::dropzone::{Dropzone, DropzoneArea, DropzoneFileList, DropzoneHint, DropzoneIcon, DropzoneLabel};
```

```rust
rsx! {
    Dropzone {
        DropzoneArea {
            DropzoneIcon { Upload { class: "size-7" } }
            DropzoneLabel { "Drop files here" }
            DropzoneHint { "Up to 8 MB each" }
        }
        DropzoneFileList {}
    }
}
```

## Examples

### List view

Files appear as a row list with name, size, and remove button.

<StaticDropzone />

### Grid view

Files appear as a card grid with image and video previews on hover.

<StaticDropzoneGrid />

### List / grid toggle

Switch between list and grid view using the toggle buttons.

<StaticDropzoneToggle />

## See Also

- [Input](/components/input)
- [Drag and Drop](/components/drag-and-drop)
