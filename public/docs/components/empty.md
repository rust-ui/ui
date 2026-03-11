+++
title = "Empty"
description = "Use the Empty component to display a empty state."
tags = []
is_new = false
image = "/images/thumbnails/button.webp"
image_dark = "/images/thumbnails/button-dark.webp"
+++




<StaticEmpty />





## Installation 

<StaticInstallEmpty />



## Components

The Empty component is composed of several subcomponents:

- **Empty**: Main wrapper component for empty state display
- **EmptyHeader**: Header section for media and text
- **EmptyMedia**: Container for icon or illustration
- **EmptyTitle**: Primary heading text for the empty state
- **EmptyDescription**: Secondary descriptive text
- **EmptyContent**: Content area for action buttons



## Usage

You can use the `Empty` component in combination with the [Button](/docs/components/button) component.

```rust
use crate::components::ui::empty::{
    Empty,
    EmptyHeader,
    EmptyMedia,
    EmptyDescription,
    EmptyContent
};
```

```rust
<Empty>
  <EmptyHeader>
    <EmptyMedia variant=EmptyVariant::Icon>
      <Icon />
    </EmptyMedia>
    <EmptyTitle>"No data"</EmptyTitle>
    <EmptyDescription>"No data found"</EmptyDescription>
  </EmptyHeader>
  <EmptyContent>
    <Button>"Add data"</Button>
  </EmptyContent>
</Empty>
```




## Examples

### Background

Empty state component with muted background styling for subtle visual presentation. This example demonstrates how to create user-friendly empty states in Leptos with [Button](/docs/components/button) actions, background variants, and proper spacing for better visual hierarchy in Rust applications.

<StaticEmptyMuted />


### Background Fill

Empty state with a solid `bg-muted/30` fill, ideal as a section placeholder inside panels or dashboards where a subtle container background is needed.

<StaticEmptyBackground />


### Inside a Card

Empty state rendered inside a [Card](/docs/components/card) component. Useful when you need to display an empty state within an existing card layout while preserving the card's visual boundaries.

<StaticEmptyCard />


### Outline

Empty state with a dashed border outline. This variant draws attention to the empty area using the built-in border styling, well suited for dropzones or upload areas.

<StaticEmptyOutline />


### Avatar Group

Empty state with an [Avatar](/docs/components/avatar) group preview to hint at the expected content. Useful for team or member invitation screens where showing example avatars sets user expectations.

<StaticEmptyAvatarGroup />


### With Input Group

Empty state embedding an [InputGroup](/docs/components/input-group) with a [Kbd](/docs/components/kbd) shortcut hint. A practical pattern for 404 pages or search-driven empty states that guide users to find what they need.

<StaticEmptyInputGroup />


### RTL

Empty state with Arabic copy. Icon, heading, description, and action buttons all align to the right-to-left reading direction.

<StaticEmptyRtl />

## See Also

- [Data Table](/docs/components/data-table)
- [Table](/docs/components/table)
