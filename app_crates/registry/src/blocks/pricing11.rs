use icons::{Check, Info};
use leptos::prelude::*;
use leptos_ui::clx;

use crate::ui::button::Button;
use crate::ui::card::{Card, CardDescription, CardItem, CardList, CardTitle};

/*
 * title: Three-Tier Pricing with Tabs and Tooltips
*/

#[component]
pub fn Pricing11() -> impl IntoView {
    clx! {FeatureText, span, "flex-1 text-sm text-muted-foreground"}
    clx! {TooltipButton, button, ""}

    view! {
        <section class="py-20 bg-background">
            <div class="container mx-auto container-padding-x">
                <div class="flex flex-col gap-10 items-center md:gap-12">
                    <div class="flex flex-col gap-3 items-center max-w-xl text-center lg:gap-4">
                        <div class="flex gap-1 justify-center items-center text-sm font-medium bg-transparent w-fit [&_svg]:size-3.5 [&_svg]:shrink-0 text-muted-foreground">
                            Pricing section
                        </div>
                        <h2 id="pricing-section-title-4" class="text-4xl font-semibold lg:text-5xl text-foreground">
                            Benefit-focused headline that highlights choice
                        </h2>
                        <p class="text-base text-muted-foreground">
                            Add a concise value statement that addresses price sensitivity and showcases plan flexibility. Focus on transformation and results while keeping it under 2 lines. Align with your pricing table options.
                        </p>
                    </div>
                    <div dir="ltr" data-orientation="horizontal" data-slot="tabs" class="flex flex-col gap-2 w-fit">
                        <div
                            role="tablist"
                            aria-orientation="horizontal"
                            data-slot="tabs-list"
                            class="inline-flex justify-center items-center h-10 rounded-lg text-muted-foreground w-fit p-[3px] bg-muted"
                            tabindex="0"
                            data-orientation="horizontal"
                            style="outline:none"
                        >
                            <button
                                type="button"
                                role="tab"
                                aria-selected="true"
                                data-state="active"
                                data-slot="tabs-trigger"
                                class="inline-flex flex-1 gap-1.5 justify-center items-center py-1.5 px-3 text-sm font-medium whitespace-nowrap rounded-md border border-transparent disabled:opacity-50 disabled:pointer-events-none text-foreground h-[calc(100%-1px)] transition-[color,box-shadow] [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4 data-[state=active]:bg-background data-[state=active]:shadow-sm dark:data-[state=active]:text-foreground dark:data-[state=active]:border-input dark:data-[state=active]:bg-input/30 dark:text-muted-foreground focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:outline-ring focus-visible:ring-[3px] focus-visible:outline-1"
                                tabindex="-1"
                                data-orientation="horizontal"
                            >
                                Monthly
                            </button>
                            <button
                                type="button"
                                role="tab"
                                aria-selected="false"
                                data-state="inactive"
                                data-slot="tabs-trigger"
                                class="inline-flex flex-1 gap-1.5 justify-center items-center py-1.5 px-3 text-sm font-medium whitespace-nowrap rounded-md border border-transparent disabled:opacity-50 disabled:pointer-events-none text-foreground h-[calc(100%-1px)] transition-[color,box-shadow] [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4 data-[state=active]:bg-background data-[state=active]:shadow-sm dark:data-[state=active]:text-foreground dark:data-[state=active]:border-input dark:data-[state=active]:bg-input/30 dark:text-muted-foreground focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:outline-ring focus-visible:ring-[3px] focus-visible:outline-1"
                                tabindex="-1"
                                data-orientation="horizontal"
                            >
                                Annually
                            </button>
                        </div>
                    </div>
                    <div class="flex flex-col gap-6 mx-auto w-full lg:flex-row lg:max-w-5xl">
                        {PRICING_TIERS
                            .iter()
                            .map(|tier| {
                                view! {
                                    <Card class=format!(
                                        "p-8 space-y-8{}",
                                        if tier.is_enterprise { " bg-foreground text-background" } else { "" },
                                    )>
                                        <div class="space-y-6">
                                            <div class="space-y-3">
                                                <CardTitle class="text-lg leading-7">{tier.title}</CardTitle>
                                                <CardDescription class=format!(
                                                    "leading-5{}",
                                                    if tier.is_enterprise {
                                                        " opacity-70 text-primary-foreground"
                                                    } else {
                                                        ""
                                                    },
                                                )>{tier.description}</CardDescription>
                                            </div>
                                            <div class="flex gap-0.5 items-end">
                                                <span class="text-4xl font-semibold leading-10">${tier.price}</span>
                                                <span class=format!(
                                                    "text-base leading-6{}",
                                                    if tier.is_enterprise {
                                                        " opacity-70"
                                                    } else {
                                                        " text-muted-foreground"
                                                    },
                                                )>/month</span>
                                            </div>
                                            <Button
                                                class=format!(
                                                    "w-full{}",
                                                    if tier.is_enterprise {
                                                        " bg-secondary text-secondary-foreground hover:bg-secondary/80"
                                                    } else {
                                                        ""
                                                    },
                                                )
                                                href="#"
                                            >
                                                Purchase plan
                                            </Button>
                                        </div>
                                        <div class="space-y-4">
                                            <p class="text-sm font-medium">{tier.features_title}</p>
                                            <CardList class="gap-3">
                                                {tier
                                                    .features
                                                    .iter()
                                                    .map(|feature| {
                                                        view! {
                                                            <CardItem>
                                                                <Check
                                                                    class=format!(
                                                                        "w-5 h-5{}",
                                                                        if tier.is_enterprise { "" } else { " text-primary" },
                                                                    )
                                                                    attr:aria-hidden="true"
                                                                />
                                                                {if tier.is_enterprise {
                                                                    view! {
                                                                        <span class="flex-1 text-sm opacity-70">{*feature}</span>
                                                                    }
                                                                        .into_any()
                                                                } else {
                                                                    view! { <FeatureText>{*feature}</FeatureText> }.into_any()
                                                                }}
                                                                <TooltipButton
                                                                    attr:data-state="closed"
                                                                    attr:data-slot="tooltip-trigger"
                                                                >
                                                                    <Info
                                                                        class=format!(
                                                                            "w-4 h-4{}",
                                                                            if tier.is_enterprise {
                                                                                " opacity-40"
                                                                            } else {
                                                                                " opacity-70 text-muted-foreground"
                                                                            },
                                                                        )
                                                                        attr:aria-hidden="true"
                                                                    />
                                                                </TooltipButton>
                                                            </CardItem>
                                                        }
                                                    })
                                                    .collect_view()}
                                            </CardList>
                                        </div>
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
    is_enterprise: bool,
}

const PRICING_TIERS: &[PricingTier] = &[
    PricingTier {
        title: "Basic",
        description: "Perfect for individuals and small projects",
        price: 29,
        features_title: "What's included:",
        features: &["Up to 5 team members", "10GB storage space", "Basic analytics"],
        is_enterprise: false,
    },
    PricingTier {
        title: "Standard",
        description: "Ideal for growing teams and businesses",
        price: 49,
        features_title: "Everything in Basic, plus:",
        features: &["Up to 20 team members", "50GB storage space", "Advanced analytics", "Priority support"],
        is_enterprise: false,
    },
    PricingTier {
        title: "Enterprise",
        description: "For large enterprises and advanced needs",
        price: 99,
        features_title: "Everything in Standard, plus:",
        features: &[
            "Unlimited team members",
            "250GB storage space",
            "Custom analytics",
            "24/7 premium support",
            "White-labeling",
        ],
        is_enterprise: true,
    },
];
