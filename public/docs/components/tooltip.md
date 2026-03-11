+++
title = "Tooltip"
description = "A popup component that displays informative text when users hover over, focus on, or click an element."
tags = ["dialog"]
is_new = false
image = "/images/thumbnails/tooltip.webp"
image_dark = "/images/thumbnails/tooltip-dark.webp"
+++

<StaticTooltip />

## Installation

<StaticInstallTooltip />

## Components

The Tooltip component is composed of several subcomponents:

- **TooltipProvider**: Provides JavaScript functionality for tooltips
- **Tooltip**: Main wrapper component for trigger and content
- **TooltipContent**: Floating tooltip container with positioning
- **TooltipPosition**: Enum for position (Top/Bottom/Left/Right)



## Usage

```rust
use crate::registry::ui::tooltip::{Tooltip, TooltipContent, TooltipPosition, TooltipProvider};
```

```rust
<TooltipProvider />

<Tooltip>
    <Button>"Hover me"</Button>
    <TooltipContent>"This is a tooltip"</TooltipContent>
</Tooltip>
```

## Examples

### Positions

Tooltip component with multi-directional positioning including top, bottom, left, and right options. This example demonstrates the TooltipPosition enum in Leptos for creating flexible [Button](/docs/components/button) tooltips that adapt to viewport boundaries and improve accessibility in Rust applications.

<StaticTooltip />

### Components

Understanding the tooltip component architecture for building accessible interactive help text. The tooltip system consists of three main components that work together: TooltipProvider for JavaScript functionality, Tooltip wrapper for trigger and content, and TooltipContent for positioning and display in Leptos applications.

## API

### TooltipPosition

```rust
pub enum TooltipPosition {
    Top,
    Left,
    Right,
    Bottom,
}
```


### RTL

Tooltip with Arabic content in RTL. Tooltip text aligns right and the anchor arrow adjusts to the RTL reading direction.

<StaticTooltipRtl />

## See Also

- [Popover](/docs/components/popover)
- [Kbd](/docs/components/kbd)
