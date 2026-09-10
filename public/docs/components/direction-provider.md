+++
title = "Direction Provider"
description = "Rust/UI wrapper component that sets text direction (LTR or RTL) for all children, enabling right-to-left layout support."
tags = []
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<StaticDirectionProviderDefault />

## Installation

<StaticInstallDirectionProvider />

## Usage

```rust
use crate::ui::direction_provider::DirectionProvider;
```

```rust
rsx! {
    DirectionProvider { dir: "rtl", "Your content" }
}
```

## Examples

### Default Direction

Default layout direction without overrides.

<StaticDirectionProviderDefault />

### Custom Direction

Explicit direction provider wrapping a subtree.

<StaticDirectionProvider />

### RTL

Right-to-left rendering for Arabic or Hebrew oriented interfaces.

<StaticDirectionProviderRtl />

## See Also

- [Button Group](/components/button_group)
- [Sheet](/components/sheet)
