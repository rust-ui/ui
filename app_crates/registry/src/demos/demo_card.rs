use leptos::prelude::*;

use crate::ui::button::{Button, ButtonVariant};
use crate::ui::card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle};

#[component]
pub fn DemoCard() -> impl IntoView {
    view! {
        <Card class="max-w-lg @lg:max-w-2xl">
            <CardHeader>
                <CardTitle>"Card Title"</CardTitle>
            </CardHeader>

            <CardContent>
                <CardDescription>
                    "Hello, this is a more detailed description of the card content. You can add more text here to provide additional information about the card's purpose, features, or any other relevant details that might interest the viewer."
                </CardDescription>
            </CardContent>

            <CardFooter class="justify-end">
                <Button variant=ButtonVariant::Outline>"Cancel"</Button>
                <Button>"Confirm"</Button>
            </CardFooter>
        </Card>
    }
}
