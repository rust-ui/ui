---
title: "Login02"
name: "login02"
cargo_dependencies: []
registry_dependencies: ["button", "input", "label"]
type: "components:blocks"
path: "blocks/login02.rs"
---

# Login02

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add login02
```

## Component Code

```rust
use icons::{GalleryVerticalEnd, Github};
use leptos::prelude::*;

use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::input::Input;
use crate::components::ui::label::Label;

#[component]
pub fn Login02() -> impl IntoView {
    view! {
        <div class="grid lg:grid-cols-2 min-h-svh">
            <div class="flex flex-col gap-4 p-6 md:p-10">
                <div class="flex gap-2 justify-center md:justify-start">
                    <a href="#" class="flex gap-2 items-center font-medium">
                        <div class="flex justify-center items-center rounded-md bg-primary text-primary-foreground size-6">
                            <GalleryVerticalEnd class="size-4" />
                        </div>
                        Acme Inc.
                    </a>
                </div>
                <div class="flex flex-1 justify-center items-center">
                    <div class="w-full max-w-xs">
                        <form class="flex flex-col gap-6">
                            <div class="flex flex-col gap-2 items-center text-center">
                                <h1 class="text-2xl font-bold">Login to your account</h1>
                                <p class="text-sm text-muted-foreground text-balance">
                                    Enter your email below to login to your account
                                </p>
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
                                        <a href="#" class="ml-auto text-sm hover:underline underline-offset-4">
                                            Forgot your password?
                                        </a>
                                    </div>
                                    <Input attr:r#type="password" attr:id="password" attr:required=true />
                                </div>

                                <Button class="w-full" attr:r#type="submit">
                                    Login
                                </Button>
                                <div class="relative text-sm text-center after:border-border after:absolute after:inset-0 after:top-1/2 after:z-0 after:flex after:items-center after:border-t">
                                    <span class="relative z-10 px-2 bg-background text-muted-foreground">
                                        Or continue with
                                    </span>
                                </div>
                                <Button variant=ButtonVariant::Outline class="w-full">
                                    <Github />
                                    <span>Login with GitHub</span>
                                </Button>
                            </div>
                            <div class="text-sm text-center">
                                "Don't have an account?" <a href="#" class="underline underline-offset-4">
                                    Sign up
                                </a>
                            </div>
                        </form>
                    </div>
                </div>
            </div>
            <div class="hidden relative lg:block bg-muted">
                <img
                    src="/placeholder.svg"
                    alt="Image"
                    class="object-cover absolute inset-0 w-full h-full dark:brightness-[0.2] dark:grayscale"
                />
            </div>
        </div>
    }
}
```
