---
title: "Demo Accordion"
name: "demo_accordion"
cargo_dependencies: []
registry_dependencies: ["accordion"]
type: "components:demos"
path: "demos/demo_accordion.rs"
---

# Demo Accordion

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_accordion
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::accordion::{
    Accordion, AccordionContent, AccordionDescription, AccordionItem, AccordionTitle, AccordionTrigger,
};

#[component]
pub fn DemoAccordion() -> impl IntoView {
    view! {
        <Accordion class="max-w-[400px]">
            <AccordionItem>
                <AccordionTrigger open=true>
                    <AccordionTitle>Accordion item 01</AccordionTitle>
                </AccordionTrigger>
                <AccordionContent class="pt-0">
                    <AccordionDescription>"This is the Accordion description"</AccordionDescription>
                </AccordionContent>
            </AccordionItem>
            <AccordionItem>
                <AccordionTrigger>
                    <AccordionTitle>Accordion item 02</AccordionTitle>
                </AccordionTrigger>
                <AccordionContent class="pt-0">
                    <AccordionDescription>"This is the Accordion description"</AccordionDescription>
                </AccordionContent>
            </AccordionItem>
            <AccordionItem>
                <AccordionTrigger>
                    <AccordionTitle>Accordion item 03</AccordionTitle>
                </AccordionTrigger>
                <AccordionContent class="pt-0">
                    <AccordionDescription>"This is the Accordion description"</AccordionDescription>
                </AccordionContent>
            </AccordionItem>
        </Accordion>
    }
}
```
