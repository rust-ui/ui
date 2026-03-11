use leptos::prelude::*;

use crate::ui::button::Button;
use crate::ui::card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle};
use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::input::Input;
use crate::ui::label::Label;

#[component]
pub fn DemoDirectionProvider() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Ltr class="w-full max-w-sm">
            <Card>
                <CardHeader>
                    <CardTitle>"Login to your account"</CardTitle>
                    <CardDescription>"Enter your email below to login to your account"</CardDescription>
                </CardHeader>
                <CardContent>
                    <div class="flex flex-col gap-6">
                        <div class="grid gap-2">
                            <Label html_for="email-ltr">"Email"</Label>
                            <Input
                                id="email-ltr"
                                r#type=crate::ui::input::InputType::Email
                                placeholder="m@example.com"
                            />
                        </div>
                        <div class="grid gap-2">
                            <div class="flex items-center">
                                <Label html_for="password-ltr">"Password"</Label>
                                <a href="#" class="text-sm hover:underline ms-auto underline-offset-4">
                                    "Forgot your password?"
                                </a>
                            </div>
                            <Input id="password-ltr" r#type=crate::ui::input::InputType::Password />
                        </div>
                    </div>
                </CardContent>
                <CardFooter class="flex-col gap-2">
                    <Button class="w-full">"Login"</Button>
                    <Button variant=crate::ui::button::ButtonVariant::Outline class="w-full">
                        "Login with Google"
                    </Button>
                </CardFooter>
            </Card>
        </DirectionProvider>
    }
}
