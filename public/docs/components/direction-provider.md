+++
title = "Direction Provider"
description = "Rust/UI wrapper component that sets text direction (LTR or RTL) for all children, enabling right-to-left layout support."
tags = []
is_new = true
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++


<StaticDirectionProvider />




## Installation

<StaticInstallDirectionProvider />




## Usage

```rust
use crate::components::ui::direction_provider::{Direction, DirectionProvider};
```

```rust
<DirectionProvider dir=Direction::Rtl>
    // All children respect RTL layout automatically
</DirectionProvider>
```




## Examples

### LTR (Default)

The default direction is left-to-right. Wrapping content in a `DirectionProvider` with `dir=Direction::Ltr` is the same as not wrapping at all.

<StaticDirectionProviderDefault />

### RTL (Arabic)

Set `dir=Direction::Rtl` to flip layout direction for all children. Text aligns to the right, flex rows reverse, and Tailwind's `rtl:` variants activate automatically.

<StaticDirectionProviderRtl />


## How it works

`DirectionProvider` renders a `<div>` with the HTML `dir` attribute. Browsers use this attribute to:

- Right-align text in RTL mode
- Reverse inline flow (flex rows, margins, paddings)
- Flip form elements and controls

Tailwind's built-in `rtl:` variants are also activated, so any class like `rtl:flex-row-reverse` or `rtl:text-right` will apply automatically inside the provider.

For logical spacing, use `ms-*`/`me-*` (margin-start/end) and `ps-*`/`pe-*` (padding-start/end) instead of `ml-*`/`mr-*` to ensure consistent behavior in both directions.


## See Also

- [Field](/docs/components/field)
- [Input](/docs/components/input)
- [Button](/docs/components/button)
