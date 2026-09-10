+++
title = "Mask"
description = "Gradient fade masks for left, right, top, or bottom edges."
+++

<StaticMask />

## Installation

<StaticInstallMask />

## Usage

```rust
use crate::ui::mask::{Mask, MaskSide, MaskWrapper};
```

```rust
rsx! {
    MaskWrapper {
        div { "Scrollable content..." }
        Mask { side: MaskSide::Left }
        Mask { side: MaskSide::Right }
    }
}
```

## See Also

- [Marquee](/components/marquee)
- [Scroll Area](/components/scroll-area)
