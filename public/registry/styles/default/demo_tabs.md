---
title: "Demo Tabs"
name: "demo_tabs"
cargo_dependencies: []
registry_dependencies: ["tabs"]
type: "components:demos"
path: "demos/demo_tabs.rs"
---

# Demo Tabs

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_tabs
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::tabs::{Tabs, TabsContent, TabsList, TabsTrigger};

#[component]
pub fn DemoTabs() -> impl IntoView {
    view! {
        <Tabs default_value="account" class="w-full max-w-sm">
            <TabsList>
                <TabsTrigger value="account">Account</TabsTrigger>
                <TabsTrigger value="password">Password</TabsTrigger>
                <TabsTrigger value="settings">Settings</TabsTrigger>
            </TabsList>
            <TabsContent value="account">
                <p class="text-muted-foreground">Manage your account settings and profile information.</p>
            </TabsContent>
            <TabsContent value="password">
                <p class="text-muted-foreground">Change your password and security preferences.</p>
            </TabsContent>
            <TabsContent value="settings">
                <p class="text-muted-foreground">Configure your notification and privacy settings.</p>
            </TabsContent>
        </Tabs>
    }
}
```
