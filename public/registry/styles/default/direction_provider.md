---
title: "Direction Provider"
name: "direction_provider"
cargo_dependencies: []
registry_dependencies: []
type: "components:ui"
path: "ui/direction_provider.rs"
---

# Direction Provider

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add direction_provider
```

## Component Code

```rust
use leptos::prelude::*;
use strum::AsRefStr;
use tw_merge::*;

#[derive(Default, Clone, Copy, PartialEq, Eq, AsRefStr, strum::Display)]
#[strum(serialize_all = "lowercase")]
pub enum Direction {
    #[default]
    Ltr,
    Rtl,
}

#[component]
pub fn DirectionProvider(
    children: Children,
    #[prop(optional)] dir: Direction,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let class = tw_merge!(class);

    view! {
        <div data-slot="direction-provider" dir=dir.to_string() class=class>
            {children()}
        </div>
    }
}
```
