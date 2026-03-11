---
title: "Marquee"
name: "marquee"
cargo_dependencies: []
registry_dependencies: ["mask"]
type: "components:ui"
path: "ui/marquee.rs"
description: "Rust/UI component that displays an infinite scrolling component that can be used to display text, images, or videos."
tags: []
---

# Marquee

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add marquee
```

## Component Code

```rust
use leptos::prelude::*;
use leptos_ui::clx;
use tw_merge::*;

use crate::components::ui::mask::{Mask, MaskSide};

// TODO UI. Separate the mask from the marquee.

mod components {
    use super::*;
    clx! {MarqueeRow, div,
        "animate__marquee__row",
        "flex flex-row justify-around shrink-0 [gap:var(--gap)]"
    }
}

pub use components::*;

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn Marquee(children: Children) -> impl IntoView {
    view! {
        <style>
            {"@keyframes marquee_horizontal {
            from { transform: translateX(0); }
            to { transform: translateX(calc(-100% - var(--gap))); }
            }
            
            @keyframes marquee_vertical {
            from { transform: translateY(0); }
            to { transform: translateY(calc(-100% - var(--gap))); }
            }
            
            .animate__marquee__row {
            animation-name: marquee_horizontal;
            animation-duration: var(--duration);
            animation-timing-function: linear;
            animation-iteration-count: infinite;
            }
            
            .group:hover .animate__marquee__row {
            animation-play-state: paused;
            }
            "}
        </style>

        <div
            data-name="Marquee"
            class="flex overflow-hidden flex-row p-2 group [--gap:1rem] [gap:var(--gap)] [--duration:20s]"
        >
            {children()}
        </div>
    }
}

#[component]
pub fn MarqueeWrapper(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    let merged_class = tw_merge!(
        "flex overflow-hidden relative flex-col justify-center items-center p-20 w-full h-full md:shadow-xl min-h-[300px] bg-background",
        class
    );

    view! { <div class=merged_class>{children()} <Mask side=MaskSide::Left /> <Mask side=MaskSide::Right /></div> }
}
```
