use dioxus::prelude::*;

use crate::ui::badge::{Badge, BadgeVariant};
use crate::ui::card::{Card, CardAction, CardContent, CardDescription, CardHeader, CardTitle};

#[component]
pub fn DemoCardAction() -> Element {
    rsx! {
        div { class: "grid gap-4 max-w-2xl sm:grid-cols-2",
            Card {
                CardHeader {
                    CardDescription { "Total Revenue" }
                    CardTitle { class: "text-2xl font-bold tabular-nums", "$45,231.89" }
                    CardAction {
                        Badge { variant: BadgeVariant::Secondary, "+12.5%" }
                    }
                }
                CardContent {
                    p { class: "text-sm text-muted-foreground", "+20.1% from last month" }
                }
            }
            Card {
                CardHeader {
                    CardDescription { "Active Users" }
                    CardTitle { class: "text-2xl font-bold tabular-nums", "2,350" }
                    CardAction {
                        Badge { variant: BadgeVariant::Destructive, "-3.2%" }
                    }
                }
                CardContent {
                    p { class: "text-sm text-muted-foreground", "-180 from last month" }
                }
            }
        }
    }
}
