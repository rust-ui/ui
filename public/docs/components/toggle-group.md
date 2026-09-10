+++
title = "Toggle Group"
description = "A set of toggle buttons that can be used to group related options."
tags = []
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<StaticToggleGroup />

## Installation

<StaticInstallToggleGroup />

## Usage

```rust
use crate::ui::toggle_group::{ToggleGroup, ToggleGroupItem};
```

```rust
let mut selected = use_signal(|| "center".to_string());

rsx! {
    ToggleGroup {
        ToggleGroupItem {
            value: "left",
            pressed: selected() == "left",
            onclick: move |_| selected.set("left".to_string()),
            "Left"
        }
        ToggleGroupItem {
            value: "center",
            pressed: selected() == "center",
            onclick: move |_| selected.set("center".to_string()),
            "Center"
        }
        ToggleGroupItem {
            value: "right",
            pressed: selected() == "right",
            onclick: move |_| selected.set("right".to_string()),
            "Right"
        }
    }
}
```

## See Also

- [Tabs](/components/tabs)
