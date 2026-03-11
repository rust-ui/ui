---
title: "Pricing08"
name: "pricing08"
cargo_dependencies: []
registry_dependencies: ["badge", "button", "card"]
type: "components:blocks"
path: "blocks/pricing08.rs"
---

# Pricing08

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add pricing08
```

## Component Code

```rust
use icons::{Check, Circle};
use leptos::prelude::*;

use crate::components::ui::badge::Badge;
use crate::components::ui::button::Button;
use crate::components::ui::card::{Card, CardContent, CardItem, CardList};

#[component]
pub fn Pricing08() -> impl IntoView {
    let is_yearly_signal = RwSignal::new(false);

    view! {
        <section class="py-20">
            <div class="container">
                <div class="mx-auto mb-20 max-w-3xl text-center">
                    <h2 class="mb-4 text-4xl font-semibold lg:text-5xl">Pricing</h2>
                    <p class="lg:text-lg text-muted-foreground">
                        Lorem ipsum dolor sit amet consectetur adipisicing elit. Quia dignissimos aliquam delectus, quasi earum veniam?
                    </p>
                </div>
                <div class="flex flex-col gap-2 items-center">
                    <span class="text-muted-foreground">Billing cycle</span>
                    <div class="flex items-center p-1 h-12 text-lg rounded-md bg-muted">
                        <div
                            role="radiogroup"
                            aria-required="false"
                            dir="ltr"
                            data-slot="radio-group"
                            class="grid grid-cols-2 gap-3 h-full"
                            tabindex="0"
                            style="outline:none"
                        >
                            <div class=move || {
                                if !is_yearly_signal.get() {
                                    "h-full rounded-md transition-all bg-background"
                                } else {
                                    "h-full rounded-md transition-all"
                                }
                            }>
                                <button
                                    type="button"
                                    role="radio"
                                    aria-checked=move || (!is_yearly_signal.get()).to_string()
                                    data-state=move || if !is_yearly_signal.get() { "checked" } else { "unchecked" }
                                    value="monthly"
                                    data-slot="radio-group-item"
                                    class="hidden rounded-full border outline-none disabled:opacity-50 disabled:cursor-not-allowed border-input text-primary aria-invalid:ring-destructive/20 aria-invalid:border-destructive aspect-square size-4 shrink-0 shadow-xs transition-[color,box-shadow] peer dark:aria-invalid:ring-destructive/40 dark:bg-input/30 focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px]"
                                    id="monthly"
                                    tabindex="-1"
                                    data-radix-collection-item=""
                                >
                                    <span
                                        data-state=move || if !is_yearly_signal.get() { "checked" } else { "unchecked" }
                                        data-slot="radio-group-indicator"
                                        class="flex relative justify-center items-center"
                                    >
                                        <Circle
                                            class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 fill-primary size-2"
                                            attr:aria-hidden="true"
                                        />
                                    </span>
                                </button>
                                <label
                                    data-slot="label"
                                    class="flex gap-2 justify-center items-center px-7 h-full text-sm font-semibold leading-none cursor-pointer select-none group-data-[disabled=true]:pointer-events-none group-data-[disabled=true]:opacity-50 peer-disabled:cursor-not-allowed peer-disabled:opacity-50 text-muted-foreground peer-data-[state=checked]:text-primary"
                                    for="monthly"
                                    on:click=move |_| is_yearly_signal.set(false)
                                >
                                    Monthly
                                </label>
                            </div>
                            <div class=move || {
                                if is_yearly_signal.get() {
                                    "h-full rounded-md transition-all bg-background"
                                } else {
                                    "h-full rounded-md transition-all"
                                }
                            }>
                                <button
                                    type="button"
                                    role="radio"
                                    aria-checked=move || is_yearly_signal.get().to_string()
                                    data-state=move || if is_yearly_signal.get() { "checked" } else { "unchecked" }
                                    value="annually"
                                    data-slot="radio-group-item"
                                    class="hidden rounded-full border outline-none disabled:opacity-50 disabled:cursor-not-allowed border-input text-primary aria-invalid:ring-destructive/20 aria-invalid:border-destructive aspect-square size-4 shrink-0 shadow-xs transition-[color,box-shadow] peer dark:aria-invalid:ring-destructive/40 dark:bg-input/30 focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px]"
                                    id="annually"
                                    tabindex="-1"
                                    data-radix-collection-item=""
                                >
                                    <span
                                        data-state=move || if is_yearly_signal.get() { "checked" } else { "unchecked" }
                                        data-slot="radio-group-indicator"
                                        class="flex relative justify-center items-center"
                                    >
                                        <Circle
                                            class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 fill-primary size-2"
                                            attr:aria-hidden="true"
                                        />
                                    </span>
                                </button>
                                <label
                                    data-slot="label"
                                    class="flex gap-1 justify-center items-center px-7 h-full text-sm font-semibold leading-none cursor-pointer select-none group-data-[disabled=true]:pointer-events-none group-data-[disabled=true]:opacity-50 peer-disabled:cursor-not-allowed peer-disabled:opacity-50 text-muted-foreground peer-data-[state=checked]:text-primary"
                                    for="annually"
                                    on:click=move |_| is_yearly_signal.set(true)
                                >
                                    Yearly
                                    <Badge class="px-1.5 text-green-600 bg-green-100 border-green-200 hover:bg-green-200">
                                        -20%
                                    </Badge>
                                </label>
                            </div>
                        </div>
                    </div>
                    <div class="grid gap-6 mt-10 max-w-3xl md:grid-cols-2">
                        {PRICING_TIERS
                            .iter()
                            .map(|tier| {
                                view! {
                                    <Card>
                                        <CardContent class="flex flex-col gap-5 justify-between h-full">
                                            <div>
                                                <h3 class="mb-4 text-xl font-semibold">{tier.title}</h3>
                                                <span class="text-5xl font-semibold">
                                                    $
                                                    {move || {
                                                        if is_yearly_signal.get() {
                                                            tier.yearly_price
                                                        } else {
                                                            tier.monthly_price
                                                        }
                                                    }}
                                                </span>
                                                <span class="block mb-4 font-semibold">
                                                    {move || {
                                                        if is_yearly_signal.get() { "per year" } else { "per month" }
                                                    }}
                                                </span>
                                                <p class="text-muted-foreground">{tier.description}</p>
                                                <p class="mt-6 mb-3 font-semibold">{tier.features_title}</p>
                                                <CardList class="gap-3">
                                                    {tier
                                                        .features
                                                        .iter()
                                                        .map(|feature| {
                                                            view! {
                                                                <CardItem class="gap-2 [&_svg]:mt-1">
                                                                    <Check attr:aria-hidden="true" />
                                                                    {*feature}
                                                                </CardItem>
                                                            }
                                                        })
                                                        .collect_view()}
                                                </CardList>
                                            </div>
                                            <Button class="has-[>svg]:px-3" href=tier.button_href>
                                                {tier.button_text}
                                            </Button>
                                        </CardContent>
                                    </Card>
                                }
                            })
                            .collect_view()}
                    </div>
                </div>
            </div>
        </section>
    }
}

/* ========================================================== */
/*                     ✨ CONSTANTS ✨                        */
/* ========================================================== */

struct PricingTier {
    title: &'static str,
    monthly_price: u32,
    yearly_price: u32,
    description: &'static str,
    features_title: &'static str,
    features: &'static [&'static str],
    button_text: &'static str,
    button_href: &'static str,
}

const PRICING_TIERS: &[PricingTier] = &[
    PricingTier {
        title: "Basic Plan",
        monthly_price: 79,
        yearly_price: 63,
        description: "Good for small teams, or small businesses just starting out.",
        features_title: "Includes",
        features: &[
            "5 projects limit",
            "5GB storage",
            "Up to 3 users",
            "Support by email only",
            "No time tracking feature",
        ],
        button_text: "Start a free trial",
        button_href: "#",
    },
    PricingTier {
        title: "Pro Plan",
        monthly_price: 299,
        yearly_price: 239,
        description: "Good for medium to large businesses. Get all the features you need.",
        features_title: "Everything in Basic, plus",
        features: &["Unlimited projects", "50GB storage", "Unlimited users", "Priority support"],
        button_text: "Start a free trial",
        button_href: "#",
    },
];
```
