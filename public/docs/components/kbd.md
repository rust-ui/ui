+++
title = "Kbd"
description = "Display keyboard shortcuts and key combinations with proper styling."
tags = ["utils"]
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++




<StaticKbd />





## Installation

<StaticInstallKbd />



## Components

- **Kbd**: A single keyboard key with accessible styling and tooltip context override
- **KbdGroup**: Wraps multiple `Kbd` elements in an inline row with consistent spacing



## Usage

```rust
use crate::components::ui::kbd::{Kbd, KbdGroup};
```

```rust
<KbdGroup>
    <Kbd>"⌘"</Kbd>
    <Kbd>"K"</Kbd>
</KbdGroup>
```



## Examples

### Default

Modifier keys and multi-key combinations using `Kbd` and `KbdGroup`.

<StaticKbd />


### In Button

Pair a [Button](/docs/components/button) with a `Kbd` to surface keyboard shortcuts inline.

<StaticKbdButton />


### With Input Group

Add `Kbd` keys inside an [InputGroup](/docs/components/input-group) to hint at search shortcuts.

<StaticKbdInputGroup />


### RTL

Keyboard shortcut hints in an RTL layout. The key sequence reads right-to-left while individual key caps remain legible.

<StaticKbdRtl />

## See Also

- [Tooltip](/docs/components/tooltip)
- [Command](/docs/components/command)
