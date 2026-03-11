---
title: "Data"
name: "_data"
cargo_dependencies: []
registry_dependencies: []
type: "components:ui/toast_custom"
path: "ui/_data.rs"
---

# Data

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add _data
```

## Component Code

```rust
use leptos::prelude::*;

pub type ToastId = u64;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ToastLevel {
    Info,
    Success,
    Warn,
    Error,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ToastPosition {
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
}

/* ========================================================== */
/*                       🧬 STRUCT 🧬                         */
/* ========================================================== */

#[derive(Clone, Debug)]
pub struct ToastData {
    pub id: ToastId,
    pub expiry: Option<u32>,
    pub message: String,
    pub progress: bool,
    pub dismissable: bool,
    pub clear_signal: RwSignal<bool>,

    pub level: ToastLevel,
    pub position: ToastPosition,
}
```
