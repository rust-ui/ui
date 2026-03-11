---
title: "Demo Theme Toggle"
name: "demo_theme_toggle"
cargo_dependencies: []
registry_dependencies: ["theme_toggle"]
type: "components:demos"
path: "demos/demo_theme_toggle.rs"
---

# Demo Theme Toggle

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_theme_toggle
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::theme_toggle::ThemeToggle;

#[component]
pub fn DemoThemeToggle() -> impl IntoView {
    view! { <ThemeToggle /> }
}
```
