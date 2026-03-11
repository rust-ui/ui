---
title: "Demo Input Group In Card"
name: "demo_input_group_in_card"
cargo_dependencies: []
registry_dependencies: ["button", "card", "field", "input", "input_group"]
type: "components:demos"
path: "demos/demo_input_group_in_card.rs"
---

# Demo Input Group In Card

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_input_group_in_card
```

## Component Code

```rust
use icons::{ExternalLink, Mail};
use leptos::prelude::*;

use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle};
use crate::components::ui::field::{Field, FieldGroup, FieldLabel};
use crate::components::ui::input::InputType;
use crate::components::ui::input_group::{
    InputGroup, InputGroupAddon, InputGroupAddonAlign, InputGroupInput, InputGroupText, InputGroupTextarea,
};

#[component]
pub fn DemoInputGroupInCard() -> impl IntoView {
    view! {
        <Card class="w-full max-w-md">
            <CardHeader>
                <CardTitle>"Profile Settings"</CardTitle>
                <CardDescription>"Update your contact information."</CardDescription>
            </CardHeader>
            <CardContent>
                <FieldGroup>
                    <Field>
                        <FieldLabel html_for="card-email">"Email Address"</FieldLabel>
                        <InputGroup>
                            <InputGroupInput id="card-email" r#type=InputType::Email placeholder="you@example.com" />
                            <InputGroupAddon align=InputGroupAddonAlign::InlineEnd>
                                <Mail />
                            </InputGroupAddon>
                        </InputGroup>
                    </Field>

                    <Field>
                        <FieldLabel html_for="card-website">"Website"</FieldLabel>
                        <InputGroup>
                            <InputGroupAddon>
                                <InputGroupText>"https://"</InputGroupText>
                            </InputGroupAddon>
                            <InputGroupInput id="card-website" placeholder="example.com" />
                            <InputGroupAddon align=InputGroupAddonAlign::InlineEnd>
                                <ExternalLink />
                            </InputGroupAddon>
                        </InputGroup>
                    </Field>

                    <Field>
                        <FieldLabel html_for="card-bio">"Bio"</FieldLabel>
                        <InputGroup>
                            <InputGroupTextarea
                                attr:id="card-bio"
                                attr:placeholder="Tell us a little about yourself..."
                                attr:rows="3"
                            />
                            <InputGroupAddon align=InputGroupAddonAlign::BlockEnd class="border-t">
                                <InputGroupText>"0 / 200 characters"</InputGroupText>
                            </InputGroupAddon>
                        </InputGroup>
                    </Field>
                </FieldGroup>
            </CardContent>
            <CardFooter class="gap-2 justify-end">
                <Button variant=ButtonVariant::Outline>"Cancel"</Button>
                <Button>"Save changes"</Button>
            </CardFooter>
        </Card>
    }
}
```
