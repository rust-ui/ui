+++
title = "Use Is Mobile"
description = "A reactive hook that returns true when the viewport is below the mobile breakpoint (768px)."
tags = ["utils"]
is_new = true
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++


<StaticUseIsMobile />



## Installation

<StaticInstallUseIsMobile />



## Usage

```rust
use crate::components::hooks::use_is_mobile::use_is_mobile;
```

```rust
let is_mobile = use_is_mobile();

view! {
    {move || if is_mobile.get() {
        view! { <Drawer>...</Drawer> }.into_any()
    } else {
        view! { <Dialog>...</Dialog> }.into_any()
    }}
}
```



## Examples

### Default

Shows a `Mobile` or `Desktop` indicator that updates as you resize the window below/above 768px.

<StaticUseIsMobile />



## See Also

- [Use Media Query](/docs/hooks/use-media-query)
- [Drawer](/docs/components/drawer)
- [Dialog](/docs/components/dialog)
