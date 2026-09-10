use dioxus::prelude::*;
use icons::Bluetooth;

use crate::ui::alert_dialog::{
    AlertDialog, AlertDialogBody, AlertDialogClose, AlertDialogContent, AlertDialogDescription, AlertDialogFooter,
    AlertDialogHeader, AlertDialogTitle, AlertDialogTrigger,
};
use crate::ui::button::Button;

#[component]
pub fn DemoAlertDialogSmallMedia() -> Element {
    rsx! {
        AlertDialog {
            AlertDialogTrigger { "Show Dialog" }
            AlertDialogContent { class: "w-[300px]",
                AlertDialogBody {
                    AlertDialogHeader { class: "items-center sm:items-center sm:text-center",
                        div { class: "flex justify-center items-center mb-2 rounded-md size-10 bg-muted",
                            Bluetooth { class: "size-5" }
                        }
                        AlertDialogTitle { "Allow accessory to connect?" }
                        AlertDialogDescription {
                            "Do you want to allow the Bluetooth accessory to connect to this device?"
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
