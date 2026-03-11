use leptos::prelude::*;

use crate::ui::button::{Button, ButtonVariant};
use crate::ui::card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardSize, CardTitle};

#[component]
pub fn DemoCardSm() -> impl IntoView {
    view! {
        <Card size=CardSize::Sm class="max-w-sm">
            <CardHeader>
                <CardTitle>"Card Title"</CardTitle>
            </CardHeader>

            <CardContent>
                <CardDescription>
                    "A compact card with reduced padding, ideal for dense UI panels like customizers or sidebars."
                </CardDescription>
            </CardContent>

            <CardFooter class="justify-end">
                <Button variant=ButtonVariant::Outline>"Cancel"</Button>
                <Button>"Confirm"</Button>
            </CardFooter>
        </Card>
    }
}
