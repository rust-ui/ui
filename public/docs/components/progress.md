+++
title = "Progress"
description = "Rust/UI component that displays a progress bar indicating task completion."
tags = []
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<StaticProgress />

## Installation

<StaticInstallProgress />

## Usage

```rust
use crate::ui::progress::Progress;
```

```rust
rsx! {
    Progress { value: 60.0 }
}
```

## Examples

### Animated

Use a Dioxus signal to drive the progress value dynamically.

## See Also

- [Spinner](/components/spinner)
- [Skeleton](/components/skeleton)
