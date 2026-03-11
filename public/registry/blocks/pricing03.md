


```rust
use icons::Check;
use leptos::prelude::*;
use leptos_ui::clx;

use crate::components::ui::button::Button;
use crate::components::ui::card::{Card, CardContent, CardDescription, CardHeader, CardItem, CardList, CardTitle};

#[component]
pub fn Pricing03() -> impl IntoView {
    clx! {PriceContainer, div, "flex items-start justify-center"}
    clx! {PriceSign, span, "mt-2 text-lg font-semibold"}
    clx! {PriceAmount, span, "text-6xl font-semibold"}
    clx! {FeatureSection, div, "grow border-t p-8 text-left"}

    view! {
        <section class="py-20">
            <div class="container">
                <div class="mx-auto mb-20 max-w-5xl text-center">
                    <h2 class="mb-3 text-4xl font-bold lg:text-6xl text-pretty">Pricing</h2>
                    <p class="lg:text-xl text-muted-foreground">
                        Check out our affordable pricing plans below and choose the one that suits you best. If you need a custom plan, please contact us.
                    </p>
                </div>
                <div class="grid mx-auto max-w-xl rounded-md border lg:grid-cols-4 lg:max-w-none lg:divide-x">
                    {PRICING_TIERS
                        .iter()
                        .map(|tier| {
                            view! {
                                <Card class=format!(
                                    "h-full border-none shadow-none{}",
                                    if tier.highlight { " bg-muted" } else { "" },
                                )>
                                    <CardContent class="flex flex-col h-full">
                                        <CardHeader class="flex-none h-[360px]">
                                            <div class="flex flex-col justify-between h-full">
                                                <div class="px-8 pt-8 pb-3">
                                                    <CardTitle class="text-3xl">{tier.title}</CardTitle>
                                                </div>
                                                <div class="px-8 pb-6">
                                                    <CardDescription class="line-clamp-2 text-balance">
                                                        {tier.description}
                                                    </CardDescription>
                                                </div>
                                                <div class="flex flex-col justify-start px-8 pb-6 grow">
                                                    {tier
                                                        .price
                                                        .map(|price| {
                                                            view! {
                                                                <div class="flex justify-center items-start mb-4">
                                                                    <div class="text-center">
                                                                        <div class="flex justify-center items-start">
                                                                            <PriceSign>$</PriceSign>
                                                                            <PriceAmount>{price}</PriceAmount>
                                                                        </div>
                                                                        <CardDescription class="mt-2 text-sm">
                                                                            {tier.price_note}
                                                                        </CardDescription>
                                                                    </div>
                                                                </div>
                                                            }
                                                        })}
                                                </div>
                                                <div class="px-8 pb-8 mt-auto">
                                                    <Button class="w-full" href=tier.button_href>
                                                        {tier.button_text}
                                                    </Button>
                                                </div>
                                            </div>
                                        </CardHeader>
                                        <FeatureSection>
                                            <CardTitle class="mb-4 text-lg font-semibold">
                                                {tier.features_title}
                                            </CardTitle>
                                            <CardList class="space-y-4">
                                                {tier
                                                    .features
                                                    .iter()
                                                    .map(|item| {
                                                        view! {
                                                            <CardItem class="gap-3">
                                                                <Check
                                                                    class="text-primary size-5"
                                                                    attr:aria-hidden="true"
                                                                />
                                                                <span>{*item}</span>
                                                            </CardItem>
                                                        }
                                                    })
                                                    .collect_view()}
                                            </CardList>
                                        </FeatureSection>
                                    </CardContent>
                                </Card>
                            }
                        })
                        .collect_view()}
                </div>
            </div>
        </section>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

struct PricingTier {
    title: &'static str,
    description: &'static str,
    price: Option<u32>,
    price_note: &'static str,
    button_text: &'static str,
    button_href: &'static str,
    features_title: &'static str,
    features: &'static [&'static str],
    highlight: bool,
}

const PRICING_TIERS: &[PricingTier] = &[
    PricingTier {
        title: "Free",
        description: "For personal use only with limited features and support",
        price: Some(0),
        price_note: "Includes 1 user.",
        button_text: "Get Started",
        button_href: "#",
        features_title: "Features",
        features: &[
            "Live Collaboration",
            "1 GB Storage",
            "2 Projects",
            "Basic Support",
            "Limited Customization",
            "Limited Integration",
            "Limited API Access",
        ],
        highlight: false,
    },
    PricingTier {
        title: "Pro",
        description: "For small businesses with all the features and support",
        price: Some(29),
        price_note: "Per user, per month.",
        button_text: "Purchase",
        button_href: "#",
        features_title: "Everything in Free, and:",
        features: &[
            "2 Team Members",
            "10 GB Storage",
            "10 Projects",
            "Priority Support",
            "Full Customization",
            "Full Integration",
            "Full API Access",
        ],
        highlight: true,
    },
    PricingTier {
        title: "Premium",
        description: "For teams and organizations with advanced features and support",
        price: Some(59),
        price_note: "Per user, per month.",
        button_text: "Purchase",
        button_href: "#",
        features_title: "Everything in Pro, and:",
        features: &[
            "5 Team Members",
            "50 GB Storage",
            "50 Projects",
            "Dedicated Support",
            "Advanced Customization",
            "Analytics",
            "Reports",
        ],
        highlight: false,
    },
    PricingTier {
        title: "Enterprise",
        description: "For large companies with custom features and support and a dedicated account manager",
        price: None,
        price_note: "",
        button_text: "Contact sales",
        button_href: "#",
        features_title: "Everything in Premium, and:",
        features: &[
            "10+ Team Members",
            "100+ GB Storage",
            "100+ Projects",
            "Dedicated Account Manager",
            "Custom Features",
            "Custom Support",
            "Custom Integration",
        ],
        highlight: false,
    },
];
```