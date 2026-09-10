+++
title = "Slider"
description = "Rust/UI component that allows users to select a value from a range."
tags = []
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<StaticSlider />

## Installation

<StaticInstallSlider />

## Usage

```rust
use crate::ui::slider::Slider;
```

```rust
let mut value = use_signal(|| 50.0_f64);

rsx! {
    Slider {
        value: value(),
        oninput: move |e: FormEvent| {
            if let Ok(v) = e.value().parse::<f64>() {
                value.set(v);
            }
        },
    }
}
```

## States

## See Also

- [Progress](/components/progress)
- [Input](/components/input)
