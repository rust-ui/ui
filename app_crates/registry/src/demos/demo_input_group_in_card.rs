use icons::{ExternalLink, Mail};
use leptos::prelude::*;

use crate::ui::button::{Button, ButtonVariant};
use crate::ui::card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle};
use crate::ui::field::{Field, FieldGroup, FieldLabel};
use crate::ui::input::InputType;
use crate::ui::input_group::{
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
