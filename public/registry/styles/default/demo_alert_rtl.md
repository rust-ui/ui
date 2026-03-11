---
title: "Demo Alert Rtl"
name: "demo_alert_rtl"
cargo_dependencies: []
registry_dependencies: ["alert", "direction_provider"]
type: "components:demos"
path: "demos/demo_alert_rtl.rs"
---

# Demo Alert Rtl

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_alert_rtl
```

## Component Code

```rust
use icons::Terminal;
use leptos::prelude::*;

use crate::components::ui::alert::{Alert, AlertDescription, AlertTitle};
use crate::components::ui::direction_provider::{Direction, DirectionProvider};

#[component]
pub fn DemoAlertRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="w-full max-w-lg">
            <Alert>
                <Terminal />
                <AlertTitle>"تنبيه!"</AlertTitle>
                <AlertDescription>
                    "يمكنك إضافة المكونات إلى تطبيقك باستخدام واجهة سطر الأوامر."
                </AlertDescription>
            </Alert>
        </DirectionProvider>
    }
}
```
