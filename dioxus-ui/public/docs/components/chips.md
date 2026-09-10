+++
title = "Chips"
description = "Rust/UI component that displays a chip or a component that looks like a chip."
tags = []
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<StaticChips />

## Installation

<StaticInstallChips />

## Usage

```rust
use crate::ui::chips::{ChipsContainer, ChipItem};
```

```rust
rsx! {
    ChipsContainer {
        ChipItem { label: "sunny" }
        ChipItem { label: "cloudy" }
        ChipItem { label: "hazy" }
    }
}
```

## See Also

- [Badge](/components/badge)
- [Toggle Group](/components/toggle-group)
