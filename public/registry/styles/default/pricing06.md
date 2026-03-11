---
title: "Pricing06"
name: "pricing06"
cargo_dependencies: []
registry_dependencies: ["button", "card", "switch"]
type: "components:blocks"
path: "blocks/pricing06.rs"
---

# Pricing06

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add pricing06
```

## Component Code

```rust
use icons::{Check, Info, Minus};
use leptos::prelude::*;
use leptos_ui::clx;

use crate::components::ui::button::Button;
use crate::components::ui::card::{Card, CardContent, CardHeader, CardItem, CardList, CardTitle};
use crate::components::ui::switch::Switch;

#[component]
pub fn Pricing06() -> impl IntoView {
    clx! {FeatureText, span, "flex items-center gap-3"}
    clx! {InfoButton, button, ""}
    clx! {SectionHeader, th, "pb-4 text-sm font-semibold leading-6 pt-8"}

    let is_monthly_signal = RwSignal::new(false);

    view! {
        <section class="py-20">
            <div class="px-6 mx-auto max-w-7xl lg:px-8">
                <div class="mx-auto max-w-4xl text-center">
                    <p class="mt-2 text-4xl font-bold tracking-tight sm:text-5xl">Choose Your Plan</p>
                </div>
                <p class="mx-auto mt-6 max-w-2xl text-lg leading-8 text-center text-muted-foreground">
                    Distinctio et nulla eum soluta et neque labore quibusdam. Saepe et quasi.
                </p>
                <div class="flex flex-col gap-2 items-center mt-10 lg:hidden">
                    <span class="flex gap-3 items-center text-base font-medium">
                        Annual
                        <Switch
                            checked=is_monthly_signal.get()
                            on:input=move |ev| {
                                is_monthly_signal.set(event_target_checked(&ev));
                            }
                        />Monthly
                    </span>
                </div>

                // Mobile pricing cards
                <div class="mx-auto mt-12 space-y-8 max-w-md sm:mt-16 lg:hidden">
                    {PRICING_PLANS
                        .iter()
                        .map(|plan| {
                            let price_display = move || {
                                match &plan.pricing {
                                    PlanPricing::Free => ("$0".to_string(), "/month".to_string()),
                                    PlanPricing::Dynamic { monthly, annual } => {
                                        if is_monthly_signal.get() {
                                            (format!("${monthly}"), "/month".to_string())
                                        } else {
                                            (format!("${annual}"), "/year".to_string())
                                        }
                                    }
                                }
                            };
                            view! {
                                <Card class="flex flex-col gap-6 p-8">
                                    <CardHeader class="p-0">
                                        <div class="flex flex-col gap-2 text-center">
                                            <span class="text-xl font-bold leading-7 uppercase">
                                                <CardTitle class="text-xl">{plan.name}</CardTitle>
                                            </span>
                                            <span class="text-sm font-normal text-muted-foreground">
                                                {plan.description}
                                            </span>
                                        </div>
                                        <div class="flex flex-col gap-x-1 justify-center pt-8 text-center">
                                            <span class="text-4xl font-bold">{move || price_display().0}</span>
                                            <span class="text-sm leading-6 text-muted-foreground">
                                                {move || price_display().1}
                                            </span>
                                        </div>
                                    </CardHeader>
                                    <Button class="mt-8 w-full" href="#">
                                        Buy plan
                                    </Button>
                                    <CardContent class="p-0">
                                        <CardList class="mt-10 space-y-4 text-sm leading-6">
                                            {plan
                                                .feature_groups
                                                .iter()
                                                .map(|group| {
                                                    view! {
                                                        <li>
                                                            <CardList attr:role="list" class="space-y-4">
                                                                {group
                                                                    .iter()
                                                                    .map(|feature| {
                                                                        view! {
                                                                            <CardItem class="justify-between">
                                                                                <FeatureText>
                                                                                    <Check class="flex-none w-5 h-5" attr:aria-hidden="true" />
                                                                                    <span>{*feature}</span>
                                                                                </FeatureText>
                                                                                <InfoButton attr:data-state="closed">
                                                                                    <Info
                                                                                        class="ml-1 w-4 h-4 text-muted-foreground"
                                                                                        attr:aria-hidden="true"
                                                                                    />
                                                                                </InfoButton>
                                                                            </CardItem>
                                                                        }
                                                                    })
                                                                    .collect_view()}
                                                            </CardList>
                                                        </li>
                                                    }
                                                })
                                                .collect_view()}
                                        </CardList>
                                    </CardContent>
                                </Card>
                            }
                        })
                        .collect_view()}
                </div>

                // Desktop pricing table
                <div class="hidden mt-20 lg:block isolate">
                    <div class="relative -mx-8">
                        <div class="flex absolute inset-y-0 inset-x-4 -z-10">
                            <div class="flex px-4 w-1/4" style="margin-left:25%">
                                <div class="w-full border-x"></div>
                            </div>
                        </div>
                        <div class="flex absolute inset-y-0 inset-x-4 -z-10">
                            <div class="flex px-4 w-1/4" style="margin-left:50%">
                                <div class="w-full border-x"></div>
                            </div>
                        </div>
                        <div class="flex absolute inset-y-0 inset-x-4 -z-10">
                            <div class="flex px-4 w-1/4" style="margin-left:75%">
                                <div class="w-full border-x"></div>
                            </div>
                        </div>
                        <table class="w-full text-left border-separate table-fixed border-spacing-x-8">
                            <thead>
                                <tr>
                                    <td></td>
                                    {PRICING_PLANS
                                        .iter()
                                        .map(|plan| {
                                            view! {
                                                <th class="px-6 pt-6 xl:px-8 xl:pt-8">
                                                    <div class="flex flex-col gap-2 text-center">
                                                        <span class="text-xl font-bold leading-7 uppercase">
                                                            {plan.name}
                                                        </span>
                                                        <span class="text-sm font-normal text-muted-foreground">
                                                            {plan.description}
                                                        </span>
                                                    </div>
                                                </th>
                                            }
                                        })
                                        .collect_view()}
                                </tr>
                            </thead>
                            <tbody>
                                <tr>
                                    <th>
                                        <div class="flex flex-col gap-2">
                                            <p class="text-sm font-normal text-muted-foreground">Billings</p>
                                            <span class="flex gap-3 items-center text-base font-medium">
                                                Annual
                                                <Switch
                                                    checked=is_monthly_signal.get()
                                                    on:input=move |ev| {
                                                        is_monthly_signal.set(event_target_checked(&ev));
                                                    }
                                                />Monthly
                                            </span>
                                        </div>
                                    </th>
                                    {PRICING_PLANS
                                        .iter()
                                        .map(|plan| {
                                            let price_display = move || {
                                                match &plan.pricing {
                                                    PlanPricing::Free => ("$0".to_string(), "/month".to_string()),
                                                    PlanPricing::Dynamic { monthly, annual } => {
                                                        if is_monthly_signal.get() {
                                                            (format!("${monthly}"), "/month".to_string())
                                                        } else {
                                                            (format!("${annual}"), "/year".to_string())
                                                        }
                                                    }
                                                }
                                            };
                                            view! {
                                                <td class="px-6 pt-10 xl:px-8">
                                                    <div class="flex flex-col gap-x-1 justify-center text-center">
                                                        <span class="text-4xl font-bold">
                                                            {move || price_display().0}
                                                        </span>
                                                        <span class="text-sm leading-6 text-muted-foreground">
                                                            {move || price_display().1}
                                                        </span>
                                                    </div>
                                                    <Button class="mt-8 w-full" href="#">
                                                        Get Started
                                                    </Button>
                                                </td>
                                            }
                                        })
                                        .collect_view()}
                                </tr>
                                {FEATURE_SECTIONS
                                    .iter()
                                    .map(|section| {
                                        view! {
                                            <>
                                                <tr>
                                                    <SectionHeader class=section.class>{section.title}</SectionHeader>
                                                </tr>
                                                {section
                                                    .features
                                                    .iter()
                                                    .map(|feature| {
                                                        view! {
                                                            <tr>
                                                                <th class="flex justify-between items-center py-4 text-sm font-normal leading-6">
                                                                    {feature.name} <button attr:data-state="closed">
                                                                        <Info
                                                                            class="ml-1 w-4 h-4 text-muted-foreground hover:text-foreground"
                                                                            attr:aria-hidden="true"
                                                                        />
                                                                    </button>
                                                                </th>
                                                                {feature
                                                                    .availability
                                                                    .iter()
                                                                    .map(|available| {
                                                                        view! {
                                                                            <td class="py-4 px-6 xl:px-8">
                                                                                {if *available {
                                                                                    view! {
                                                                                        <Check class="mx-auto w-5 h-5" attr:aria-hidden="true" />
                                                                                    }
                                                                                        .into_any()
                                                                                } else {
                                                                                    view! {
                                                                                        <Minus
                                                                                            class="mx-auto w-5 h-5 text-muted-foreground"
                                                                                            attr:aria-hidden="true"
                                                                                        />
                                                                                    }
                                                                                        .into_any()
                                                                                }}
                                                                            </td>
                                                                        }
                                                                    })
                                                                    .collect_view()}
                                                            </tr>
                                                        }
                                                    })
                                                    .collect_view()}
                                            </>
                                        }
                                    })
                                    .collect_view()}
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        </section>
    }
}

