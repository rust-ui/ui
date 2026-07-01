+++
title = "Bubble"
description = "Displays a chat message bubble with multiple style variants and an optional reactions overlay."
tags = []
is_new = true
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++


<StaticBubble />




## Installation

<StaticInstallBubble />




## Usage

```rust
use crate::components::ui::bubble::{Bubble, BubbleContent, BubbleGroup, BubbleReactions, BubbleVariant};
```

```rust
<Bubble>
    <BubbleContent>"I checked the registry output and removed the stale route."</BubbleContent>
    <BubbleReactions>
        <span>"👍"</span>
    </BubbleReactions>
</Bubble>
```

## Composition

```
Bubble
├── BubbleContent          (renders as <div>, <a>, or <button>)
└── BubbleReactions        (absolute overlay — emoji / reaction buttons)

BubbleGroup                (stacks multiple bubbles with gap)
```

## Examples

### Variants

Use `variant` to change the visual treatment of the bubble.

<StaticBubbleVariants />

### Alignment

Use `align` on `Bubble` to align the bubble to the start or end of the conversation.

<StaticBubbleAlignment />

### Bubble Group

Use `BubbleGroup` to group consecutive bubbles from the same sender.

<StaticBubbleGroup />

### Links and Buttons

Pass `on_click` to `BubbleContent` to render it as a `<button>`, or `href` to render it as an `<a>`.

<StaticBubbleLinkButton />

### Reactions

Use `BubbleReactions` for bubble reactions. Use `side` (`Top` / `Bottom`) and `align` (`Start` / `End`) to position the overlay.

<StaticBubbleReactions />

### Show More / Collapsible

Compose `BubbleContent` with `Collapsible` for show-more / show-less long content.

<StaticBubbleCollapsible />

### Tooltip

Wrap a bubble in a `Tooltip` to reveal metadata on hover.

<StaticBubbleTooltip />

### Popover

Pair a bubble with a `Popover` to surface more information on demand.

<StaticBubblePopover />

## See Also

- [Message](/docs/components/message)
- [Marker](/docs/components/marker)
