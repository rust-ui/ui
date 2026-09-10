+++
title = "MultiSelect"
description = "Rust/UI component that displays a dropdown menu that allows the user to select several options."
tags = ["select"]
is_new = false
image = "/images/thumbnails/select.webp"
image_dark = "/images/thumbnails/select-dark.webp"
+++

<StaticMultiSelect />

## Installation

<StaticInstallMultiSelect />

## Usage

```rust
use crate::ui::multi_select::{
    MultiSelect,
    MultiSelectContent,
    MultiSelectGroup,
    MultiSelectItem,
    MultiSelectOption,
    MultiSelectTrigger,
    MultiSelectValue,
};
```

```rust
rsx! {
    DemoMultiSelect {}
}
```

## Examples

### Basic Multi Select

Select multiple values from a compact popover list.

<StaticMultiSelect />

### Alignment

Start and end aligned menu positioning relative to the trigger.

<StaticMultiSelectAlign />

### Scrollable

Scrollable option list for larger datasets like timezones.

<StaticMultiSelectScrollable />

## See Also

- [Select](/components/select)
- [Combobox](/components/combobox)
