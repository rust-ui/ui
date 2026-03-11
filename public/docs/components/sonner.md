+++
title = "Sonner"
description = "Rust/UI Toast, inspired by Sonner."
tags = []
is_new = false
image = "/images/thumbnails/toast.webp"
image_dark = "/images/thumbnails/toast-dark.webp"
+++



<StaticSonner />





## Installation

<StaticInstallSonner />


## Components

The Sonner component is composed of several subcomponents:

- **SonnerToaster**: Toast container that renders all active toasts
- **SonnerTrigger**: Button that triggers a toast notification
- **SonnerContainer**: Container wrapper for toast positioning
- **SonnerList**: List container for stacked toasts
- **ToastType**: Enum for toast variants (Success/Error/Warning/Info)
- **SonnerPosition**: Enum for toast position on screen
- **SonnerDirection**: Enum for stack direction



## Usage

```rust
use crate::components::ui::sonner::{
    SonnerToaster,
    SonnerTrigger,
    SonnerContainer,
    SonnerList,
    ToastType,
    SonnerPosition,
    SonnerDirection,
};
```

```rust
<SonnerToaster position=SonnerPosition::BottomRight />

<SonnerTrigger
    variant=ToastType::Success
    title="Success"
    description="Your action was successful."
>
    "Show Toast"
</SonnerTrigger>
```




## Examples

### Positions

Toast notifications can appear at six different positions on the screen. Use the `position` prop on `SonnerToaster` to customize the toast location.

<StaticSonnerPositions />


### Variants

Toast notification variants including success, error, warning, and info styles triggered by [Button](/docs/components/button) actions. This example demonstrates how to implement accessible toast notifications in Leptos with automatic dismissal and stacking behavior for effective user feedback in Rust applications.

<StaticSonnerVariants />



## See Also

- [Toast](/docs/components/toast)
- [Alert](/docs/components/alert)
