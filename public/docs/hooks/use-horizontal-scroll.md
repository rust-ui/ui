+++
title = "Use Horizontal Scroll"
description = "A Rust/UI hook that manages horizontal scrolling with state tracking and programmatic scroll controls."
tags = ["utils"]
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++



<StaticUseHorizontalScroll />







## Installation

<StaticInstallUseHorizontalScroll />





## Usage

```rust
use leptos::html::Div;
use leptos::prelude::*;
use crate::registry::hooks::use_horizontal_scroll::{use_horizontal_scroll, HorizontalScrollState};
```

```rust
let scroll_container_ref = NodeRef::<Div>::new();

// Use the hook with default options (50% scroll, 300ms delay)
let scroll_ctx = use_horizontal_scroll(scroll_container_ref, None, None);

// Or customize scroll percentage and update delay
let scroll_ctx = use_horizontal_scroll(
    scroll_container_ref,
    Some(0.75),  // Scroll 75% of container width
    Some(500)    // 500ms delay before state update
);

// Attach to your scrollable container
view! {
    <div
        node_ref=scroll_container_ref
        on:scroll=move |e| scroll_ctx.on_scroll.run(e)
        class="overflow-x-scroll"
    >
        // Your scrollable content
    </div>
}

// Scroll programmatically
scroll_ctx.scroll_by.run(-1); // Scroll left
scroll_ctx.scroll_by.run(1);  // Scroll right

// Check scroll state
match scroll_ctx.scroll_state.get() {
    HorizontalScrollState::Start => {},   // At the beginning
    HorizontalScrollState::Middle => {},  // In the middle
    HorizontalScrollState::End => {},     // At the end
}
```
