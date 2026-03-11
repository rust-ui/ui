---
title: "Demo Tabs Rtl"
name: "demo_tabs_rtl"
cargo_dependencies: []
registry_dependencies: ["direction_provider", "tabs"]
type: "components:demos"
path: "demos/demo_tabs_rtl.rs"
---

# Demo Tabs Rtl

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_tabs_rtl
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::direction_provider::{Direction, DirectionProvider};
use crate::components::ui::tabs::{Tabs, TabsContent, TabsList, TabsTrigger};

#[component]
pub fn DemoTabsRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="w-full max-w-sm">
            <Tabs default_value="account" class="w-full max-w-sm">
                <TabsList>
                    <TabsTrigger value="account">"الحساب"</TabsTrigger>
                    <TabsTrigger value="password">"كلمة المرور"</TabsTrigger>
                    <TabsTrigger value="settings">"الإعدادات"</TabsTrigger>
                </TabsList>
                <TabsContent value="account">
                    <p class="text-muted-foreground">
                        "إدارة إعدادات حسابك ومعلومات ملفك الشخصي."
                    </p>
                </TabsContent>
                <TabsContent value="password">
                    <p class="text-muted-foreground">
                        "تغيير كلمة المرور وتفضيلات الأمان."
                    </p>
                </TabsContent>
                <TabsContent value="settings">
                    <p class="text-muted-foreground">"ضبط إعدادات الإشعارات والخصوصية."</p>
                </TabsContent>
            </Tabs>
        </DirectionProvider>
    }
}
```
