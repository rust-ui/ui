+++
title = "Hover Card"
description = "Rust/UI component that displays rich content in a floating card when hovering over a trigger element."
tags = []
is_new = true
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++



<StaticHoverCard />



## Installation

<StaticInstallHoverCard />



## Components

The HoverCard component is composed of:

- **HoverCard**: Root wrapper — manages anchor positioning and entrance animation
- **HoverCardTrigger**: Any element that triggers the card on hover (wraps children in a `<span>`)
- **HoverCardContent**: The floating card panel shown on hover
- **HoverCardSide**: Enum controlling which side the card appears (`Top` / `Bottom` / `Left` / `Right`)



## Usage

```rust
use crate::components::ui::hover_card::{
    HoverCard,
    HoverCardContent,
    HoverCardSide,
    HoverCardTrigger,
};
```

```rust
<HoverCard>
    <HoverCardTrigger>
        <Button variant=ButtonVariant::Link>"@rust-lang"</Button>
    </HoverCardTrigger>
    <HoverCardContent class="w-80">
        <p>"Card content goes here."</p>
    </HoverCardContent>
</HoverCard>
```



### RTL

Hover card with Arabic bio text. Avatar, username, and metadata layout reverse to follow the right-to-left reading direction.

<StaticHoverCardRtl />

## See Also

- [Popover](/docs/components/popover)
- [Tooltip](/docs/components/tooltip)
- [Dialog](/docs/components/dialog)
