---
title: "Demo Pagination"
name: "demo_pagination"
cargo_dependencies: []
registry_dependencies: ["pagination"]
type: "components:demos"
path: "demos/demo_pagination.rs"
---

# Demo Pagination

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_pagination
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::pagination::{
    PageDirection, Pagination, PaginationItem, PaginationLink, PaginationList, PaginationNavButton,
};

#[component]
pub fn DemoPagination() -> impl IntoView {
    view! {
        <Pagination>
            <PaginationList>
                <PaginationItem>
                    <PaginationNavButton direction=PageDirection::Previous />
                </PaginationItem>
                <PaginationItem>
                    <PaginationLink page=1 />
                </PaginationItem>
                <PaginationItem>
                    <PaginationLink page=2 />
                </PaginationItem>
                <PaginationItem>
                    <PaginationLink page=3 />
                </PaginationItem>
                <PaginationItem>
                    <PaginationLink page=4 />
                </PaginationItem>
                <PaginationItem>
                    <PaginationLink page=5 />
                </PaginationItem>
                <PaginationItem>
                    <PaginationNavButton direction=PageDirection::Next />
                </PaginationItem>
            </PaginationList>
        </Pagination>
    }
}
```
