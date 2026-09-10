+++
title = "Select"
description = "Rust/UI component that displays a dropdown menu that allows the user to select an option."
tags = ["select"]
is_new = false
image = "/images/thumbnails/select.webp"
image_dark = "/images/thumbnails/select-dark.webp"
+++

<StaticSelect />

## Installation

<StaticInstallSelect />

## Usage

```rust
use crate::ui::select::{Select, SelectContent, SelectGroup, SelectLabel, SelectOption, SelectTrigger, SelectValue};
```

```rust
rsx! {
    DemoSelect {}
}
```

## Examples

### Basic Select

Single-value select menu.

<StaticSelect />

### Scrollable

Scrollable select content for longer option lists.

<StaticSelectScrollable />

### RTL

Select menu in a right-to-left layout.

<StaticSelectRtl />

## See Also

- [Multi Select](/components/multi_select)
