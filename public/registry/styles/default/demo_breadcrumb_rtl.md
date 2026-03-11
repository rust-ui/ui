---
title: "Demo Breadcrumb Rtl"
name: "demo_breadcrumb_rtl"
cargo_dependencies: []
registry_dependencies: ["breadcrumb", "direction_provider"]
type: "components:demos"
path: "demos/demo_breadcrumb_rtl.rs"
---

# Demo Breadcrumb Rtl

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_breadcrumb_rtl
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::breadcrumb::{
    Breadcrumb, BreadcrumbEllipsis, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage, BreadcrumbSeparator,
};
use crate::components::ui::direction_provider::{Direction, DirectionProvider};

#[component]
pub fn DemoBreadcrumbRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="w-full max-w-lg">
            <Breadcrumb>
                <BreadcrumbList>
                    <BreadcrumbItem>
                        <BreadcrumbLink attr:href="/">"الرئيسية"</BreadcrumbLink>
                    </BreadcrumbItem>

                    <BreadcrumbSeparator />

                    <BreadcrumbItem>
                        <BreadcrumbEllipsis />
                    </BreadcrumbItem>

                    <BreadcrumbSeparator />

                    <BreadcrumbItem>
                        <BreadcrumbLink attr:href="/demo-components/button">"المكونات"</BreadcrumbLink>
                    </BreadcrumbItem>

                    <BreadcrumbSeparator />

                    <BreadcrumbItem>
                        <BreadcrumbPage>"التوثيق"</BreadcrumbPage>
                    </BreadcrumbItem>
                </BreadcrumbList>
            </Breadcrumb>
        </DirectionProvider>
    }
}
```
