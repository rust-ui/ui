+++
title = "Use Lock Body Scroll"
description = "A Rust/UI hook that locks and unlocks body scrolling, useful for modal dialogs, sheets, and overlays."
tags = ["utils", "dialog"]
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++



<StaticUseLockBodyScroll />







## Installation

<StaticInstallUseLockBodyScroll />





## Usage

```rust
use crate::components::hooks::use_lock_body_scroll::use_lock_body_scroll;
```

```rust
let scroll_locked = use_lock_body_scroll(false);

// Lock body scrolling
scroll_locked.set(true);

// Unlock body scrolling  
scroll_locked.set(false);
```
