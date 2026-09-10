+++
title = "Marker"
description = "Displays an inline status, system note, bordered row, or labeled separator in a conversation."
tags = []
is_new = true
image = "/images/thumbnails/marker.webp"
image_dark = "/images/thumbnails/marker-dark.webp"
+++

<StaticMarker />

## Installation

<StaticInstallMarker />

## Usage

```rust
use crate::ui::marker::{Marker, MarkerContent, MarkerIcon};
```

```rust
rsx! {
    Marker {
        MarkerIcon { GitBranch {} }
        MarkerContent { "Switched to a new branch" }
    }
}
```

## Examples

### Variants

Use the `variant` prop to switch between `Default`, `Separator`, and `Border` layouts.

<StaticMarkerVariants />

### Border

<StaticMarkerBorder />

### Separator

Renders inline bars on either side of the content, useful for timeline dividers.

<StaticMarkerSeparator />

### Icon

<StaticMarkerIcon />

### Status

Pass `role="status"` for markers announcing in-progress work.

<StaticMarkerStatus />

### Shimmer

Combine `role="status"` with a `shimmer` class on `MarkerContent` for an animated loading state.

<StaticMarkerShimmer />

### Link / Button

Pass `href` to render an `<a>`, or `onclick` to render a `<button>`.

<StaticMarkerLinkButton />

## See Also

- [Badge](/components/badge)
- [Status](/components/status)
