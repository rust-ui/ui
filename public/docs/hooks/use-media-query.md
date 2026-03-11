+++
title = "Use Media Query"
description = "A reactive hook that tracks whether a CSS media query matches, updating automatically when the viewport changes."
tags = ["utils"]
is_new = true
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++


<StaticUseMediaQuery />



## Installation

<StaticInstallUseMediaQuery />



## Usage

```rust
use crate::components::hooks::use_media_query::use_media_query;
```

```rust
let is_wide = use_media_query("(min-width: 1024px)");

view! {
    {move || if is_wide.get() { "Wide layout" } else { "Narrow layout" }}
}
```



## Examples

### Default

Track multiple breakpoints simultaneously — signals update reactively as the viewport resizes.

<StaticUseMediaQuery />



## See Also

- [Use Is Mobile](/docs/hooks/use-is-mobile)
