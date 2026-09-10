+++
title = "Toast"
description = "Rust/UI component that displays toast notifications."
tags = []
is_new = false
image = "/images/thumbnails/toast.webp"
image_dark = "/images/thumbnails/toast-dark.webp"
+++

<StaticToast />

## Variants

<StaticToastVariants />

## Installation

<StaticInstallToast />

## Usage

```rust
use crate::ui::toast_custom::_builder::ToastBuilder;
use crate::ui::toast_custom::_data::ToastLevel;
use crate::ui::toast_custom::toaster::{expect_toaster, Toaster};
```

```rust
rsx! {
    Toaster {}
}

let toaster = expect_toaster();
toaster.toast(ToastBuilder::new("Hello World!"));
toaster.toast(ToastBuilder::new("Success!").with_level(ToastLevel::Success));
toaster.toast(ToastBuilder::new("Warning!").with_level(ToastLevel::Warn));
toaster.toast(ToastBuilder::new("Error!").with_level(ToastLevel::Error));
```

## Examples

### Basic Toast

Minimal toast notification flow.

<StaticToast />

### Variants

Toast styles for different notification levels.

<StaticToastVariants />

## See Also

- [Sonner](/components/sonner)
- [Alert](/components/alert)
