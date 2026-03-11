+++
title = "Scroll Area"
description = "Rust/UI component that provides custom scrolling functionality with cross-browser styling."
tags = ["utils"]
is_new = true
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++




<StaticScrollArea />





## Installation

<StaticInstallScrollArea />



## Usage

You can use the `ScrollArea` component in combination with the [Separator](/docs/components/separator) component.

```rust
use crate::components::ui::scroll_area::ScrollArea;
```

```rust
<ScrollArea class="h-72 w-48 rounded-md border">
    <div class="p-4">
        // Your scrollable content here
    </div>
</ScrollArea>
```




## Examples

### Horizontal Scrolling

Horizontal scroll area for displaying wide content with custom scrollbar styling. This example demonstrates how to create horizontal scrolling containers in Leptos with cross-browser consistent scrollbars and smooth scroll behavior in Rust applications.

<StaticScrollAreaHorizontal />


### Scroll Snap

Scroll area with CSS scroll snap for precise item-to-item scrolling. This example shows how to implement snap scrolling in Leptos for creating [Card](/docs/components/card) carousel-like experiences and gallery views with smooth snap points in Rust applications.

<StaticScrollAreaSnap />



### RTL

Scrollable list with Arabic content. The scrollbar moves to the left side and text aligns to the right in RTL mode.

<StaticScrollAreaRtl />

## See Also

- [Data Table](/docs/components/data-table)
- [Table](/docs/components/table)
