use dioxus::prelude::*;

use crate::ui::alert_dialog::{
    AlertDialog, AlertDialogBody, AlertDialogClose, AlertDialogContent, AlertDialogDescription, AlertDialogFooter,
    AlertDialogHeader, AlertDialogTitle, AlertDialogTrigger,
};
use crate::ui::button::Button;

#[component]
pub fn DemoAlertDialog() -> Element {
    rsx! {
        AlertDialog {
            AlertDialogTrigger { "Open Alert" }

            AlertDialogContent { class: "w-[425px]",
                AlertDialogBody {
                    AlertDialogHeader {
                        AlertDialogTitle { "Are you absolutely sure?" }

                        AlertDialogDescription {
                            "This action cannot be undone. This will permanently delete your account and remove your data from our servers."
                        }
                    }

                    AlertDialogFooter {
                        AlertDialogClose { class: "w-full sm:w-fit", "Cancel" }
                        Button { button_type: "submit", class: "w-full sm:w-fit", "Continue" }
                    }
                }
            }
        }
    }
}
