+++
title = "Collapsible"
description = "An interactive component which expands/collapses a panel with smooth animation."
tags = []
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++



<StaticCollapsible />




## Installation

<StaticInstallCollapsible />




## Components

The Collapsible component is composed of three subcomponents:

- **Collapsible**: Root wrapper that holds state and provides context
- **CollapsibleTrigger**: Button that toggles open/closed on click
- **CollapsibleContent**: Animated panel — expands/collapses with a CSS grid height transition



## Usage

```rust
use crate::components::ui::collapsible::{
    Collapsible,
    CollapsibleContent,
    CollapsibleTrigger,
};
```

```rust
<Collapsible>
    <CollapsibleTrigger>"Toggle"</CollapsibleTrigger>
    <CollapsibleContent>
        "Content revealed on expand."
    </CollapsibleContent>
</Collapsible>
```

### Controlled State

Pass an external `RwSignal<bool>` to drive the open state from outside:

```rust
let open = RwSignal::new(false);

<Collapsible open=open>
    <CollapsibleTrigger>"Toggle"</CollapsibleTrigger>
    <CollapsibleContent>"Content"</CollapsibleContent>
</Collapsible>
```




## Examples

### Basic

Product details panel in a Card — trigger button with a rotating `ChevronDown` icon using `group-data-[state=open]:rotate-180`.

<StaticCollapsibleBasic />




### File Tree

Recursive nested collapsibles — folders expand/collapse with a rotating `ChevronRight` icon using `group-data-[state=open]:rotate-90`.

<StaticCollapsibleFileTree />




### Settings

Corner radius inputs — top corners always visible, bottom corners revealed on expand. Uses `outer_class="col-span-full"` on `CollapsibleContent` to participate in the parent `grid-cols-2` layout via `grid-cols-subgrid`.

<StaticCollapsibleSettings />




### RTL

Collapsible panel with Arabic content. The toggle button and label swap sides, and content aligns to the right.

<StaticCollapsibleRtl />

## See Also

- [Accordion](/docs/components/accordion)
- [Card](/docs/components/card)
