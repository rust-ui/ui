+++
title = "Chips"
description = "Rust/UI component that displays a chip or a component that looks like a chip."
tags = []
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++


<StaticChips />







## Installation 

<StaticInstallChips />






## Components

The Chips component is composed of several subcomponents:

- **ChipsContainer**: Main wrapper component for chip items
- **ChipItem**: Individual chip element with label



## Usage

```rust
use crate::components::ui::chips::Chips;
```

```rust
<ChipsContainer>
    <ChipItem label="sunny" />
    <ChipItem label="cloudy" />
    <ChipItem label="hazy" />
</ChipsContainer>
```




## See Also

- [Badge](/docs/components/badge)
- [Multi Select](/docs/components/multi-select)
