


```rust
use icons::{OctagonAlert, Truck, WalletCards};
use leptos::prelude::*;

use crate::components::ui::accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger};
use crate::components::ui::button::{Button, ButtonVariant};

const ACCORDION_ITEM_CLASS: &str = "py-1 px-6 rounded-xl border-none data-[state=open]:bg-card data-[state=open]:ring-foreground/5 data-[state=open]:shadow-sm data-[state=open]:ring-1";
const ACCORDION_TRIGGER_CLASS: &str =
    "flex-1 gap-4 items-start py-4 text-base font-medium rounded-none border-b data-[state=open]:border-transparent";

#[component]
pub fn Faq03() -> impl IntoView {
    view! {
        <main role="main">
            <div data-theme="quartz" class="scheme-light bg-background">
                <section class="py-16 w-full md:py-24 bg-muted">
                    <div class="px-1 mx-auto max-w-5xl md:px-6">
                        <div class="max-w-lg max-md:px-6">
                            <h2 class="text-4xl font-semibold text-foreground">FAQs</h2>
                            <p class="mt-4 text-lg text-muted-foreground text-balance">
                                Discover quick and comprehensive answers to common questions about our platform, services, and features.
                            </p>
                        </div>
                        <div class="grid mt-6 md:grid-cols-5 md:mt-20 @container">
                            <div class="flex sticky top-0 z-10 flex-col gap-2 md:top-12 md:col-span-2 md:-mt-3 max-md:bg-muted h-fit max-md:justify-center max-md:p-2">
                                <Button variant=ButtonVariant::Outline attr:href="#general">
                                    <OctagonAlert />
                                    <span>General</span>
                                </Button>
                                <Button variant=ButtonVariant::Outline attr:href="#shipping">
                                    <Truck />
                                    <span>Shipping</span>
                                </Button>
                                <Button variant=ButtonVariant::Outline attr:href="#payment">
                                    <WalletCards />
                                    <span>Payment</span>
                                </Button>
                            </div>
                            <div class="space-y-12 md:col-span-3 max-md:mt-6">
                                <div class="space-y-4" id="general" data-faq-group="general">
                                    <h3 class="pl-6 text-lg font-semibold text-foreground">General</h3>
                                    <Accordion class="-space-y-1">
                                        <AccordionItem class=ACCORDION_ITEM_CLASS>
                                            <AccordionTrigger class=ACCORDION_TRIGGER_CLASS>
                                                How long does shipping take?
                                            </AccordionTrigger>
                                            <AccordionContent class="py-4 text-muted-foreground">
                                                Standard shipping typically takes 5-7 business days. Express shipping options are available for 2-3 day delivery.
                                            </AccordionContent>
                                        </AccordionItem>
                                        <AccordionItem class=ACCORDION_ITEM_CLASS>
                                            <AccordionTrigger class=ACCORDION_TRIGGER_CLASS>
                                                What payment methods do you accept?
                                            </AccordionTrigger>
                                            <AccordionContent class="py-4 text-muted-foreground">
                                                We accept all major credit cards, PayPal, Apple Pay, and Google Pay for your convenience.
                                            </AccordionContent>
                                        </AccordionItem>
                                        <AccordionItem class=ACCORDION_ITEM_CLASS>
                                            <AccordionTrigger class=ACCORDION_TRIGGER_CLASS>
                                                Can I change or cancel my order?
                                            </AccordionTrigger>
                                            <AccordionContent class="py-4 text-muted-foreground">
                                                You can modify or cancel your order within 24 hours of placing it. After that, please contact our support team.
                                            </AccordionContent>
                                        </AccordionItem>
                                        <AccordionItem class=ACCORDION_ITEM_CLASS>
                                            <AccordionTrigger class=ACCORDION_TRIGGER_CLASS>
                                                Do you offer gift wrapping?
                                            </AccordionTrigger>
                                            <AccordionContent class="py-4 text-muted-foreground">
                                                Yes, we offer complimentary gift wrapping on all orders. You can select this option at checkout.
                                            </AccordionContent>
                                        </AccordionItem>
                                        <AccordionItem class=ACCORDION_ITEM_CLASS>
                                            <AccordionTrigger class=ACCORDION_TRIGGER_CLASS>
                                                How can I track my order?
                                            </AccordionTrigger>
                                            <AccordionContent class="py-4 text-muted-foreground">
                                                "Once your order ships, you'll receive a tracking number via email that you can use to monitor your delivery."
                                            </AccordionContent>
                                        </AccordionItem>
                                    </Accordion>
                                </div>
                                <div class="space-y-4" id="shipping" data-faq-group="shipping">
                                    <h3 class="pl-6 text-lg font-semibold text-foreground">Shipping</h3>
                                    <Accordion class="-space-y-1">
                                        <AccordionItem class=ACCORDION_ITEM_CLASS>
                                            <AccordionTrigger class=ACCORDION_TRIGGER_CLASS>
                                                Do you ship internationally?
                                            </AccordionTrigger>
                                            <AccordionContent class="py-4 text-muted-foreground">
                                                Yes, we ship to most countries worldwide. International shipping times vary by destination.
                                            </AccordionContent>
                                        </AccordionItem>
                                        <AccordionItem class=ACCORDION_ITEM_CLASS>
                                            <AccordionTrigger class=ACCORDION_TRIGGER_CLASS>
                                                What is your return policy?
                                            </AccordionTrigger>
                                            <AccordionContent class="py-4 text-muted-foreground">
                                                We offer a 30-day return policy for unused items in original packaging. Return shipping is free for US customers.
                                            </AccordionContent>
                                        </AccordionItem>
                                        <AccordionItem class=ACCORDION_ITEM_CLASS>
                                            <AccordionTrigger class=ACCORDION_TRIGGER_CLASS>
                                                What are your shipping rates?
                                            </AccordionTrigger>
                                            <AccordionContent class="py-4 text-muted-foreground">
                                                Shipping rates are calculated based on weight and destination. Free shipping on orders over $50.
                                            </AccordionContent>
                                        </AccordionItem>
                                    </Accordion>
                                </div>
                                <div class="space-y-4" id="payment" data-faq-group="payment">
                                    <h3 class="pl-6 text-lg font-semibold text-foreground">Payment</h3>
                                    <Accordion class="-space-y-1">
                                        <AccordionItem class=ACCORDION_ITEM_CLASS>
                                            <AccordionTrigger class=ACCORDION_TRIGGER_CLASS>
                                                What currencies do you accept?
                                            </AccordionTrigger>
                                            <AccordionContent class="py-4 text-muted-foreground">
                                                We accept USD, EUR, GBP, and CAD. Prices are automatically converted based on current exchange rates.
                                            </AccordionContent>
                                        </AccordionItem>
                                        <AccordionItem class=ACCORDION_ITEM_CLASS>
                                            <AccordionTrigger class=ACCORDION_TRIGGER_CLASS>
                                                Is my payment information secure?
                                            </AccordionTrigger>
                                            <AccordionContent class="py-4 text-muted-foreground">
                                                Yes, we use industry-standard SSL encryption to protect all payment information. We never store your credit card details.
                                            </AccordionContent>
                                        </AccordionItem>
                                        <AccordionItem class=ACCORDION_ITEM_CLASS>
                                            <AccordionTrigger class=ACCORDION_TRIGGER_CLASS>
                                                Can I get a refund if I change my mind?
                                            </AccordionTrigger>
                                            <AccordionContent class="py-4 text-muted-foreground">
                                                Yes, refunds are processed within 5-7 business days after we receive your return. The original payment method will be credited.
                                            </AccordionContent>
                                        </AccordionItem>
                                    </Accordion>
                                </div>
                            </div>
                        </div>
                    </div>
                </section>
            </div>
        </main>
    }
}
```