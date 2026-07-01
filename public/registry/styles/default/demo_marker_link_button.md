---
title: "Demo Marker Link Button"
name: "demo_marker_link_button"
cargo_dependencies: ["icons/leptos"]
registry_dependencies: ["marker"]
type: "components:demos"
path: "demos/demo_marker_link_button.rs"
---

# Demo Marker Link Button

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_marker_link_button
```

## Component Code

```rust
// TODO PORT: shadcn demo uses `render={<a href="..." />}` and `render={<button onClick={toast(...)} />}`
// via the Radix asChild / render prop pattern. We split into:
//   - `href` prop → renders <a> (link case)
//   - `on_click` prop → renders <button> (action case)
// Also: shadcn calls `toast()` from sonner. We use `web_sys::window().alert_with_message()`
// since wiring up a toast in a standalone demo is disproportionate.
use icons::{GitBranch, RotateCcw};
use leptos::prelude::*;

use crate::components::ui::marker::{Marker, MarkerContent, MarkerIcon};

#[component]
pub fn DemoMarkerLinkButton() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-8 py-12 w-full max-w-sm">
            <Marker href="#links-and-buttons">
                <MarkerIcon>
                    <GitBranch />
                </MarkerIcon>
                <MarkerContent>"View the pull request"</MarkerContent>
            </Marker>
            <Marker
                class="transition-colors hover:text-foreground"
                on_click=Callback::new(move |_| {
                    web_sys::window().and_then(|w| w.alert_with_message("You clicked the revert button").ok());
                })
            >
                <MarkerIcon>
                    <RotateCcw />
                </MarkerIcon>
                <MarkerContent>"Revert this change"</MarkerContent>
            </Marker>
        </div>
    }
}
```
