use icons::Check;
use leptos::prelude::*;

use crate::ui::button::Button;
use crate::ui::card::{Card, CardContent, CardFooter, CardHeader, CardItem, CardList};
use crate::ui::separator::Separator;

/*
 * title: Simple Pricing with Features
*/

const FEATURE_GROUPS: &[&[&str]] = &[
    &["Unlimited", "Integrations", "24/7 support"],
    &["Live collaborations", "Unlimited storage", "30-day money back"],
    &["Unlimited members", "Customization", "Unlimited users"],
];

#[component]
pub fn Pricing01() -> impl IntoView {
    view! {
        <section class="py-20">
            <div class="container">
                <div class="flex flex-col gap-6 items-center mx-auto max-w-5xl text-center">
                    <h2 class="text-4xl font-semibold lg:text-6xl text-pretty">Pricing</h2>
                    <p class="max-w-md lg:text-xl text-muted-foreground">Simple pricing with a free 7 day trial.</p>
                    <Card class="mx-auto w-full sm:w-fit sm:min-w-80">
                        <CardHeader>
                            <div class="flex justify-center">
                                <span class="text-lg font-semibold">$</span>
                                <span class="text-6xl font-semibold">29</span>
                                <span class="self-end text-muted-foreground">/mo</span>
                            </div>
                        </CardHeader>
                        <CardContent>
                            {FEATURE_GROUPS
                                .iter()
                                .enumerate()
                                .map(|(idx, group)| {
                                    view! {
                                        <div>
                                            <CardList class="gap-3">
                                                {group
                                                    .iter()
                                                    .map(|feature| {
                                                        view! {
                                                            <CardItem class="justify-between text-sm font-medium">
                                                                <span>{*feature}</span>
                                                                <Check class="inline" attr:aria-hidden="true" />
                                                            </CardItem>
                                                        }
                                                    })
                                                    .collect_view()}
                                            </CardList>
                                            {(idx < FEATURE_GROUPS.len() - 1)
                                                .then(|| view! { <Separator class="my-6" /> })}
                                        </div>
                                    }
                                })
                                .collect_view()}
                        </CardContent>
                        <CardFooter>
                            <Button class="w-full" href="#">
                                Start free trial
                            </Button>
                        </CardFooter>
                    </Card>
                </div>
            </div>
        </section>
    }
}
