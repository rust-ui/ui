---
title: "Alert"
name: "alert"
cargo_dependencies: []
registry_dependencies: []
type: "components:ui"
path: "ui/alert.rs"
description: "Rust/UI component that displays a callout to the user."
tags: []
---

# Alert

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add alert
```

## Component Code

```rust
use leptos::prelude::*;
use leptos_ui::clx;

mod components {
    use super::*;
    clx! {Alert, div, "relative w-full rounded-lg border px-4 py-3 text-sm [&>svg+div]:translate-y-[-3px] [&>svg]:absolute [&>svg]:left-4 [&>svg]:top-4 [&>svg]:text-foreground [&>svg~*]:pl-7"}
    clx! {AlertTitle, h4, "mb-1 font-medium tracking-tight leading-none"}
    clx! {AlertDescription, p, "text-sm [&_p]:leading-relaxed"}
}

pub use components::*;
```
