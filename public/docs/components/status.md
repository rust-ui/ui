+++
title = "Status"
description = "Rust/UI component for displaying statuses."
tags = []
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++



<StaticStatus />




## Installation

<StaticInstallStatus />






## Usage

```rust
use crate::components::ui::status::{Status, StatusIndactor, StatusIndactorVariant};
```

```rust
<Status variant=StatusIndactorVariant::Active>
    <span>"Online"</span>
</Status>
```








## Examples

### Variants

Status indicator variants including success, warning, error, and info states with [Badge](/docs/components/badge) styling. This example demonstrates how to create consistent status displays in Leptos with color-coded indicators and proper ARIA labels for accessibility in Rust applications.

<StaticStatusVariants />


## See Also

- [Badge](/docs/components/badge)
- [Spinner](/docs/components/spinner)
