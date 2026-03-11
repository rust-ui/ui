use leptos::prelude::*;
use registry::ui::button::{Button, ButtonVariant};
use registry::ui::card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle};
use registry::ui::switch::Switch;

#[component]
pub fn CardCookieSettings() -> impl IntoView {
    view! {
        <Card>
            <CardHeader>
                <CardTitle>Cookie Settings</CardTitle>
                <CardDescription>Manage your cookie settings here.</CardDescription>
            </CardHeader>
            <CardContent class="grid gap-6">
                <div class="flex gap-4 justify-between items-start">
                    <div class="flex flex-col gap-1.5">
                        <span class="text-base font-medium">Strictly Necessary</span>
                        <span class="text-sm font-normal leading-snug text-muted-foreground">
                            These cookies are essential in order to use the website and use its features.
                        </span>
                    </div>
                    <Switch checked=true />
                </div>
                <div class="flex gap-4 justify-between items-start">
                    <div class="flex flex-col gap-1.5">
                        <span class="text-base font-medium">Functional Cookies</span>
                        <span class="text-sm font-normal leading-snug text-muted-foreground">
                            These cookies allow the website to provide personalized functionality.
                        </span>
                    </div>
                    <Switch />
                </div>
            </CardContent>
            <CardFooter>
                <Button variant=ButtonVariant::Outline class="w-full">
                    Save preferences
                </Button>
            </CardFooter>
        </Card>
    }
}
