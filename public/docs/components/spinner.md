+++
title = "Spinner"
description = "A loading spinner component with animation for indicating processing states."
tags = ["animation", "utils"]
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<StaticSpinner />

## Installation

<StaticInstallSpinner />

## Usage

```rust
use crate::ui::spinner::Spinner;
```

```rust
rsx! {
    Spinner {}
}
```

## Examples

### In Button

Loading spinner integrated within button components for async action feedback. This example demonstrates how to combine Spinner and [Button](/components/button) in Dioxus to create accessible loading states with proper visual indicators.

<StaticSpinnerButton />

## See Also

- [Skeleton](/components/skeleton)
- [Button](/components/button)
