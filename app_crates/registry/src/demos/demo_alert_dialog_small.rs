use dioxus::prelude::*;

use crate::ui::alert_dialog::{
    AlertDialog, AlertDialogBody, AlertDialogClose, AlertDialogContent, AlertDialogDescription, AlertDialogFooter,
    AlertDialogHeader, AlertDialogTitle, AlertDialogTrigger,
};
use crate::ui::button::Button;

#[component]
pub fn DemoAlertDialogSmall() -> Element {
    rsx! {
        AlertDialog {
            AlertDialogTrigger { "Show Dialog" }
            AlertDialogContent { class: "w-[300px]",
                AlertDialogBody {
                    AlertDialogHeader { class: "items-center sm:items-center sm:text-center",
                        AlertDialogTitle { "Allow accessory to connect?" }
                        AlertDialogDescription {
                            "Do you want to allow the USB accessory to connect to this device?"
                        }
                    }
                    AlertDialogFooter { class: "flex-row",
                        AlertDialogClose { class: "flex-1", "Don't allow" }
                        Button { button_type: "submit", class: "flex-1", "Allow" }
                    }
                }
            }
        }
    }
}
