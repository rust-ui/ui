---
title: "Demo Breadcrumb Dropdown"
name: "demo_breadcrumb_dropdown"
cargo_dependencies: []
registry_dependencies: ["breadcrumb", "dropdown_menu"]
type: "components:demos"
path: "demos/demo_breadcrumb_dropdown.rs"
---

# Demo Breadcrumb Dropdown

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_breadcrumb_dropdown
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::breadcrumb::{
    Breadcrumb, BreadcrumbEllipsis, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage, BreadcrumbSeparator,
};
use crate::components::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuLink, DropdownMenuTrigger,
};

#[component]
pub fn DemoBreadcrumbDropdown() -> impl IntoView {
    view! {
        <Breadcrumb>
            <BreadcrumbList>
                <BreadcrumbItem>
                    <BreadcrumbLink attr:href="/">Home</BreadcrumbLink>
                </BreadcrumbItem>

                <BreadcrumbSeparator />

                <BreadcrumbItem>
                    <DropdownMenu>
                        <DropdownMenuTrigger>
                            <BreadcrumbEllipsis />
                        </DropdownMenuTrigger>
                        <DropdownMenuContent>
                            <DropdownMenuItem>
                                <DropdownMenuLink attr:href="/docs">"Documentation"</DropdownMenuLink>
                            </DropdownMenuItem>
                            <DropdownMenuItem>
                                <DropdownMenuLink attr:href="/docs/components">"Components"</DropdownMenuLink>
                            </DropdownMenuItem>
                            <DropdownMenuItem>
                                <DropdownMenuLink attr:href="/docs/components/button">"Button"</DropdownMenuLink>
                            </DropdownMenuItem>
                        </DropdownMenuContent>
                    </DropdownMenu>
                </BreadcrumbItem>

                <BreadcrumbSeparator />

                <BreadcrumbItem>
                    <BreadcrumbLink attr:href="/docs/components/breadcrumb">Breadcrumb</BreadcrumbLink>
                </BreadcrumbItem>

                <BreadcrumbSeparator />

                <BreadcrumbItem>
                    <BreadcrumbPage>"Dropdown"</BreadcrumbPage>
                </BreadcrumbItem>
            </BreadcrumbList>
        </Breadcrumb>
    }
}
```