/* ========================================================== */
/*                     ✨ CONSTANTS ✨                        */
/* ========================================================== */

enum PlanPricing {
    Free,
    Dynamic { monthly: u32, annual: u32 },
}

struct PricingPlan {
    name: &'static str,
    description: &'static str,
    pricing: PlanPricing,
    feature_groups: &'static [&'static [&'static str]],
}

struct Feature {
    name: &'static str,
    availability: &'static [bool],
}

struct FeatureSection {
    title: &'static str,
    class: &'static str,
    features: &'static [Feature],
}

const PRICING_PLANS: &[PricingPlan] = &[
    PricingPlan {
        name: "Free",
        description: "Quis suspendisse ut fermentum neque vivamus.",
        pricing: PlanPricing::Free,
        feature_groups: &[&["Live Collaboration", "Unlimited projects"], &["Basic reports"]],
    },
    PricingPlan {
        name: "Pro",
        description: "Quis eleifend a tincidunt pellentesque.",
        pricing: PlanPricing::Dynamic { monthly: 10, annual: 100 },
        feature_groups: &[
            &["Live Collaboration", "Unlimited projects", "Custom permissions"],
            &["Basic reports", "Advanced reports"],
        ],
    },
    PricingPlan {
        name: "Premium",
        description: "Orci volutpat ut sed sed neque, dui eget.",
        pricing: PlanPricing::Dynamic { monthly: 15, annual: 150 },
        feature_groups: &[
            &["Live Collaboration", "Unlimited projects", "Custom permissions", "Team members"],
            &["Basic reports", "Advanced reports", "Custom reports", "Export data"],
        ],
    },
];

const FEATURE_SECTIONS: &[FeatureSection] = &[
    FeatureSection {
        title: "Key Features",
        class: "",
        features: &[
            Feature { name: "Live Collaboration", availability: &[true, true, true] },
            Feature { name: "Unlimited projects", availability: &[true, true, true] },
            Feature { name: "Custom permissions", availability: &[false, true, true] },
            Feature { name: "Team members", availability: &[false, false, true] },
        ],
    },
    FeatureSection {
        title: "Reporting",
        class: "pt-16",
        features: &[
            Feature { name: "Basic reports", availability: &[true, true, true] },
            Feature { name: "Advanced reports", availability: &[false, true, true] },
            Feature { name: "Custom reports", availability: &[false, false, true] },
            Feature { name: "Export data", availability: &[false, false, true] },
        ],
    },
];
```
