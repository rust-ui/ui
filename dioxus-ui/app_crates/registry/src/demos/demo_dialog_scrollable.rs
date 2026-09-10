use dioxus::prelude::*;

use crate::ui::dialog::{
    Dialog, DialogAction, DialogBody, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader,
    DialogTitle, DialogTrigger,
};

#[component]
pub fn DemoDialogScrollable() -> Element {
    rsx! {
        Dialog {
            DialogTrigger { "Open Scrollable Dialog" }
            DialogContent {
                DialogBody {
                    DialogHeader {
                        DialogTitle { "Terms of Service" }
                        DialogDescription { "Please read the terms before continuing." }
                    }
                    div { class: "flex flex-col gap-3 text-sm text-muted-foreground",
                        for i in 1..=8 {
                            p {
                                "Section {i}: Lorem ipsum dolor sit amet, consectetur adipiscing elit. \
                                Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. \
                                Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris."
                            }
                        }
                    }
                    DialogFooter {
                        DialogClose { "Decline" }
                        DialogAction { "Accept" }
                    }
                }
            }
        }
    }
}
