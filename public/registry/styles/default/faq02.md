---
title: "Faq02"
name: "faq02"
cargo_dependencies: []
registry_dependencies: ["accordion"]
type: "components:blocks"
path: "blocks/faq02.rs"
---

# Faq02

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add faq02
```

## Component Code

```rust
use icons::{Clock, CreditCard, Globe, Package, Truck};
use leptos::prelude::*;

use crate::components::ui::accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger};

const ACCORDION_ITEM_CLASS: &str = "px-4 rounded-lg border last:border-b bg-background shadow-xs";
const ACCORDION_TRIGGER_CLASS: &str = "flex-1 gap-4 py-5 text-sm font-medium";

#[component]
pub fn Faq02() -> impl IntoView {
    view! {
        <section class="py-20 bg-muted">
            <div class="px-4 mx-auto max-w-5xl md:px-6">
                <div class="flex flex-col gap-10 md:flex-row md:gap-16">
                    <div class="md:w-1/3">
                        <div class="sticky top-20">
                            <h2 class="mt-4 text-3xl font-bold">Frequently Asked Questions</h2>
                            <p class="mt-4 text-muted-foreground">
                                "Can't find what you're looking for? Contact our "
                                <a class="font-medium hover:underline text-primary" href="#">
                                    customer support team
                                </a>
                            </p>
                        </div>
                    </div>
                    <div class="md:w-2/3">
                        <Accordion class="space-y-2 w-full">
                            <AccordionItem class=ACCORDION_ITEM_CLASS>
                                <AccordionTrigger class=ACCORDION_TRIGGER_CLASS>
                                    <div class="flex gap-3 items-center">
                                        <Clock />
                                        <span class="text-base">What are your business hours?</span>
                                    </div>
                                </AccordionTrigger>
                                <AccordionContent class="pb-4 text-muted-foreground">
                                    Our customer support team is available Monday through Friday, 9 AM to 6 PM EST. We also offer limited support on weekends from 10 AM to 4 PM EST.
                                </AccordionContent>
                            </AccordionItem>
                            <AccordionItem class=ACCORDION_ITEM_CLASS>
                                <AccordionTrigger class=ACCORDION_TRIGGER_CLASS>
                                    <div class="flex gap-3 items-center">
                                        <CreditCard />
                                        <span class="text-base">How do subscription payments work?</span>
                                    </div>
                                </AccordionTrigger>
                                <AccordionContent class="pb-4 text-muted-foreground">
                                    Subscriptions are billed automatically on a monthly or annual basis, depending on your chosen plan. You can manage your subscription and payment methods in your account settings.
                                </AccordionContent>
                            </AccordionItem>
                            <AccordionItem class=ACCORDION_ITEM_CLASS>
                                <AccordionTrigger class=ACCORDION_TRIGGER_CLASS>
                                    <div class="flex gap-3 items-center">
                                        <Truck />
                                        <span class="text-base">Can I expedite my shipping?</span>
                                    </div>
                                </AccordionTrigger>
                                <AccordionContent class="pb-4 text-muted-foreground">
                                    Yes, we offer expedited shipping options at checkout. Express shipping typically delivers within 2-3 business days, while overnight shipping guarantees next-day delivery.
                                </AccordionContent>
                            </AccordionItem>
                            <AccordionItem class=ACCORDION_ITEM_CLASS>
                                <AccordionTrigger class=ACCORDION_TRIGGER_CLASS>
                                    <div class="flex gap-3 items-center">
                                        <Globe />
                                        <span class="text-base">Do you offer localized support?</span>
                                    </div>
                                </AccordionTrigger>
                                <AccordionContent class="pb-4 text-muted-foreground">
                                    We provide support in multiple languages including English, Spanish, French, and German. Our team can assist you in your preferred language during business hours.
                                </AccordionContent>
                            </AccordionItem>
                            <AccordionItem class=ACCORDION_ITEM_CLASS>
                                <AccordionTrigger class=ACCORDION_TRIGGER_CLASS>
                                    <div class="flex gap-3 items-center">
                                        <Package />
                                        <span class="text-base">How do I track my order?</span>
                                    </div>
                                </AccordionTrigger>
                                <AccordionContent class="pb-4 text-muted-foreground">
                                    Once your order ships, you will receive a tracking number via email. You can use this number to track your package on our website or directly on the carrier site.
                                </AccordionContent>
                            </AccordionItem>
                        </Accordion>
                    </div>
                </div>
            </div>
        </section>
    }
}
```
