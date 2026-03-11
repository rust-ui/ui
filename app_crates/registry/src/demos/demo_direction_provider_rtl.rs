use leptos::prelude::*;

use crate::ui::button::Button;
use crate::ui::card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle};
use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::input::Input;
use crate::ui::label::Label;

#[component]
pub fn DemoDirectionProviderRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="w-full max-w-sm">
            <Card>
                <CardHeader>
                    <CardTitle>"تسجيل الدخول إلى حسابك"</CardTitle>
                    <CardDescription>
                        "أدخل بريدك الإلكتروني أدناه لتسجيل الدخول إلى حسابك"
                    </CardDescription>
                </CardHeader>
                <CardContent>
                    <div class="flex flex-col gap-6">
                        <div class="grid gap-2">
                            <Label html_for="email-rtl">"البريد الإلكتروني"</Label>
                            <Input
                                id="email-rtl"
                                r#type=crate::ui::input::InputType::Email
                                placeholder="m@example.com"
                            />
                        </div>
                        <div class="grid gap-2">
                            <div class="flex items-center">
                                <Label html_for="password-rtl">"كلمة المرور"</Label>
                                <a href="#" class="text-sm hover:underline ms-auto underline-offset-4">
                                    "نسيت كلمة المرور؟"
                                </a>
                            </div>
                            <Input id="password-rtl" r#type=crate::ui::input::InputType::Password />
                        </div>
                    </div>
                </CardContent>
                <CardFooter class="flex-col gap-2">
                    <Button class="w-full">"تسجيل الدخول"</Button>
                    <Button variant=crate::ui::button::ButtonVariant::Outline class="w-full">
                        "تسجيل الدخول باستخدام Google"
                    </Button>
                </CardFooter>
            </Card>
        </DirectionProvider>
    }
}
