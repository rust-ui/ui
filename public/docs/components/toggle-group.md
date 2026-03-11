+++
title = "Toggle Group"
description = "A set of toggle buttons that can be used to group related options."
tags = []
is_new = true
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<StaticToggleGroup />




## Installation

<StaticInstallToggleGroup />




## Usage

```rust
use crate::components::ui::toggle_group::{
    ToggleGroup,
    ToggleGroupItem,
};
```

```rust
<ToggleGroup>
    <ToggleGroupItem title="Bold" pressed=bold_signal>
        <Bold />
    </ToggleGroupItem>
    <ToggleGroupItem title="Italic" pressed=italic_signal>
        <Italic />
    </ToggleGroupItem>
</ToggleGroup>
```


## Examples

### Outline

<StaticToggleGroupOutline />

### Vertical

<StaticToggleGroupVertical />

### Spacing

Items with `spacing=0` render joined together like a button group.

<StaticToggleGroupSpacing />

### Font Weight Selector

<StaticToggleGroupFontWeight />


### RTL

Toggle group in an RTL layout. Icon buttons maintain their visual grouping while the overall row order reverses.

<StaticToggleGroupRtl />

## See Also

- [Button](/docs/components/button)
- [Button Group](/docs/components/button-group)
- [Tabs](/docs/components/tabs)
