use leptos::prelude::*;

use crate::ui::badge::{Badge, BadgeVariant};
use crate::ui::card::{Card, CardAction, CardContent, CardDescription, CardHeader, CardTitle};

#[component]
pub fn DemoCardAction() -> impl IntoView {
    view! {
        <div class="grid gap-4 max-w-2xl sm:grid-cols-2">
            <Card>
                <CardHeader>
                    <CardDescription>"Total Revenue"</CardDescription>
                    <CardTitle class="text-2xl font-bold tabular-nums">"$45,231.89"</CardTitle>
                    <CardAction>
                        <Badge variant=BadgeVariant::Secondary>"+12.5%"</Badge>
                    </CardAction>
                </CardHeader>
                <CardContent>
                    <p class="text-sm text-muted-foreground">"+20.1% from last month"</p>
                </CardContent>
            </Card>

            <Card>
                <CardHeader>
                    <CardDescription>"Active Users"</CardDescription>
                    <CardTitle class="text-2xl font-bold tabular-nums">"2,350"</CardTitle>
                    <CardAction>
                        <Badge variant=BadgeVariant::Destructive>"-3.2%"</Badge>
                    </CardAction>
                </CardHeader>
                <CardContent>
                    <p class="text-sm text-muted-foreground">"-180 from last month"</p>
                </CardContent>
            </Card>
        </div>
    }
}
