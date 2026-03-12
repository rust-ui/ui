---
title: "Demo Direction Provider Default"
name: "demo_direction_provider_default"
cargo_dependencies: []
registry_dependencies: ["button", "card", "direction_provider", "input", "label"]
type: "components:demos"
path: "demos/demo_direction_provider_default.rs"
---

# Demo Direction Provider Default

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_direction_provider_default
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::button::Button;
use crate::components::ui::card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle};
use crate::components::ui::direction_provider::{Direction, DirectionProvider};
use crate::components::ui::input::Input;
use crate::components::ui::label::Label;

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
```
