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
use crate::components::ui::marker::{Marker, MarkerContent, MarkerIcon};
```

```rust
<Marker>
    <MarkerIcon><Check /></MarkerIcon>
    <MarkerContent>"Explored 4 files"</MarkerContent>
</Marker>
```

## Composition

```
Marker
├── MarkerIcon
└── MarkerContent
```

## Examples

### Variants

Use `variant` to switch between an inline marker, bordered row, and labeled separator.

<StaticMarkerVariants />

| Variant     | Description                                          |
| ----------- | ---------------------------------------------------- |
| `Default`   | An inline marker for status, notes, and actions.     |
| `Border`    | A default marker with a bottom border under the row. |
| `Separator` | A centered label with divider lines on each side.    |

### Status

Set `role="status"` and include a `Spinner` for streaming or in-progress markers.

<StaticMarkerStatus />

### Shimmer

Add the `shimmer` utility class to `MarkerContent` for an animated streaming-text effect.

<StaticMarkerShimmer />

### Separator

Use the `Separator` variant for labeled dividers such as dates or section breaks.

<StaticMarkerSeparator />

### Border

Use the `Border` variant for status rows that separate the next row with a bottom border.

<StaticMarkerBorder />

### With Icon

Use `MarkerIcon` to render an icon alongside the content. Use `class="flex-col"` to stack the icon above the content.

<StaticMarkerIcon />

### Links and Buttons

Pass `href` to render the marker as an `<a>`, or `on_click` to render it as a `<button>`.

<StaticMarkerLinkButton />

## See Also

- [Message](/docs/components/message)
- [Bubble](/docs/components/bubble)
- [Spinner](/docs/components/spinner)
