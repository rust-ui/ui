+++
title = "Marker"
description = "A contextual event row for chat transcripts — separators, system events, and branch/file activity markers."
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
    <MarkerIcon><UserRound /></MarkerIcon>
    <MarkerContent>"Alice joined the conversation"</MarkerContent>
</Marker>
```




## Examples

### Default

System events such as users joining or leaving, message delivery confirmations.

<StaticMarker />

### Border

Branch switches, file reviews, and activity separators with a bottom border rule.

<StaticMarkerBorder />

### Separator

Full-width separator with centered content — ideal for timestamps, thinking indicators, or navigation links.

<StaticMarkerSeparator />

### With Accordion

Separator containing a collapsible tool-call detail list (e.g. "Exploring codebase").

<StaticMarkerAccordion />

### With Drawer

Separator containing a Drawer trigger for activity logs or side panels.

<StaticMarkerDrawer />

## See Also

- [Message](/docs/components/message)
- [Bubble](/docs/components/bubble)
