---
title: "Demo Docker"
name: "demo_docker"
cargo_dependencies: []
registry_dependencies: []
type: "components:demos"
path: "demos/demo_docker.rs"
---

# Demo Docker

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_docker
```

## Component Code

```rust
use icons::FileQuestion;
use leptos::prelude::*;

#[component]
pub fn DemoDocker() -> impl IntoView {
    let button_titles = vec!["Settings", "Browser", "Mail", "Map", "Messages", "Music Player", "Apps", "Documents"];

    view! {
        <link rel="stylesheet" href="/components/docker.css" />
        // <script src="/xx.js"></script>

        <div class="h-[600px]">

            <h1 class="text-3xl font-bold lg:text-4xl text-pretty">Dock magnification using <code>:has()</code></h1>
            <nav class="flex fixed gap-1 justify-center items-end dockerNav">
                {button_titles
                    .into_iter()
                    .map(|title| {
                        view! {
                            <button
                                type="button"
                                class="relative border-none transition-all duration-300 ease-in-out cursor-pointer outline-hidden bg-[rgba(0,0,0,0.75)] text-[rgba(215,255,255,1)] w-[var(--btn-width,var(--btn-size))] h-[var(--btn-height,var(--btn-size))] aspect-ratio-1 rounded-[calc(var(--btn-size)*.25)]"
                                data-title=title
                            >
                                <FileQuestion class="size-14" />
                            </button>
                        }
                    })
                    .collect::<Vec<_>>()}
            </nav>

        </div>
    }
}
```
