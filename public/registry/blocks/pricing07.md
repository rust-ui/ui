


```rust
use icons::Check;
use leptos::prelude::*;
use leptos_ui::clx;

use crate::components::ui::button::Button;

#[component]
pub fn Pricing07() -> impl IntoView {
    clx! {FeatureItem, li, "mt-4 flex items-center gap-2 text-sm font-medium"}

    view! {
        <section class="py-20 bg-muted">
            <div class="container">
                <div class="flex flex-col gap-4 items-center text-center">
                    <h2 class="mb-2 text-3xl font-medium lg:text-5xl">{PRICING_DATA.title}</h2>
                    <p class="max-w-lg text-muted-foreground">{PRICING_DATA.description}</p>
                </div>
                <div class="mt-10">
                    <div
                        data-slot="card"
                        class="flex flex-col gap-10 justify-between p-6 mx-auto w-full max-w-sm text-center rounded-lg border shadow-sm bg-card text-card-foreground"
                    >
                        <p class="text-2xl">{PRICING_DATA.plan.label}</p>
                        <div>
                            <div class="flex justify-center">
                                <span class="text-lg">$</span>
                                <span class="text-4xl font-medium lg:text-5xl">{PRICING_DATA.plan.price}</span>
                            </div>
                            <p class="mt-1 text-sm text-muted-foreground">{PRICING_DATA.plan.note}</p>
                        </div>
                        <Button class="w-full" href=PRICING_DATA.plan.button_href>
                            {PRICING_DATA.plan.button_text}
                        </Button>
                    </div>
                    <p class="mt-8 text-xl font-medium text-center lg:mt-10 lg:text-1xl">
                        {PRICING_DATA.features_title}
                    </p>
                    <ul class="grid mx-auto mt-4 md:grid-cols-2 md:pl-14 md:w-auto md:max-w-2xl lg:grid-cols-3 lg:max-w-4xl w-fit">
                        {PRICING_DATA
                            .features
                            .iter()
                            .map(|feature| {
                                view! {
                                    <FeatureItem>
                                        <Check class="size-5" attr:aria-hidden="true" />
                                        <span class="text-muted-foreground">{*feature}</span>
                                    </FeatureItem>
                                }
                            })
                            .collect_view()}
                    </ul>
                </div>
            </div>
        </section>
    }
}

/* ========================================================== */
/*                     ✨ CONSTANTS ✨                        */
/* ========================================================== */

struct PricingPlan {
    label: &'static str,
    price: &'static str,
    note: &'static str,
    button_text: &'static str,
    button_href: &'static str,
}

struct PricingData {
    title: &'static str,
    description: &'static str,
    plan: PricingPlan,
    features_title: &'static str,
    features: &'static [&'static str],
}

const PRICING_DATA: PricingData = PricingData {
    title: "Pricing",
    description: "Lorem ipsum dolor sit amet consectetur adipisicing elit. Error ipsum dolorem asperiores expedita!",
    plan: PricingPlan {
        label: "Starting at",
        price: "16/month",
        note: "With a 7-day free trial",
        button_text: "TRY FOR FREE",
        button_href: "#",
    },
    features_title: "What's included in the plan",
    features: &[
        "Automated backups",
        "24/7 support",
        "Unlimited projects",
        "Unlimited users",
        "Custom domain",
        "Custom branding",
        "Advanced analytics",
        "Custom permissions",
        "Advanced reports",
    ],
};
```