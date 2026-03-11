---
title: "Demo Card Rtl"
name: "demo_card_rtl"
cargo_dependencies: []
registry_dependencies: ["button", "card", "direction_provider"]
type: "components:demos"
path: "demos/demo_card_rtl.rs"
---

# Demo Card Rtl

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_card_rtl
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle};
use crate::components::ui::direction_provider::{Direction, DirectionProvider};

#[component]
pub fn DemoCardRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="w-full max-w-lg">
            <Card class="max-w-lg">
                <CardHeader>
                    <CardTitle>"عنوان البطاقة"</CardTitle>
                </CardHeader>

                <CardContent>
                    <CardDescription>
                        "مرحباً، هذا وصف تفصيلي لمحتوى البطاقة. يمكنك إضافة المزيد من النصوص هنا لتوفير معلومات إضافية حول غرض البطاقة وميزاتها وأي تفاصيل أخرى ذات صلة."
                    </CardDescription>
                </CardContent>

                <CardFooter class="justify-end">
                    <Button variant=ButtonVariant::Outline>"إلغاء"</Button>
                    <Button>"تأكيد"</Button>
                </CardFooter>
            </Card>
        </DirectionProvider>
    }
}
```
