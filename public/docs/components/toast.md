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

```bash
Coming soon
```





## Usage

```rust
use crate::components::toast_custom::_builder::ToastBuilder;
use crate::components::toast_custom::_data::ToastLevel;
use crate::components::toast_custom::toaster::{expect_toaster, Toaster};
```

```rust
// Add the Toaster component in your app root
<Toaster />

// Create and show a toast
let toaster = expect_toaster();
toaster.toast(ToastBuilder::new("Hello World!"));

// With different levels
toaster.toast(ToastBuilder::new("Success!").with_level(ToastLevel::Success));
toaster.toast(ToastBuilder::new("Warning!").with_level(ToastLevel::Warn));
toaster.toast(ToastBuilder::new("Error!").with_level(ToastLevel::Error));
```


## See Also

- [Sonner](/docs/components/sonner)
- [Alert](/docs/components/alert)
