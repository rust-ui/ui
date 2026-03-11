---
title: "Skeleton"
name: "skeleton"
cargo_dependencies: []
registry_dependencies: []
type: "components:ui"
path: "ui/skeleton.rs"
description: "Rust/UI component that show a placeholder while content is loading."
tags: []
---

# Skeleton

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add skeleton
```

## Component Code

```rust
use leptos::prelude::*;
use leptos_ui::void;

const PULSE_ANIMATION: &str = "animate-pulse";

// TODO UI. Skeleton should be able to receive children (or not).

mod components {
    use super::*;
    void! {Skeleton, div, PULSE_ANIMATION, "rounded-md bg-muted"}
}

pub use components::*;
```
