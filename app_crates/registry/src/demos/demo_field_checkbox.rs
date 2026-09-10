use dioxus::prelude::*;

use crate::ui::checkbox::Checkbox;
use crate::ui::field::{
    Field, FieldContent, FieldDescription, FieldGroup, FieldLabel, FieldLegend, FieldLegendVariant, FieldSeparator,
    FieldSet, FieldVariant,
};

#[component]
pub fn DemoFieldCheckbox() -> Element {
    rsx! {
        FieldGroup { class: "w-full max-w-xs",
            FieldSet {
                FieldLegend { variant: FieldLegendVariant::Label, "Show these items on the desktop" }
                FieldDescription { "Select the items you want to show on the desktop." }
                FieldGroup { data_slot: "checkbox-group",
                    Field { variant: FieldVariant::Horizontal,
                        Checkbox { }
                        FieldLabel { class: "font-normal", "Hard disks" }
                    }
                    Field { variant: FieldVariant::Horizontal,
                        Checkbox { }
                        FieldLabel { class: "font-normal", "External disks" }
                    }
                    Field { variant: FieldVariant::Horizontal,
                        Checkbox { }
                        FieldLabel { class: "font-normal", "CDs, DVDs, and iPods" }
                    }
                }
            }
            FieldSeparator {}
            Field { variant: FieldVariant::Horizontal,
                Checkbox { checked: true }
                FieldContent {
                    FieldLabel { "Sync Desktop & Documents folders" }
                    FieldDescription {
                        "Your Desktop & Documents folders are being synced with cloud storage."
                    }
                }
            }
        }
    }
}
