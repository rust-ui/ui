+++
title = "Use Press Hold"
description = "A Rust/UI hook for press-and-hold interactions with animated progress."
is_new = true
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++


<StaticUsePressHold />




## Installation

<StaticInstallUsePressHold />




## Usage

```rust
use crate::registry::hooks::use_press_hold::use_press_hold;
```

```rust
let on_complete = Callback::new(move |_| {
    // Action when hold completes
});

let disabled = Signal::derive(|| false);
let press_hold = use_press_hold(2000, on_complete, disabled);

// Attach to element events
view! {
    <button
        on:pointerdown=move |_| press_hold.on_pointer_down()
        on:pointerup=move |_| press_hold.on_pointer_up()
        on:pointerleave=move |_| press_hold.on_pointer_up()
        on:pointercancel=move |_| press_hold.on_pointer_up()
    >
        // Use progress for visual feedback
        {move || format!("{:.0}%", press_hold.progress.get() * 100.0)}
    </button>
}
```
