---
title: "Demo Tabs Line"
name: "demo_tabs_line"
cargo_dependencies: []
registry_dependencies: ["tabs"]
type: "components:demos"
path: "demos/demo_tabs_line.rs"
---

# Demo Tabs Line

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_tabs_line
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::tabs::{Tabs, TabsContent, TabsList, TabsTrigger, TabsVariant};

#[component]
pub fn DemoTabsLine() -> impl IntoView {
    view! {
        <Tabs default_value="preview" class="w-full max-w-sm">
            <TabsList variant=TabsVariant::Line>
                <TabsTrigger value="preview">Preview</TabsTrigger>
                <TabsTrigger value="code">Code</TabsTrigger>
                <TabsTrigger value="output">Output</TabsTrigger>
            </TabsList>
            <TabsContent value="preview">
                <p class="text-muted-foreground">Live preview of your component.</p>
            </TabsContent>
            <TabsContent value="code">
                <p class="text-muted-foreground">View and edit the source code.</p>
            </TabsContent>
            <TabsContent value="output">
                <p class="text-muted-foreground">Compiled output and build logs.</p>
            </TabsContent>
        </Tabs>
    }
}
```
