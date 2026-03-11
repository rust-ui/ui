+++
title = "Badge"
description = "Rust/UI component that displays a badge or a component that looks like a badge."
tags = []
is_new = false
image = "/images/thumbnails/badge.webp"
image_dark = "/images/thumbnails/badge-dark.webp"
+++


<StaticBadge />







## Installation 

<StaticInstallBadge />






## Usage

```rust
use crate::components::ui::badge::Badge;
```

```rust
<Badge>"Badge"</Badge>
```




## Examples

### Variants

Available Badge style variants include default, secondary, destructive, and outline options. Each variant provides different visual styling while maintaining consistent typography and accessibility standards across your Leptos application.

<StaticBadgeVariants />

### Colors

Semantic color variants for status indicators: success (green), warning (orange), and info (blue). Each adapts automatically to light and dark mode using the design system's color tokens.

<StaticBadgeColors />


### Custom

Customize Badge styles with Tailwind CSS classes to match your design system. This example demonstrates how to override default badge styling while preserving component functionality and type safety in Rust UI components.

<StaticBadgeCustom />


### RTL

Badge variants with Arabic labels. Icon slots (`inline-start` / `inline-end`) flip automatically so icons stay visually anchored to the correct side of the text.

<StaticBadgeRtl />

## See Also

- [Status](/docs/components/status)
- [Chips](/docs/components/chips)
- [Alert](/docs/components/alert)
