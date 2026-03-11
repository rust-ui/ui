


```rust
use icons::{Clock, Eye, FileText, HardDrive, Key, Lock, Mail, Palette, ShieldCheck};
use leptos::prelude::*;
use leptos_ui::clx;

use crate::components::ui::avatar::{Avatar, AvatarImage};
use crate::components::ui::badge::Badge;
use crate::components::ui::button::Button;
use crate::components::ui::card::{Card, CardContent, CardHeader, CardItem, CardList};

#[component]
pub fn Pricing02() -> impl IntoView {
    clx! {FeatureCategory, div, "mb-6"}
    clx! {CategoryTitle, h4, "text-muted-foreground mb-3 text-sm font-medium"}

    view! {
        <section class="py-20">
            <div class="container">
                <div class="mx-auto max-w-3xl text-center">
                    <h2 class="mb-4 text-4xl font-semibold lg:text-5xl">Choose the right plan for you</h2>
                </div>

                // Trust indicators
                <div class="flex gap-4 justify-center items-center mx-auto mb-12 max-w-3xl">
                    <div class="flex relative items-center mr-2">
                        <Avatar class="border size-8 border-primary">
                            <AvatarImage attr:src="/images/block/avatar-1.webp" />
                        </Avatar>
                        <Avatar class="absolute left-6 border size-8 border-primary">
                            <AvatarImage attr:src="/images/block/avatar-2.webp" />
                        </Avatar>
                        <Avatar class="absolute left-12 border size-8 border-primary">
                            <AvatarImage attr:src="/images/block/avatar-3.webp" />
                        </Avatar>
                    </div>
                    <div class="flex gap-4 items-center ml-8">
                        <span class="text-sm font-medium">50K+ developers trust us</span>
                        <div class="w-px h-8 bg-border"></div>
                        <span class="text-sm font-medium">Cancel any time, without any hassle</span>
                    </div>
                </div>

                // Pricing cards grid
                <div class="grid gap-2 mt-10 md:grid-cols-2 lg:grid-cols-4">
                    {PRICING_TIERS
                        .iter()
                        .map(|tier| {
                            let render_icon = move |icon_type: &IconType| -> AnyView {
                                match icon_type {
                                    IconType::ShieldCheck => {
                                        view! { <ShieldCheck attr:aria-hidden="true" /> }.into_any()
                                    }
                                    IconType::Lock => view! { <Lock attr:aria-hidden="true" /> }.into_any(),
                                    IconType::Key => view! { <Key attr:aria-hidden="true" /> }.into_any(),
                                    IconType::Palette => view! { <Palette attr:aria-hidden="true" /> }.into_any(),
                                    IconType::Eye => view! { <Eye attr:aria-hidden="true" /> }.into_any(),
                                    IconType::HardDrive => view! { <HardDrive attr:aria-hidden="true" /> }.into_any(),
                                    IconType::FileText => view! { <FileText attr:aria-hidden="true" /> }.into_any(),
                                    IconType::Mail => view! { <Mail attr:aria-hidden="true" /> }.into_any(),
                                    IconType::Clock => view! { <Clock attr:aria-hidden="true" /> }.into_any(),
                                }
                            };

                            view! {
                                <Card class="rounded-2xl">
                                    <CardHeader class=if tier.show_badge {
                                        "!flex flex-row justify-between items-center"
                                    } else {
                                        ""
                                    }>
                                        <h3 class="text-xl">{tier.title}</h3>
                                        {tier.show_badge.then(|| view! { <Badge>Popular</Badge> })}
                                    </CardHeader>
                                    <CardContent>
                                        <div class="flex gap-2 items-center mb-2">
                                            {if let Some(price) = tier.price {
                                                view! {
                                                    <>
                                                        <span class="text-5xl font-semibold">${price}</span>
                                                        <span class="text-muted-foreground">/mo</span>
                                                    </>
                                                }
                                                    .into_any()
                                            } else {
                                                view! { <span class="text-5xl font-semibold">Custom</span> }.into_any()
                                            }}
                                        </div>
                                        <span class="border-b border-dashed cursor-pointer border-muted-foreground">
                                            {tier.credits}
                                        </span>
                                        <Button class="mt-6 w-full rounded-full" href=tier.button_href>
                                            {tier.button_text}
                                        </Button>
                                        <div class="mt-6">
                                            {tier
                                                .categories
                                                .iter()
                                                .map(|category| {
                                                    view! {
                                                        <FeatureCategory>
                                                            <CategoryTitle>{category.title}</CategoryTitle>
                                                            <CardList class="gap-3">
                                                                {category
                                                                    .features
                                                                    .iter()
                                                                    .map(|feature| {
                                                                        view! {
                                                                            <CardItem class="gap-2 [&_svg]:mt-1 [&_svg]:text-primary">
                                                                                {render_icon(&feature.icon)} <span>{feature.text}</span>
                                                                            </CardItem>
                                                                        }
                                                                    })
                                                                    .collect_view()}
                                                            </CardList>
                                                        </FeatureCategory>
                                                    }
                                                })
                                                .collect_view()}
                                        </div>
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
/*                     ✨ CONSTANTS ✨                        */
/* ========================================================== */

enum IconType {
    ShieldCheck,
    Lock,
    Key,
    Palette,
    Eye,
    HardDrive,
    FileText,
    Mail,
    Clock,
}

struct Feature {
    icon: IconType,
    text: &'static str,
}

struct Category {
    title: &'static str,
    features: &'static [Feature],
}

struct PricingTier {
    title: &'static str,
    price: Option<u32>,
    credits: &'static str,
    button_text: &'static str,
    button_href: &'static str,
    show_badge: bool,
    categories: &'static [Category],
}

const PRICING_TIERS: &[PricingTier] = &[
    PricingTier {
        title: "Basic Plan",
        price: Some(79),
        credits: "10 credit total",
        button_text: "Get Basic Plan",
        button_href: "#",
        show_badge: false,
        categories: &[
            Category {
                title: "Security",
                features: &[
                    Feature { icon: IconType::ShieldCheck, text: "Standard security" },
                    Feature { icon: IconType::Lock, text: "Basic encryption" },
                    Feature { icon: IconType::Key, text: "Two-factor authentication" },
                ],
            },
            Category {
                title: "Style",
                features: &[
                    Feature { icon: IconType::Palette, text: "Basic branding" },
                    Feature { icon: IconType::Eye, text: "Limited style customization" },
                ],
            },
            Category {
                title: "Storage",
                features: &[
                    Feature { icon: IconType::HardDrive, text: "5GB storage" },
                    Feature { icon: IconType::FileText, text: "File versioning" },
                ],
            },
            Category {
                title: "Support",
                features: &[
                    Feature { icon: IconType::Mail, text: "Support by email only" },
                    Feature { icon: IconType::Clock, text: "Basic time tracking" },
                ],
            },
        ],
    },
    PricingTier {
        title: "Pro Plan",
        price: Some(299),
        credits: "50 credit total",
        button_text: "Get Pro Plan",
        button_href: "#",
        show_badge: false,
        categories: &[
            Category {
                title: "Security",
                features: &[
                    Feature { icon: IconType::ShieldCheck, text: "Enhanced security" },
                    Feature { icon: IconType::Lock, text: "Advanced encryption" },
                    Feature { icon: IconType::Key, text: "SSO authentication" },
                ],
            },
            Category {
                title: "Style",
                features: &[
                    Feature { icon: IconType::Palette, text: "Full branding control" },
                    Feature { icon: IconType::Eye, text: "Complete style customization" },
                ],
            },
            Category {
                title: "Storage",
                features: &[
                    Feature { icon: IconType::HardDrive, text: "100GB storage" },
                    Feature { icon: IconType::FileText, text: "Advanced file management" },
                ],
            },
            Category {
                title: "Support",
                features: &[
                    Feature { icon: IconType::Mail, text: "Priority support" },
                    Feature { icon: IconType::Clock, text: "Advanced time tracking" },
                ],
            },
        ],
    },
    PricingTier {
        title: "Business Plan",
        price: Some(599),
        credits: "100 credit total",
        button_text: "Get Business Plan",
        button_href: "#",
        show_badge: false,
        categories: &[
            Category {
                title: "Security",
                features: &[
                    Feature { icon: IconType::ShieldCheck, text: "Premium security" },
                    Feature { icon: IconType::Lock, text: "Enterprise-grade encryption" },
                    Feature { icon: IconType::Key, text: "Custom authentication" },
                ],
            },
            Category {
                title: "Style",
                features: &[
                    Feature { icon: IconType::Palette, text: "White-label solutions" },
                    Feature { icon: IconType::Eye, text: "Unlimited customization" },
                ],
            },
            Category {
                title: "Storage",
                features: &[
                    Feature { icon: IconType::HardDrive, text: "500GB storage" },
                    Feature { icon: IconType::FileText, text: "Premium file management" },
                ],
            },
            Category {
                title: "Support",
                features: &[
                    Feature { icon: IconType::Mail, text: "24/7 dedicated support" },
                    Feature { icon: IconType::Clock, text: "Enterprise time tracking" },
                ],
            },
        ],
    },
    PricingTier {
        title: "Enterprise Plan",
        price: None,
        credits: "Unlimited credits",
        button_text: "Contact Sales",
        button_href: "#",
        show_badge: true,
        categories: &[
            Category {
                title: "Security",
                features: &[
                    Feature { icon: IconType::ShieldCheck, text: "Maximum security" },
                    Feature { icon: IconType::Lock, text: "Custom security protocols" },
                    Feature { icon: IconType::Key, text: "Advanced authentication" },
                ],
            },
            Category {
                title: "Style",
                features: &[
                    Feature { icon: IconType::Palette, text: "Full white-label" },
                    Feature { icon: IconType::Eye, text: "Custom design system" },
                ],
            },
            Category {
                title: "Storage",
                features: &[
                    Feature { icon: IconType::HardDrive, text: "Unlimited storage" },
                    Feature { icon: IconType::FileText, text: "Custom file management" },
                ],
            },
            Category {
                title: "Support",
                features: &[
                    Feature { icon: IconType::Mail, text: "Dedicated account manager" },
                    Feature { icon: IconType::Clock, text: "Custom time tracking" },
                ],
            },
        ],
    },
];
```