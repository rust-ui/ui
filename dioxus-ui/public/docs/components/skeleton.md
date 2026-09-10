+++
title = "Skeleton"
description = "Rust/UI component that show a placeholder while content is loading."
tags = []
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<StaticSkeleton />

## Installation

<StaticInstallSkeleton />

## Usage

```rust
use crate::ui::skeleton::Skeleton;
```

```rust
rsx! {
    Skeleton { class: "w-[100px] h-[20px] rounded" }
}
```

## Examples

### Skeleton Image

Skeleton loader placeholder for image content with proper aspect ratio preservation. This example shows how to implement skeleton screens in Dioxus for better perceived performance while card or avatar content loads.

<StaticSkeletonImage />

### Skeleton Avatar

Placeholder pattern for lists of user profiles — circle avatar combined with name and subtitle lines.

<StaticSkeletonAvatar />

### Skeleton Text

Multi-line text block placeholder with varying widths to simulate natural paragraph flow.

<StaticSkeletonText />

### Skeleton Form

Placeholder for form layouts showing label and input field pairs while content loads.

<StaticSkeletonForm />

### Skeleton Table

Row-based placeholder for tabular data with varying cell widths to simulate realistic table content.

<StaticSkeletonTable />

## See Also

- [Spinner](/components/spinner)
- [Card](/components/card)
