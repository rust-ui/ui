---
title: "Demo Collapsible Rtl"
name: "demo_collapsible_rtl"
cargo_dependencies: []
registry_dependencies: ["collapsible", "direction_provider"]
type: "components:demos"
path: "demos/demo_collapsible_rtl.rs"
---

# Demo Collapsible Rtl

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_collapsible_rtl
```

## Component Code

```rust
use icons::ChevronsUpDown;
use leptos::prelude::*;

use crate::components::ui::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};
use crate::components::ui::direction_provider::{Direction, DirectionProvider};

#[component]
pub fn DemoCollapsibleRtl() -> impl IntoView {
    let open = RwSignal::new(false);

    view! {
        <DirectionProvider dir=Direction::Rtl class="max-w-fit">
            <Collapsible open=open class="flex flex-col gap-2 w-[350px]">
                <div class="flex gap-4 justify-between items-center px-4">
                    <h4 class="text-sm font-semibold">"طلب رقم #٤١٨٩"</h4>
                    <CollapsibleTrigger class="inline-flex justify-center items-center rounded-md transition-colors size-8 hover:bg-accent hover:text-accent-foreground">
                        <ChevronsUpDown class="size-4" />
                        <span class="sr-only">"تبديل التفاصيل"</span>
                    </CollapsibleTrigger>
                </div>
                <div class="flex justify-between items-center py-2 px-4 text-sm rounded-md border">
                    <span class="text-muted-foreground">"الحالة"</span>
                    <span class="font-medium">"تم الشحن"</span>
                </div>
                <CollapsibleContent class="flex flex-col gap-2">
                    <div class="py-2 px-4 text-sm rounded-md border">
                        <p class="font-medium">"عنوان الشحن"</p>
                        <p class="text-muted-foreground">"١٠٠ شارع السوق، الرياض"</p>
                    </div>
                    <div class="py-2 px-4 text-sm rounded-md border">
                        <p class="font-medium">"العناصر"</p>
                        <p class="text-muted-foreground">"٢x سماعات استوديو"</p>
                    </div>
                </CollapsibleContent>
            </Collapsible>
        </DirectionProvider>
    }
}
```
