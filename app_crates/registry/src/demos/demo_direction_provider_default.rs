use leptos::prelude::*;

use crate::ui::button::Button;
use crate::ui::card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle};
use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::input::Input;
use crate::ui::label::Label;

#[component]
pub fn DemoDirectionProviderDefault() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Ltr class="w-full max-w-sm">
            <Card>
                <CardHeader>
                    <CardTitle>"Create an account"</CardTitle>
                    <CardDescription>"Enter your details below to create your account"</CardDescription>
                </CardHeader>
                <CardContent>
                    <div class="flex flex-col gap-6">
                        <div class="grid gap-2">
                            <Label html_for="name-ltr-default">"Full Name"</Label>
                            <Input id="name-ltr-default" placeholder="John Doe" />
                        </div>
                        <div class="grid gap-2">
                            <Label html_for="email-ltr-default">"Email"</Label>
                            <Input
                                id="email-ltr-default"
                                r#type=crate::ui::input::InputType::Email
                                placeholder="m@example.com"
                            />
                        </div>
                    </div>
                </CardContent>
                <CardFooter class="flex-col gap-2">
                    <Button class="w-full">"Create Account"</Button>
                    <Button variant=crate::ui::button::ButtonVariant::Outline class="w-full">
                        "Sign up with Google"
                    </Button>
                </CardFooter>
            </Card>
        </DirectionProvider>
    }
}
