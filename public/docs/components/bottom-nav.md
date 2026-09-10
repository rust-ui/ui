+++
title = "Bottom Nav"
description = "Rust/UI component that displays a mobile-friendly bottom navigation bar."
tags = ["navigation"]
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<StaticBottomNav />

## Installation

<StaticInstallBottomNav />

## Usage

```rust
use crate::ui::bottom_nav::{BottomNav, BottomNavGrid, BottomNavButton, BottomNavLabel};
```

```rust
rsx! {
    BottomNav {
        BottomNavGrid {
            BottomNavButton { aria-current: "page",
                House { class: "size-5" }
                BottomNavLabel { "Home" }
            }
        }
    }
}
```

## See Also

- [Tabs](/components/tabs)
- [Button](/components/button)
