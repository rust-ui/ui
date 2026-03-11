use icons::{Maximize, Minimize};
use leptos::prelude::*;

use crate::ui::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};
use crate::ui::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};
use crate::ui::field::{Field, FieldGroup, FieldLabel};
use crate::ui::input::Input;

#[component]
pub fn DemoCollapsibleSettings() -> impl IntoView {
    let open = RwSignal::new(false);

    view! {
        <Card class="mx-auto w-full max-w-xs">
            <CardHeader>
                <CardTitle>"Radius"</CardTitle>
                <CardDescription>"Set the corner radius of the element."</CardDescription>
            </CardHeader>
            <CardContent>
                <Collapsible open=open class="flex gap-2 items-start">
                    <FieldGroup class="grid grid-cols-2 gap-2 w-full">
                        <Field>
                            <FieldLabel html_for="radius-tl" class="sr-only">
                                "Top Left"
                            </FieldLabel>
                            <Input attr:id="radius-tl" attr:placeholder="0" attr:value="0" />
                        </Field>
                        <Field>
                            <FieldLabel html_for="radius-tr" class="sr-only">
                                "Top Right"
                            </FieldLabel>
                            <Input attr:id="radius-tr" attr:placeholder="0" attr:value="0" />
                        </Field>
                        <CollapsibleContent outer_class="col-span-full" class="grid gap-2 grid-cols-subgrid">
                            <Field>
                                <FieldLabel html_for="radius-bl" class="sr-only">
                                    "Bottom Left"
                                </FieldLabel>
                                <Input attr:id="radius-bl" attr:placeholder="0" attr:value="0" />
                            </Field>
                            <Field>
                                <FieldLabel html_for="radius-br" class="sr-only">
                                    "Bottom Right"
                                </FieldLabel>
                                <Input attr:id="radius-br" attr:placeholder="0" attr:value="0" />
                            </Field>
                        </CollapsibleContent>
                    </FieldGroup>
                    <CollapsibleTrigger class="inline-flex justify-center items-center rounded-md border transition-colors size-9 border-input shrink-0 hover:bg-accent hover:text-accent-foreground">
                        {move || {
                            if open.get() {
                                view! { <Minimize class="size-4" /> }.into_any()
                            } else {
                                view! { <Maximize class="size-4" /> }.into_any()
                            }
                        }}
                    </CollapsibleTrigger>
                </Collapsible>
            </CardContent>
        </Card>
    }
}
