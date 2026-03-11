


```rust
use icons::{BadgeCheck, Briefcase, Building, Rocket};
use leptos::prelude::*;
use leptos_ui::clx;

use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::card::CardTitle;
use crate::components::ui::table::{Table, TableBody, TableCell, TableHead, TableHeader, TableRow};

#[component]
pub fn Pricing04() -> impl IntoView {
    let is_yearly_signal = RwSignal::new(false);

    let monthly_variant =
        Signal::derive(move || if !is_yearly_signal.get() { ButtonVariant::Default } else { ButtonVariant::Outline });

    let yearly_variant =
        Signal::derive(move || if is_yearly_signal.get() { ButtonVariant::Default } else { ButtonVariant::Outline });

    clx! {PlanIcon, div, "text-muted-foreground flex items-center gap-2.5"}
    clx! {PriceDisplay, div, "flex items-baseline font-medium"}
    clx! {PriceAmount, span, "text-[3.5rem] leading-[120%] tracking-[-3.92px]"}
    clx! {PriceUnit, span, "text-muted-foreground-subtle text-2xl tracking-[-0.96px]"}
    clx! {CategoryTitle, h3, "text-lg tracking-[-0.36px] mb-2"}
    clx! {FeatureValue, span, "text-muted-foreground-subtle font-semibold"}

    let render_icon = move |icon_type: &PlanIconType| -> AnyView {
        match icon_type {
            PlanIconType::Rocket => view! { <Rocket class="size-4" attr:aria-hidden="true" /> }.into_any(),
            PlanIconType::Briefcase => view! { <Briefcase class="size-4" attr:aria-hidden="true" /> }.into_any(),
            PlanIconType::Building => view! { <Building class="size-4" attr:aria-hidden="true" /> }.into_any(),
        }
    };

    view! {
        <section class="py-20">
            <div class="container">
                <section class="py-14 md:py-20 lg:py-24">
                    // Toggle between Monthly and Yearly
                    <div class="flex justify-center">
                        <div class="inline-flex rounded-md border gap-[2px] p-[2px]">
                            <Button
                                class="transition-colors"
                                variant=monthly_variant
                                on:click=move |_| is_yearly_signal.set(false)
                            >
                                Monthly
                            </Button>
                            <Button
                                class="transition-colors"
                                variant=yearly_variant
                                on:click=move |_| is_yearly_signal.set(true)
                            >
                                Yearly
                            </Button>
                        </div>
                    </div>

                    // Pricing table
                    <div class="mt-12">
                        <Table>
                            <TableHeader>
                                <tr class="border-b">
                                    <TableHead class="w-1/4">Features</TableHead>
                                    {PRICING_PLANS
                                        .iter()
                                        .map(|plan| {
                                            view! {
                                                <TableHead class="text-center">
                                                    <div class="pb-6 space-y-2">
                                                        <PlanIcon class="justify-center">
                                                            {render_icon(&plan.icon)}
                                                            <CardTitle class="text-xl tracking-[-0.8px]">
                                                                {plan.name}
                                                            </CardTitle>
                                                        </PlanIcon>
                                                        <PriceDisplay class="justify-center">
                                                            <PriceAmount>
                                                                $
                                                                {move || {
                                                                    if is_yearly_signal.get() {
                                                                        plan.yearly_price
                                                                    } else {
                                                                        plan.monthly_price
                                                                    }
                                                                }}
                                                            </PriceAmount>
                                                            <PriceUnit>
                                                                {move || if is_yearly_signal.get() { "/yr" } else { "/mo" }}
                                                            </PriceUnit>
                                                        </PriceDisplay>
                                                        <p class="text-sm text-muted-foreground">
                                                            {move || {
                                                                if is_yearly_signal.get() {
                                                                    format!("or ${} monthly", plan.monthly_price)
                                                                } else {
                                                                    format!("or ${} yearly", plan.yearly_price)
                                                                }
                                                            }}
                                                        </p>
                                                        <Button class="mt-2 w-full" href="#">
                                                            Get started
                                                        </Button>
                                                    </div>
                                                </TableHead>
                                            }
                                        })
                                        .collect_view()}
                                </tr>
                            </TableHeader>
                            <TableBody>
                                {FEATURE_CATEGORIES
                                    .iter()
                                    .map(|category| {
                                        view! {
                                            <>
                                                <TableRow>
                                                    <TableCell class="font-semibold bg-muted/50" attr:colspan="4">
                                                        <CategoryTitle>{category.name}</CategoryTitle>
                                                    </TableCell>
                                                </TableRow>
                                                {category
                                                    .features
                                                    .iter()
                                                    .map(|feature| {
                                                        view! {
                                                            <TableRow>
                                                                <TableCell class="font-medium">{feature.name}</TableCell>
                                                                {feature
                                                                    .values
                                                                    .iter()
                                                                    .map(|value| {
                                                                        view! {
                                                                            <TableCell class="text-center">
                                                                                {match value {
                                                                                    FeatureValue::Text(text) => {
                                                                                        view! { <FeatureValue>{*text}</FeatureValue> }.into_any()
                                                                                    }
                                                                                    FeatureValue::Check => {
                                                                                        view! {
                                                                                            <BadgeCheck
                                                                                                class="mx-auto text-muted-foreground size-5"
                                                                                                attr:aria-hidden="true"
                                                                                            />
                                                                                        }
                                                                                            .into_any()
                                                                                    }
                                                                                    FeatureValue::None => view! { {""} }.into_any(),
                                                                                }}
                                                                            </TableCell>
                                                                        }
                                                                    })
                                                                    .collect_view()}
                                                            </TableRow>
                                                        }
                                                    })
                                                    .collect_view()}
                                            </>
                                        }
                                    })
                                    .collect_view()}
                            </TableBody>
                        </Table>
                    </div>
                </section>
            </div>
        </section>
    }
}

/* ========================================================== */
/*                     ✨ CONSTANTS ✨                        */
/* ========================================================== */

enum PlanIconType {
    Rocket,
    Briefcase,
    Building,
}

struct PricingPlan {
    name: &'static str,
    icon: PlanIconType,
    monthly_price: &'static str,
    yearly_price: &'static str,
}

enum FeatureValue {
    Text(&'static str),
    Check,
    None,
}

struct Feature {
    name: &'static str,
    values: &'static [FeatureValue],
}

struct FeatureCategory {
    name: &'static str,
    features: &'static [Feature],
}

const PRICING_PLANS: &[PricingPlan] = &[
    PricingPlan { name: "Basic plan", icon: PlanIconType::Rocket, monthly_price: "19", yearly_price: "199" },
    PricingPlan { name: "Business plan", icon: PlanIconType::Briefcase, monthly_price: "29", yearly_price: "299" },
    PricingPlan { name: "Enterprise plan", icon: PlanIconType::Building, monthly_price: "49", yearly_price: "499" },
];

const FEATURE_CATEGORIES: &[FeatureCategory] = &[
    FeatureCategory {
        name: "Core Tools",
        features: &[
            Feature {
                name: "Task Management",
                values: &[FeatureValue::Text("10"), FeatureValue::Text("25"), FeatureValue::Text("Unlimited")],
            },
            Feature { name: "Calendar Sync", values: &[FeatureValue::Check, FeatureValue::Check, FeatureValue::Check] },
            Feature { name: "Reminders", values: &[FeatureValue::Check, FeatureValue::Check, FeatureValue::Check] },
            Feature { name: "Collaboration", values: &[FeatureValue::None, FeatureValue::Check, FeatureValue::Check] },
            Feature { name: "Notifications", values: &[FeatureValue::None, FeatureValue::None, FeatureValue::Check] },
        ],
    },
    FeatureCategory {
        name: "Productivity Insights",
        features: &[
            Feature {
                name: "Analytics",
                values: &[
                    FeatureValue::Text("10 25 Unlimited"),
                    FeatureValue::Text("10 25 Unlimited"),
                    FeatureValue::Text("10 25 Unlimited"),
                ],
            },
            Feature { name: "Reports", values: &[FeatureValue::Check, FeatureValue::Check, FeatureValue::Check] },
            Feature { name: "Time Tracking", values: &[FeatureValue::Check, FeatureValue::Check, FeatureValue::Check] },
            Feature { name: "Goal Tracking", values: &[FeatureValue::None, FeatureValue::Check, FeatureValue::Check] },
            Feature { name: "Trends", values: &[FeatureValue::None, FeatureValue::None, FeatureValue::Check] },
        ],
    },
    FeatureCategory {
        name: "Workflow Automation",
        features: &[
            Feature {
                name: "Task Automation",
                values: &[FeatureValue::Text("10"), FeatureValue::Text("25"), FeatureValue::Text("Unlimited")],
            },
            Feature {
                name: "Recurring Tasks",
                values: &[FeatureValue::Check, FeatureValue::Check, FeatureValue::Check],
            },
            Feature { name: "Integrations", values: &[FeatureValue::Check, FeatureValue::Check, FeatureValue::Check] },
            Feature { name: "API Access", values: &[FeatureValue::None, FeatureValue::Check, FeatureValue::Check] },
            Feature {
                name: "Workflow Templates",
                values: &[FeatureValue::None, FeatureValue::None, FeatureValue::Check],
            },
        ],
    },
];
```