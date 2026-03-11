use icons::GalleryVerticalEnd;
use leptos::prelude::*;

use crate::ui::button::{Button, ButtonVariant};
use crate::ui::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};
use crate::ui::input::Input;
use crate::ui::label::Label;

/*
 * title: Two-Factor Authentication
*/

#[component]
pub fn Login03() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-6 justify-center items-center p-6 md:p-10 bg-muted min-h-svh">
            <div class="flex flex-col gap-6 w-full max-w-sm">
                <a href="#" class="flex gap-2 items-center self-center font-medium">
                    <div class="flex justify-center items-center rounded-md bg-primary text-primary-foreground size-6">
                        <GalleryVerticalEnd class="size-4" />
                    </div>
                    <span>Acme Inc.</span>
                </a>
                <div class="flex flex-col gap-6">
                    <Card>
                        <CardHeader class="text-center">
                            <CardTitle class="text-xl">Welcome back</CardTitle>
                            <CardDescription>Login with your Apple or Google account</CardDescription>
                        </CardHeader>
                        <CardContent>
                            <form>
                                <div class="grid gap-6">
                                    <div class="flex flex-col gap-4">
                                        <Button variant=ButtonVariant::Outline class="w-full">
                                            <LogoApple />
                                            <span>Login with Apple</span>
                                        </Button>
                                        <Button variant=ButtonVariant::Outline class="w-full">
                                            <LogoGoogle />
                                            <span>Login with Google</span>
                                        </Button>
                                    </div>
                                    <div class="relative text-sm text-center after:border-border after:absolute after:inset-0 after:top-1/2 after:z-0 after:flex after:items-center after:border-t">
                                        <span class="relative z-10 px-2 bg-card text-muted-foreground">
                                            Or continue with
                                        </span>
                                    </div>
                                    <div class="grid gap-6">
                                        <div class="grid gap-3">
                                            <Label html_for="email">Email</Label>
                                            <Input
                                                attr:r#type="email"
                                                attr:id="email"
                                                attr:placeholder="m@example.com"
                                                attr:required=true
                                            />
                                        </div>
                                        <div class="grid gap-3">
                                            <div class="flex items-center">
                                                <Label html_for="password">Password</Label>
                                                <a
                                                    attr:href="#"
                                                    class="ml-auto text-sm hover:underline underline-offset-4"
                                                >
                                                    Forgot your password?
                                                </a>
                                            </div>
                                            <Input attr:r#type="password" attr:id="password" attr:required=true />
                                        </div>
                                        <Button class="w-full" attr:r#type="submit">
                                            Login
                                        </Button>
                                    </div>
                                    <div class="text-sm text-center">
                                        "Don't have an account?" <a href="#" class="underline underline-offset-4">
                                            Sign up
                                        </a>
                                    </div>
                                </div>
                            </form>
                        </CardContent>
                    </Card>
                    <div class="text-xs text-center text-muted-foreground *:[a]:hover:text-primary text-balance *:[a]:underline *:[a]:underline-offset-4">
                        By clicking continue, you agree to our <a href="#">Terms of Service</a>and
                        <a href="#">Privacy Policy</a>.
                    </div>
                </div>
            </div>
        </div>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn LogoGoogle() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
            <path
                d="M12.48 10.92v3.28h7.84c-.24 1.84-.853 3.187-1.787 4.133-1.147 1.147-2.933 2.4-6.053 2.4-4.827 0-8.6-3.893-8.6-8.72s3.773-8.72 8.6-8.72c2.6 0 4.507 1.027 5.907 2.347l2.307-2.307C18.747 1.44 16.133 0 12.48 0 5.867 0 .307 5.387.307 12s5.56 12 12.173 12c3.573 0 6.267-1.173 8.373-3.36 2.16-2.16 2.84-5.213 2.84-7.667 0-.76-.053-1.467-.173-2.053H12.48z"
                fill="currentColor"
            ></path>
        </svg>
    }
}

#[component]
pub fn LogoApple() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
            <path
                d="M17.05 20.28c-.98.95-2.05.8-3.08.35-1.09-.46-2.09-.48-3.24 0-1.44.62-2.2.44-3.06-.35C2.79 15.25 3.51 7.59 9.05 7.31c1.35.07 2.29.74 3.08.8 1.18-.24 2.31-.93 3.57-.84 1.51.12 2.65.72 3.4 1.8-3.12 1.87-2.38 5.98.48 7.13-.57 1.5-1.31 2.99-2.54 4.09l.01-.01zM12.03 7.25c-.15-2.23 1.66-4.07 3.74-4.25.29 2.58-2.34 4.5-3.74 4.25z"
                fill="currentColor"
            ></path>
        </svg>
    }
}
