---
title: "Demo Tabs Vertical"
name: "demo_tabs_vertical"
cargo_dependencies: []
registry_dependencies: ["tabs"]
type: "components:demos"
path: "demos/demo_tabs_vertical.rs"
---

# Demo Tabs Vertical

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_tabs_vertical
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::tabs::{Tabs, TabsContent, TabsList, TabsOrientation, TabsTrigger};

#[component]
pub fn DemoTabsVertical() -> impl IntoView {
    view! {
        <Tabs default_value="general" orientation=TabsOrientation::Vertical class="w-full max-w-md">
            <TabsList>
                <TabsTrigger value="general">General</TabsTrigger>
                <TabsTrigger value="security">Security</TabsTrigger>
                <TabsTrigger value="notifications">Notifications</TabsTrigger>
            </TabsList>
            <TabsContent value="general">
                <p class="text-muted-foreground">Configure general application settings.</p>
            </TabsContent>
            <TabsContent value="security">
                <p class="text-muted-foreground">Manage your security preferences and two-factor authentication.</p>
            </TabsContent>
            <TabsContent value="notifications">
                <p class="text-muted-foreground">Control how and when you receive notifications.</p>
            </TabsContent>
        </Tabs>
    }
}
```
