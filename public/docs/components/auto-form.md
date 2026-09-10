+++
title = "AutoForm"
description = "Automatically generate form UI from Rust structs using a derive macro with built-in validation support."
tags = ["input"]
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<StaticAutoForm />

## Installation

<StaticInstallAutoForm />

## Usage

```rust
use crate::ui::auto_form::AutoForm;
```

```rust
rsx! {
    DemoAutoForm {}
}
```

## Examples

### Generated Form

Automatically generated form fields from a typed schema. Useful when you want strong typing and a fast way to scaffold forms in Dioxus.

<StaticAutoForm />

## See Also

- [Form](/components/form)
- [Input](/components/input)
