use leptos::prelude::*;

use crate::ui::checkbox::Checkbox;
use crate::ui::form::{FormDescription, FormGroup, FormLabel, FormSeparator, FormSet};

#[component]
pub fn DemoFormGroup() -> impl IntoView {
    view! {
        <div class="w-full max-w-md">
            <FormGroup>
                <FormSet>
                    <FormLabel>Responses</FormLabel>
                    <FormDescription>
                        Get notified when ChatGPT responds to requests that take time, like research or image generation.
                    </FormDescription>
                    <FormGroup attr:data-name="CheckboxGroup">
                        <div class="flex flex-row gap-3 items-center w-full">
                            <Checkbox attr:id="push" checked=true disabled=true />
                            <FormLabel html_for="push" class="font-normal">
                                Push notifications
                            </FormLabel>
                        </div>
                    </FormGroup>
                </FormSet>
                <FormSeparator />
                <FormSet>
                    <FormLabel>Tasks</FormLabel>
                    <FormDescription>
                        "Get notified when tasks you've created have updates. " <a href="#">Manage tasks</a>
                    </FormDescription>
                    <FormGroup attr:data-name="CheckboxGroup">
                        <div class="flex flex-row gap-3 items-center w-full">
                            <Checkbox attr:id="push-tasks" />
                            <FormLabel html_for="push-tasks" class="font-normal">
                                Push notifications
                            </FormLabel>
                        </div>
                        <div class="flex flex-row gap-3 items-center w-full">
                            <Checkbox attr:id="email-tasks" />
                            <FormLabel html_for="email-tasks" class="font-normal">
                                Email notifications
                            </FormLabel>
                        </div>
                    </FormGroup>
                </FormSet>
            </FormGroup>
        </div>
    }
}
