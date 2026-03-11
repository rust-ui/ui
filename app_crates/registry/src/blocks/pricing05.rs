use icons::CircleCheck;
use leptos::prelude::*;

use crate::ui::button::Button;
use crate::ui::card::{Card, CardContent, CardFooter, CardHeader, CardItem, CardList, CardTitle};
use crate::ui::separator::Separator;
use crate::ui::switch::Switch;

/*
 * title: Pricing with Monthly/Yearly Toggle
*/

#[component]
pub fn Pricing05() -> impl IntoView {
    let is_yearly_signal = RwSignal::new(false);

    view! {
        <section class="py-20">
            <div class="container">
                <div class="flex flex-col gap-6 items-center mx-auto max-w-5xl text-center">
                    <h2 class="text-4xl font-semibold lg:text-6xl text-pretty">Pricing</h2>
                    <p class="lg:text-xl text-muted-foreground">Check out our affordable pricing plans</p>
                    <div class="flex gap-3 items-center text-lg">
                        <span>Monthly</span>
                        <Switch id="pricing-toggle" prop:checked=is_yearly_signal />
                        <span>Yearly</span>
                    </div>
                    <div class="flex flex-col gap-6 items-stretch md:flex-row">
                        {PRICING_PLANS
                            .iter()
                            .map(|plan| {
                                let price = Signal::derive(move || {
                                    if is_yearly_signal.get() { plan.yearly_price } else { plan.monthly_price }
                                });
                                let period = Signal::derive(move || {
                                    if is_yearly_signal.get() { "/yr" } else { "/mo" }
                                });
                                view! {
                                    <Card class="flex flex-col justify-between w-80 text-left">
                                        <CardHeader>
                                            <CardTitle>
                                                <p>{plan.title}</p>
                                            </CardTitle>
                                            <p class="text-sm text-muted-foreground">{plan.description}</p>
                                            <div class="flex items-end">
                                                <span class="text-4xl font-semibold">${move || price.get()}</span>
                                                <span class="text-2xl font-semibold text-muted-foreground">
                                                    {move || period.get()}
                                                </span>
                                            </div>
                                        </CardHeader>
                                        <CardContent>
                                            <Separator class="mb-6" />
                                            {plan
                                                .features_prefix
                                                .map(|prefix| view! { <p class="mb-3 font-semibold">{prefix}</p> })}
                                            <CardList class="space-y-4">
                                                {plan
                                                    .features
                                                    .iter()
                                                    .map(|feature| {
                                                        view! {
                                                            <CardItem class="gap-2 text-sm">
                                                                <CircleCheck class="size-4" attr:aria-hidden="true" />
                                                                <span>{*feature}</span>
                                                            </CardItem>
                                                        }
                                                    })
                                                    .collect_view()}
                                            </CardList>
                                        </CardContent>
                                        <CardFooter class="mt-auto">
                                            <Button class="w-full" href="#">
                                                Purchase
                                            </Button>
                                        </CardFooter>
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

struct PricingPlan {
    title: &'static str,
    description: &'static str,
    monthly_price: &'static str,
    yearly_price: &'static str,
    features_prefix: Option<&'static str>,
    features: &'static [&'static str],
}

const PRICING_PLANS: &[PricingPlan] = &[
    PricingPlan {
        title: "Plus",
        description: "For personal use",
        monthly_price: "19",
        yearly_price: "190",
        features_prefix: None,
        features: &["Up to 5 team members", "Basic components library", "Community support", "1GB storage space"],
    },
    PricingPlan {
        title: "Pro",
        description: "For professionals",
        monthly_price: "49",
        yearly_price: "490",
        features_prefix: Some("Everything in Plus, and:"),
        features: &["Unlimited team members", "Advanced components", "Priority support", "Unlimited storage"],
    },
];
