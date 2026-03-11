---
title: "Demo Field Checkbox"
name: "demo_field_checkbox"
cargo_dependencies: []
registry_dependencies: ["checkbox", "field"]
type: "components:demos"
path: "demos/demo_field_checkbox.rs"
---

# Demo Field Checkbox

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_field_checkbox
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::checkbox::Checkbox;
use crate::components::ui::field::{
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
```
