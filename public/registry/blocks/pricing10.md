


```rust
use icons::{Check, Info};
use leptos::prelude::*;
use leptos_ui::clx;

use crate::components::ui::button::Button;
use crate::components::ui::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};

#[component]
pub fn Pricing10() -> impl IntoView {
    clx! {PlanHeader, div, "relative flex flex-col gap-3"}
    clx! {PriceContainer, div, "flex items-end gap-0.5"}
    clx! {PriceAmount, span, "text-4xl font-semibold"}
    clx! {PricePeriod, span, "text-base text-muted-foreground"}
    clx! {FeatureSection, div, "flex flex-col gap-4"}
    clx! {FeatureTitle, p, "text-sm font-medium"}
    clx! {FeatureList, ul, "flex flex-col gap-4"}
    clx! {FeatureItem, li, "flex items-center gap-3"}
    clx! {FeatureText, span, "flex-1 text-sm text-muted-foreground"}
    clx! {InfoButton, button, "cursor-pointer opacity-70 hover:opacity-100"}

    view! {
        <section class="section-padding-y bg-background">
            <div class="container mx-auto container-padding-x">
                <div class="flex flex-col gap-10 items-center md:gap-12">
                    <div class="flex flex-col items-center max-w-xl text-center section-title-gap-lg">
                        <div class="flex gap-1 justify-center items-center text-sm font-medium bg-transparent w-fit text-muted-foreground [&_svg]:size-3.5 [&_svg]:shrink-0">
                            Pricing section
                        </div>
                        <h2 id="pricing-section-title-2" class="heading-lg text-foreground">
                            "Benefit-focused headline that highlights choice"
                        </h2>
                        <p class="text-base text-muted-foreground">
                            "Add a concise value statement that addresses price sensitivity and showcases plan flexibility. Focus on transformation and results while keeping it under 2 lines. Align with your pricing table options."
                        </p>
                    </div>
                    <div class="grid grid-cols-1 gap-4 w-full md:grid-cols-2 lg:grid-cols-4">
                        {PRICING_TIERS
                            .iter()
                            .map(|tier| {
                                view! {
                                    <Card class=format!(
                                        "p-6{}",
                                        if tier.featured { " border-2 shadow-lg border-primary" } else { "" },
                                    )>
                                        <CardContent class="flex flex-col gap-8 p-0">
                                            <CardHeader class="flex flex-col gap-6 p-0">
                                                <PlanHeader>
                                                    <CardTitle class=format!(
                                                        "text-lg{}",
                                                        if tier.featured { " text-primary" } else { "" },
                                                    )>{tier.title}</CardTitle>
                                                    <CardDescription>{tier.description}</CardDescription>
                                                </PlanHeader>
                                                <PriceContainer>
                                                    <PriceAmount>${tier.price}</PriceAmount>
                                                    <PricePeriod>/month</PricePeriod>
                                                </PriceContainer>
                                                <Button class="w-full" href="#">
                                                    Purchase plan
                                                </Button>
                                            </CardHeader>
                                            <FeatureSection>
                                                <FeatureTitle>{tier.features_title}</FeatureTitle>
                                                <FeatureList>
                                                    {tier
                                                        .features
                                                        .iter()
                                                        .map(|feature| {
                                                            view! {
                                                                <FeatureItem>
                                                                    <Check
                                                                        class="w-5 h-5 text-primary"
                                                                        attr:aria-hidden="true"
                                                                    />
                                                                    <FeatureText>{*feature}</FeatureText>
                                                                    <InfoButton
                                                                        attr:data-state="closed"
                                                                        attr:data-slot="tooltip-trigger"
                                                                        attr:aria-label=format!(
                                                                            "More information about {}",
                                                                            *feature,
                                                                        )
                                                                    >
                                                                        <Info
                                                                            class="w-4 h-4 text-muted-foreground"
                                                                            attr:aria-hidden="true"
                                                                        />
                                                                    </InfoButton>
                                                                </FeatureItem>
                                                            }
                                                        })
                                                        .collect_view()}
                                                </FeatureList>
                                            </FeatureSection>
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
    description: &'static str,
    price: u32,
    features_title: &'static str,
    features: &'static [&'static str],
    featured: bool,
}

const PRICING_TIERS: &[PricingTier] = &[
    PricingTier {
        title: "Basic",
        description: "A short benefit statement that highlights the ideal user for this.",
        price: 99,
        features_title: "What's included:",
        features: &["Basic project management", "5GB storage space", "Email support"],
        featured: false,
    },
    PricingTier {
        title: "Standard",
        description: "A short benefit statement that highlights the ideal user for this.",
        price: 199,
        features_title: "Everything in Basic, plus:",
        features: &["Advanced project tools", "25GB storage space", "Priority email support", "Team collaboration"],
        featured: false,
    },
    PricingTier {
        title: "Pro",
        description: "A short benefit statement that highlights the ideal user for this.",
        price: 299,
        features_title: "Everything in Standard, plus:",
        features: &["Custom workflows", "100GB storage space", "Phone support", "Advanced analytics", "API access"],
        featured: true,
    },
    PricingTier {
        title: "Premium",
        description: "A short benefit statement that highlights the ideal user for this.",
        price: 499,
        features_title: "Everything in Pro, plus:",
        features: &[
            "Enterprise security",
            "Unlimited storage",
            "24/7 priority support",
            "Custom training",
            "Advanced integrations",
            "Dedicated success manager",
        ],
        featured: false,
    },
];
```