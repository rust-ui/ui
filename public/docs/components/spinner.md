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
use crate::components::ui::spinner::Spinner;
```

```rust
<Spinner />
```




## Examples

### In Button

Loading spinner integrated within button components for async action feedback. This example demonstrates how to combine Spinner and [Button](/docs/components/button) in Leptos to create accessible loading states with proper ARIA attributes and visual indicators in Rust applications.

<StaticSpinnerButton />


### RTL

Spinner in an RTL button context. The `inline-start` / `inline-end` slot respects the RTL reading direction so the spinner stays on the correct side of the label.

<StaticSpinnerRtl />

## See Also

- [Skeleton](/docs/components/skeleton)
- [Shimmer](/docs/components/shimmer)
- [Animate](/docs/components/animate)
