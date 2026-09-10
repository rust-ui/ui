use dioxus::prelude::*;

use crate::ui::button::{Button, ButtonVariant};
use crate::ui::sheet::{
    Sheet, SheetClose, SheetContent, SheetDescription, SheetDirection, SheetFooter, SheetHeader, SheetTitle,
    SheetTrigger,
};

#[component]
pub fn DemoSheetNoCloseButton() -> Element {
    rsx! {
        Sheet {
            SheetTrigger { "Open Sheet" }

            SheetContent { direction: SheetDirection::Right, show_close_button: false,
                SheetHeader {
                    SheetTitle { "No Close Button" }
                    SheetDescription { "This sheet hides the default close button." }
                }

                div { class: "p-4 text-sm text-muted-foreground",
                    "Use the Cancel button or press ESC to close."
                }

                SheetFooter {
                    SheetClose { variant: ButtonVariant::Outline, "Cancel" }
                    Button { "Confirm" }
                }
            }
        }
    }
}
