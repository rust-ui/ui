use icons::{Check, Info};
use leptos::prelude::*;
use leptos_ui::clx;

use crate::ui::button::{Button, ButtonVariant};
use crate::ui::card::{Card, CardContent, CardDescription, CardItem, CardList, CardTitle};
use crate::ui::tooltip::{Tooltip, TooltipContent, TooltipProvider};

/*
 * title: Two-Tier Comparison Pricing with Tooltips
*/

#[component]
pub fn Pricing09() -> impl IntoView {
    clx! {PlanHeader, div, "flex flex-col gap-6"}
    clx! {TitleSection, div, "relative flex flex-col gap-3"}
    clx! {PriceContainer, div, "flex items-end gap-0.5"}
    clx! {PriceAmount, span, "text-4xl font-semibold"}
    clx! {PricePeriod, span, "text-muted-foreground text-base"}
    clx! {FeaturesSection, div, "flex flex-col gap-4"}
    clx! {FeaturesTitle, p, "text-sm font-medium"}
    clx! {FeatureText, span, "text-muted-foreground flex-1 text-sm"}

    view! {
        <TooltipProvider />

        <section class="py-20">
            <div class="container px-6 mx-auto lg:px-8">
                <div class="flex flex-col gap-10 items-center md:gap-12">
                    <div class="flex flex-col gap-6 items-center max-w-2xl text-center">
                        <div class="flex gap-1 justify-center items-center text-sm font-medium w-fit [&_svg]:size-3.5 [&_svg]:shrink-0 text-muted-foreground">
                            "Pricing section"
                        </div>
                        <h2 id="pricing-section-title-3" class="text-4xl font-bold tracking-tight sm:text-5xl">
                            "Benefit-focused headline that highlights choice"
                        </h2>
                        <p class="text-base text-muted-foreground">
                            "Add a concise value statement that addresses price sensitivity and showcases plan flexibility. Focus on transformation and results while keeping it under 2 lines. Align with your pricing table options."
                        </p>
                    </div>
                    <div class="flex flex-col gap-4 items-center w-full md:flex-row md:gap-0 md:max-w-3xl">
                        {PRICING_PLANS
                            .iter()
                            .enumerate()
                            .map(|(index, plan)| {
                                let card_class = if index == 0 {
                                    "p-6 sm:p-12 md:rounded-tr-none md:rounded-br-none md:rounded-tl-xl md:rounded-bl-xl md:border-r-0"
                                } else {
                                    "p-6 sm:p-12 md:rounded-xl md:shadow-xl md:border-r-1"
                                };
                                let title_class = if plan.is_featured { "text-lg text-primary" } else { "text-lg" };
                                let button_variant = if index == 0 {
                                    ButtonVariant::Secondary
                                } else {
                                    ButtonVariant::Default
                                };

                                view! {
                                    <Card class=card_class>
                                        <CardContent class="flex flex-col gap-8 p-0">
                                            <PlanHeader>
                                                <TitleSection>
                                                    <CardTitle class=title_class>{plan.title}</CardTitle>
                                                    <CardDescription>{plan.description}</CardDescription>
                                                </TitleSection>
                                                <PriceContainer>
                                                    <PriceAmount>"$" {plan.price}</PriceAmount>
                                                    <PricePeriod>"/month"</PricePeriod>
                                                </PriceContainer>
                                                <Button class="w-full" variant=button_variant href=plan.button_href>
                                                    {plan.button_text}
                                                </Button>
                                            </PlanHeader>
                                            <FeaturesSection>
                                                <FeaturesTitle>{plan.features_title}</FeaturesTitle>
                                                <CardList>
                                                    {plan
                                                        .features
                                                        .iter()
                                                        .map(|feature| {
                                                            view! {
                                                                <CardItem class="gap-3">
                                                                    <Check
                                                                        class="w-5 h-5 text-primary"
                                                                        attr:aria-hidden="true"
                                                                    />
                                                                    <FeatureText>{feature.text}</FeatureText>
                                                                    <Tooltip>
                                                                        <Info
                                                                            class="w-4 h-4 opacity-70 cursor-pointer hover:opacity-100 text-muted-foreground"
                                                                            attr:aria-hidden="true"
                                                                        />
                                                                        <TooltipContent>{feature.tooltip}</TooltipContent>
                                                                    </Tooltip>
                                                                </CardItem>
                                                            }
                                                        })
                                                        .collect_view()}
                                                </CardList>
                                            </FeaturesSection>
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

struct Feature {
    text: &'static str,
    tooltip: &'static str,
}

struct PricingPlan {
    title: &'static str,
    description: &'static str,
    price: u32,
    button_text: &'static str,
    button_href: &'static str,
    features_title: &'static str,
    features: &'static [Feature],
    is_featured: bool,
}

const PRICING_PLANS: &[PricingPlan] = &[
    PricingPlan {
        title: "Basic",
        description: "A short benefit statement that highlights the ideal user for this.",
        price: 99,
        button_text: "Purchase plan",
        button_href: "#",
        features_title: "What's included:",
        features: &[
            Feature { text: "Basic project management", tooltip: "Manage up to 10 projects with essential tools" },
            Feature { text: "5GB storage space", tooltip: "Store your files and documents securely" },
            Feature { text: "Email support", tooltip: "Get help via email during business hours" },
            Feature { text: "Basic reporting", tooltip: "Generate standard reports for your projects" },
        ],
        is_featured: false,
    },
    PricingPlan {
        title: "Pro",
        description: "A short benefit statement that highlights the ideal user for this.",
        price: 299,
        button_text: "Purchase plan",
        button_href: "#",
        features_title: "Everything in Basic, plus:",
        features: &[
            Feature { text: "Custom workflows", tooltip: "Create and automate custom workflows for your team" },
            Feature { text: "100GB storage space", tooltip: "20x more storage for all your project files" },
            Feature { text: "Phone support", tooltip: "Priority phone support available 24/7" },
            Feature { text: "Advanced reporting", tooltip: "Detailed analytics and custom report generation" },
            Feature { text: "Advanced analytics", tooltip: "Track performance metrics and team productivity" },
            Feature { text: "API access", tooltip: "Integrate with your existing tools via REST API" },
        ],
        is_featured: true,
    },
];
