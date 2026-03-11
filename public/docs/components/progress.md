+++
title = "Progress"
description = "Rust/UI component that displays a progress bar indicating task completion."
tags = []
is_new = true
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++


<StaticProgress />




## Installation

<StaticInstallProgress />




## Usage

```rust
use crate::components::ui::progress::Progress;
```

```rust
<Progress value=60.0 />
```


## Examples

### Label

Display a progress bar with a label and percentage indicator.

<StaticProgressLabel />

### Controlled

Control the progress value interactively with a slider.

<StaticProgressControlled />


### RTL

Progress bar in RTL. The fill grows from right to left, matching the visual expectation for right-to-left interfaces.

<StaticProgressRtl />

## See Also

- [Skeleton](/docs/components/skeleton)
- [Spinner](/docs/components/spinner)
- [Slider](/docs/components/slider)
