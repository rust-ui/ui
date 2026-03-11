---
title: "Demo Accordion Rtl"
name: "demo_accordion_rtl"
cargo_dependencies: []
registry_dependencies: ["accordion", "direction_provider"]
type: "components:demos"
path: "demos/demo_accordion_rtl.rs"
---

# Demo Accordion Rtl

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_accordion_rtl
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::accordion::{
    Accordion, AccordionContent, AccordionDescription, AccordionItem, AccordionTitle, AccordionTrigger,
};
use crate::components::ui::direction_provider::{Direction, DirectionProvider};

#[component]
pub fn DemoAccordionRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="w-full max-w-[400px]">
            <Accordion class="max-w-[400px]">
                <AccordionItem>
                    <AccordionTrigger open=true>
                        <AccordionTitle>"البند الأول"</AccordionTitle>
                    </AccordionTrigger>
                    <AccordionContent class="pt-0">
                        <AccordionDescription>
                            "هذا هو وصف العنصر القابل للطي."
                        </AccordionDescription>
                    </AccordionContent>
                </AccordionItem>
                <AccordionItem>
                    <AccordionTrigger>
                        <AccordionTitle>"البند الثاني"</AccordionTitle>
                    </AccordionTrigger>
                    <AccordionContent class="pt-0">
                        <AccordionDescription>
                            "هذا هو وصف العنصر القابل للطي."
                        </AccordionDescription>
                    </AccordionContent>
                </AccordionItem>
                <AccordionItem>
                    <AccordionTrigger>
                        <AccordionTitle>"البند الثالث"</AccordionTitle>
                    </AccordionTrigger>
                    <AccordionContent class="pt-0">
                        <AccordionDescription>
                            "هذا هو وصف العنصر القابل للطي."
                        </AccordionDescription>
                    </AccordionContent>
                </AccordionItem>
            </Accordion>
        </DirectionProvider>
    }
}
```
