---
title: "Demo Accordion Card"
name: "demo_accordion_card"
cargo_dependencies: []
registry_dependencies: ["accordion", "card"]
type: "components:demos"
path: "demos/demo_accordion_card.rs"
---

# Demo Accordion Card

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_accordion_card
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::accordion::{
    Accordion, AccordionContent, AccordionDescription, AccordionItem, AccordionTitle, AccordionTrigger,
};
use crate::components::ui::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};

#[component]
pub fn DemoAccordionCard() -> impl IntoView {
    view! {
        <Card class="max-w-sm">
            <CardHeader>
                <CardTitle>"Subscription & Billing"</CardTitle>
                <CardDescription>"Common questions about plans and payments."</CardDescription>
            </CardHeader>
            <CardContent>
                <Accordion>
                    <AccordionItem>
                        <AccordionTrigger open=true>
                            <AccordionTitle>"What plans do you offer?"</AccordionTitle>
                        </AccordionTrigger>
                        <AccordionContent class="pt-0">
                            <AccordionDescription>
                                "We offer three tiers: Starter, Pro, and Enterprise. Each plan includes different usage limits and feature sets."
                            </AccordionDescription>
                        </AccordionContent>
                    </AccordionItem>
                    <AccordionItem>
                        <AccordionTrigger>
                            <AccordionTitle>"Can I change my plan later?"</AccordionTitle>
                        </AccordionTrigger>
                        <AccordionContent class="pt-0">
                            <AccordionDescription>
                                "Yes, you can upgrade or downgrade your plan at any time from your account settings. Changes take effect on your next billing cycle."
                            </AccordionDescription>
                        </AccordionContent>
                    </AccordionItem>
                    <AccordionItem>
                        <AccordionTrigger>
                            <AccordionTitle>"How do I cancel my subscription?"</AccordionTitle>
                        </AccordionTrigger>
                        <AccordionContent class="pt-0">
                            <AccordionDescription>
                                "You can cancel anytime from the billing section. Your access continues until the end of the current billing period."
                            </AccordionDescription>
                        </AccordionContent>
                    </AccordionItem>
                </Accordion>
            </CardContent>
        </Card>
    }
}
```
