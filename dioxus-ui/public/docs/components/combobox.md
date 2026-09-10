+++
title = "Combobox"
description = "Autocomplete input and command palette with a list of suggestions."
tags = []
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<StaticCombobox />

## Installation

<StaticInstallCombobox />

## Usage

```rust
use crate::ui::command::{Command, CommandInput, CommandList, CommandItem};
use crate::ui::popover::{Popover, PopoverContent, PopoverTrigger};
```

```rust
rsx! {
    DemoCombobox {}
}
```

## Examples

### Searchable Select

Combines a popover with command-style filtering for fast keyboard-driven selection.

<StaticCombobox />

## See Also

- [Command](/components/command)
- [Select](/components/select)
