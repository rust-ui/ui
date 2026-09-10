use dioxus::prelude::*;
use icons::CirclePlus;

use crate::ui::alert_dialog::{
    AlertDialog, AlertDialogBody, AlertDialogClose, AlertDialogContent, AlertDialogDescription, AlertDialogFooter,
    AlertDialogHeader, AlertDialogTitle, AlertDialogTrigger,
};
use crate::ui::button::Button;

#[component]
pub fn DemoAlertDialogMedia() -> Element {
    rsx! {
        AlertDialog {
            AlertDialogTrigger { "Share Project" }
            AlertDialogContent { class: "w-[425px]",
                AlertDialogBody {
                    AlertDialogHeader {
                        div { class: "flex justify-center items-center mb-2 rounded-md size-10 bg-muted",
                            CirclePlus { class: "size-5" }
                        }
                        AlertDialogTitle { "Share this project?" }
                        AlertDialogDescription {
                            "Anyone with the link will be able to view and edit this project."
                        }
                    }
                    AlertDialogFooter {
                        AlertDialogClose { class: "w-full sm:w-fit", "Cancel" }
                        Button { button_type: "submit", class: "w-full sm:w-fit", "Share" }
                    }
                }
            }
        }
    }
}
