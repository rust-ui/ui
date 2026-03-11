---
title: "Demo Button Stateful"
name: "demo_button_stateful"
cargo_dependencies: []
registry_dependencies: ["button"]
type: "components:demos"
path: "demos/demo_button_stateful.rs"
---

# Demo Button Stateful

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_button_stateful
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::button::Button;

#[component]
pub fn DemoButtonStateful() -> impl IntoView {
    view! {
        <style>
            {r#"
            @media (prefers-reduced-motion: no-preference) {
            ::view-transition-old(button-stateful),
            ::view-transition-new(button-stateful) {
            height: 100%;
            width: 100%;
            }
            }
            "#}
        </style>

        <Button attr:id="ButtonStateful" attr:style="view-transition-name: button-stateful;">
            Do some hard work
        </Button>

        <script>
            {r#"
            (() => {
            const STATES = {
            idle: "Do some hard work",
            working: "⏳ working...",
            done: "Done! ✅",
            };
            
            ButtonStateful.onclick = () => {
            setState("working");
            setTimeout(() => setState("done"), 2000);
            setTimeout(() => setState("idle"), 4000);
            };
            
            function setState(state) {
            if (!document.startViewTransition) ButtonStateful.innerHTML = STATES[state];
            else document.startViewTransition(() => (ButtonStateful.innerHTML = STATES[state]));
            }
            })();
            "#}
        </script>
    }
}
```
