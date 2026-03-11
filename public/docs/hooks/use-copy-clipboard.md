+++
title = "Use Copy Clipboard"
description = "A Rust/UI hook that copies text to clipboard with optional timeout to show copied state."
tags = ["utils"]
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++


<StaticUseCopyToClipboard />




## Installation

<StaticInstallUseCopyClipboard />




## Usage

```rust
use crate::components::hooks::use_copy_clipboard::use_copy_clipboard;
```

```rust
let (copy_to_clipboard, copied) = use_copy_clipboard(Some(2000));

// Copy text to clipboard
copy_to_clipboard("text to copy".to_string());

// Check if recently copied
if copied.get() {
    // Show copied state
}
```