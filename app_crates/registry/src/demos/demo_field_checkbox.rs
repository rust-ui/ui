use leptos::prelude::*;

use crate::ui::checkbox::Checkbox;
use crate::ui::field::{
    Field, FieldContent, FieldDescription, FieldGroup, FieldLabel, FieldLegend, FieldSeparator, FieldSet, FieldVariant,
};

#[component]
pub fn DemoFieldCheckbox() -> impl IntoView {
    view! {
        <FieldGroup class="w-full max-w-xs">
            <FieldSet>
                <FieldLegend variant=crate::ui::field::FieldLegendVariant::Label>
                    Show these items on the desktop
                </FieldLegend>
                <FieldDescription>Select the items you want to show on the desktop.</FieldDescription>
                <FieldGroup attr:data-slot="checkbox-group">
                    <Field variant=FieldVariant::Horizontal>
                        <Checkbox attr:id="hard-disks" />
                        <FieldLabel html_for="hard-disks" class="font-normal">
                            Hard disks
                        </FieldLabel>
                    </Field>
                    <Field variant=FieldVariant::Horizontal>
                        <Checkbox attr:id="external-disks" />
                        <FieldLabel html_for="external-disks" class="font-normal">
                            External disks
                        </FieldLabel>
                    </Field>
                    <Field variant=FieldVariant::Horizontal>
                        <Checkbox attr:id="cds-dvds" />
                        <FieldLabel html_for="cds-dvds" class="font-normal">
                            CDs, DVDs, and iPods
                        </FieldLabel>
                    </Field>
                </FieldGroup>
            </FieldSet>
            <FieldSeparator />
            <Field variant=FieldVariant::Horizontal>
                <Checkbox attr:id="sync-folders" checked=true />
                <FieldContent>
                    <FieldLabel html_for="sync-folders">Sync Desktop & Documents folders</FieldLabel>
                    <FieldDescription>
                        Your Desktop & Documents folders are being synced with cloud storage.
                    </FieldDescription>
                </FieldContent>
            </Field>
        </FieldGroup>
    }
}
