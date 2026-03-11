use leptos::prelude::*;

use crate::ui::field::{Field, FieldContent, FieldDescription, FieldGroup, FieldLabel, FieldTitle, FieldVariant};
use crate::ui::switch::Switch;

#[component]
pub fn DemoSwitchChoiceCard() -> impl IntoView {
    view! {
        <FieldGroup class="w-full max-w-sm">
            <FieldLabel>
                <Field variant=FieldVariant::Horizontal>
                    <FieldContent>
                        <FieldTitle>"Share across devices"</FieldTitle>
                        <FieldDescription>
                            "Focus is shared across devices, and turns off when you leave the app."
                        </FieldDescription>
                    </FieldContent>
                    <Switch id="switch-share" />
                </Field>
            </FieldLabel>
            <FieldLabel>
                <Field variant=FieldVariant::Horizontal>
                    <FieldContent>
                        <FieldTitle>"Enable notifications"</FieldTitle>
                        <FieldDescription>
                            "Receive notifications when focus mode is enabled or disabled."
                        </FieldDescription>
                    </FieldContent>
                    <Switch id="switch-notifications" checked=true />
                </Field>
            </FieldLabel>
        </FieldGroup>
    }
}
