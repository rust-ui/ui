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



## Components

The BottomNav component is composed of several subcomponents:

- **BottomNav**: Main wrapper component fixed to bottom
- **BottomNavGrid**: Grid container for navigation buttons
- **BottomNavButton**: Individual navigation button with icon
- **BottomNavLabel**: Text label below the icon



## Usage

```rust
use crate::components::ui::bottom_nav::{
    BottomNav,
    BottomNavButton,
    BottomNavGrid,
    BottomNavLabel,
};
```

```rust
<BottomNav>
    <BottomNavGrid>
        <BottomNavButton>
            <House class="size-5" />
            <BottomNavLabel>"Home"</BottomNavLabel>
        </BottomNavButton>
        <BottomNavButton>
            <Wallet class="size-5" />
            <BottomNavLabel>"Wallet"</BottomNavLabel>
        </BottomNavButton>
        <BottomNavButton>
            <Settings class="size-5" />
            <BottomNavLabel>"Settings"</BottomNavLabel>
        </BottomNavButton>
    </BottomNavGrid>
</BottomNav>
```


## See Also

- [Breadcrumb](/docs/components/breadcrumb)
- [Tabs](/docs/components/tabs)
- [Pagination](/docs/components/pagination)
