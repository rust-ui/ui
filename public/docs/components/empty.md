+++
title = "Empty"
description = "Use the Empty component to display a empty state."
tags = []
is_new = false
image = "/images/thumbnails/button.webp"
image_dark = "/images/thumbnails/button-dark.webp"
+++

<StaticEmpty />

## Installation

<StaticInstallEmpty />

## Usage

```rust
use crate::ui::empty::{Empty, EmptyHeader, EmptyTitle, EmptyDescription, EmptyContent, EmptyMedia, EmptyMediaVariant};
```

```rust
rsx! {
    Empty {
        EmptyHeader {
            EmptyMedia { variant: EmptyMediaVariant::Icon,
                // your icon here
            }
            EmptyTitle { "No Projects Yet" }
            EmptyDescription { "Get started by creating your first project." }
        }
        EmptyContent {
            Button { "Create Project" }
        }
    }
}
```

## Muted

<StaticEmptyMuted />

## See Also

- [Card](/components/card)
- [Skeleton](/components/skeleton)
