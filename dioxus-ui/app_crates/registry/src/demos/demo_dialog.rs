use dioxus::prelude::*;

use crate::ui::dialog::{
    Dialog, DialogAction, DialogBody, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader,
    DialogTitle, DialogTrigger,
};
use crate::ui::input::Input;
use crate::ui::label::Label;

#[component]
pub fn DemoDialog() -> Element {
    rsx! {
        Dialog {
            DialogTrigger { "Open Dialog" }
            DialogContent {
                DialogBody {
                    DialogHeader {
                        DialogTitle { "Edit profile" }
                        DialogDescription {
                            "Make changes to your profile here. Click save when you're done."
                        }
                    }
                    div { class: "flex flex-col gap-4",
                        div { class: "flex flex-col gap-2",
                            Label { html_for: "dialog-name", "Name" }
                            Input { id: "dialog-name", placeholder: "Max Wells" }
                        }
                        div { class: "flex flex-col gap-2",
                            Label { html_for: "dialog-username", "Username" }
                            Input { id: "dialog-username", placeholder: "@maxwells" }
                        }
                    }
                    DialogFooter {
                        DialogClose { "Cancel" }
                        DialogAction { "Save changes" }
                    }
                }
            }
        }
    }
}
