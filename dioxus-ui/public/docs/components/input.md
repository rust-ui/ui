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

You can use the `Input` component in combination with the [Button](/components/button) component.

```rust
use crate::ui::input::Input;
```

```rust
rsx! {
    Input { placeholder: "Enter text..." }
}
```

## Examples

### Input Copy

Input field with integrated copy-to-clipboard functionality for easy text sharing. This example shows how to build interactive input components in Dioxus with [Button](/components/button) integration and clipboard API.

<StaticInputCopy />

## Installation

<StaticInstallInput />

## See Also

- [Button](/components/button)
- [Card](/components/card)
