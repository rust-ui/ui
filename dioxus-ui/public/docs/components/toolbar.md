+++
title = "Toolbar"
description = "A composable toolbar with toggle groups, buttons, separators, and links."
+++

<StaticToolbar />

## Installation

<StaticInstallToolbar />

## Usage

```rust
use crate::ui::toolbar::{
    Toolbar, ToolbarButton, ToolbarItem, ToolbarLink,
    ToolbarList, ToolbarSeparator, ToolbarToggleGroup, ToolbarToggleItem,
};
```

```rust
let mut bold = use_signal(|| false);

rsx! {
    Toolbar {
        ToolbarToggleGroup {
            ToolbarToggleItem {
                title: "Bold",
                pressed: bold(),
                onclick: move |_| bold.set(!bold()),
                Bold {}
            }
        }
        ToolbarSeparator {}
        ToolbarButton { "Edit" }
    }
}
```

## Examples

### Formatting

<StaticToolbar />

## See Also

- [Toggle Group](/components/toggle-group)
- [Button Group](/components/button-group)
- [Separator](/components/separator)
