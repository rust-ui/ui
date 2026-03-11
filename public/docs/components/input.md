+++
title = "Input"
description = "Rust/UI component that displays an input field that allows the user to enter text."
tags = ["input"]
is_new = false
image = "/images/thumbnails/input.webp"
image_dark = "/images/thumbnails/input-dark.webp"
+++


<StaticInput />



## Usage

You can use the `Input` component in combination with the [Button](/docs/components/button) component.

```rust
use crate::components::ui::input::Input;
```

```rust
<Input placeholder="Enter text..." />
```



## Examples

### Input Copy

Input field with integrated copy-to-clipboard functionality for easy text sharing. This example shows how to build interactive input components in Leptos with [Button](/docs/components/button) integration, clipboard API, and visual feedback for enhanced user experience in Rust applications.

<StaticInputCopy />


## Installation

<StaticInstallInput />




### RTL

Text input with an Arabic label and placeholder. The cursor starts on the right and text flows right-to-left.

<StaticInputRtl />

## See Also

- [Form](/docs/components/form)
- [Label](/docs/components/label)
- [Input Group](/docs/components/input-group)
- [Input OTP](/docs/components/input-otp)
- [Input Phone](/docs/components/input-phone)
