+++
title = "Aspect Ratio"
description = "Displays content within a desired ratio."
tags = []
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++


<StaticAspectRatio />




## Installation

<StaticInstallAspectRatio />




## Usage

```rust
use crate::components::ui::aspect_ratio::AspectRatio;
```

```rust
<AspectRatio ratio=16.0 / 9.0 class="rounded-lg bg-muted">
    <img src="..." alt="..." class="size-full object-cover" />
</AspectRatio>
```




## Examples

### Square

<StaticAspectRatioSquare />


### Portrait

<StaticAspectRatioPortrait />


## See Also

- [Card](/docs/components/card)
- [Image](/docs/components/image)
