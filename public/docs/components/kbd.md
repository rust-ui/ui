+++
title = "Kbd"
description = "Display keyboard shortcuts and key combinations with proper styling."
tags = ["utils"]
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<StaticKbd />

## Installation

<StaticInstallKbd />

## Usage

```rust
use crate::ui::kbd::Kbd;
```

```rust
rsx! {
    Kbd { "⌘K" }
}
```

## Examples

### Key Combination

Combine multiple `Kbd` elements with a separator to represent multi-key shortcuts.

## See Also

- [Badge](/components/badge)
- [Button](/components/button)
