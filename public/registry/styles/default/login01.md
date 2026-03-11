---
title: "Login01"
name: "login01"
cargo_dependencies: []
registry_dependencies: ["button", "card", "input", "label"]
type: "components:blocks"
path: "blocks/login01.rs"
---

# Login01

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add login01
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};
use crate::components::ui::input::Input;
use crate::components::ui::label::Label;

#[component]
pub fn Login01() -> impl IntoView {
    view! {
        <div class="flex justify-center items-center p-6 w-full md:p-10 min-h-svh">
            <div class="w-full max-w-sm">
                <div class="flex flex-col gap-6">
                    <Card>
                        <CardHeader>
                            <CardTitle>Login to your account</CardTitle>
                            <CardDescription>Enter your email below to login to your account</CardDescription>
                        </CardHeader>
                        <CardContent>
                            <form>
                                <div class="flex flex-col gap-6">
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
                                                class="inline-block ml-auto text-sm hover:underline underline-offset-4"
                                            >
                                                Forgot your password?
                                            </a>
                                        </div>
                                        <Input attr:r#type="password" attr:id="password" attr:required=true />
                                    </div>
                                    <div class="flex flex-col gap-3">
                                        <Button variant=ButtonVariant::Default class="w-full">
                                            Login
                                        </Button>
                                        <Button variant=ButtonVariant::Outline class="w-full">
                                            Login with Google
                                        </Button>
                                    </div>
                                </div>
                                <div class="mt-4 text-sm text-center">
                                    "Don't have an account?" <a href="#" class="underline underline-offset-4">
                                        Sign up
                                    </a>
                                </div>
                            </form>
                        </CardContent>
                    </Card>
                </div>
            </div>
        </div>
    }
}
```
