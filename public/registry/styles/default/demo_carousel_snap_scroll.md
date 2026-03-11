---
title: "Demo Carousel Snap Scroll"
name: "demo_carousel_snap_scroll"
cargo_dependencies: []
registry_dependencies: []
type: "components:demos"
path: "demos/demo_carousel_snap_scroll.rs"
---

# Demo Carousel Snap Scroll

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_carousel_snap_scroll
```

## Component Code

```rust
use leptos::prelude::*;

#[component]
pub fn DemoCarouselSnapScroll() -> impl IntoView {
    view! {
        <link rel="stylesheet" href="/components/carousel-snap-scroll.css" />

        <div class="mainDiv">
            <div class="scrollsnap-carousel">
                <div class="slide">
                    <div class="content">
                        <div class="content-wrapper">slide one</div>
                    </div>
                </div>
                <div class="slide">
                    <div class="content">
                        <div class="content-wrapper">slide two</div>
                    </div>
                </div>
                <div class="slide">
                    <div class="content">
                        <div class="content-wrapper">slide three</div>
                    </div>
                </div>
                <div class="slide">
                    <div class="content">
                        <div class="content-wrapper">slide four</div>
                    </div>
                </div>
                <div class="slide">
                    <div class="content">
                        <div class="content-wrapper">slide five</div>
                    </div>
                </div>
                <div class="slide">
                    <div class="content">
                        <div class="content-wrapper">slide six</div>
                    </div>
                </div>
                <div class="slide">
                    <div class="content">
                        <div class="content-wrapper">slide seven</div>
                    </div>
                </div>
                <div class="slide">
                    <div class="content">
                        <div class="content-wrapper">slide eight</div>
                    </div>
                </div>
            </div>

        </div>
    }
}
```
