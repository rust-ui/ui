use dioxus::prelude::*;
use icons::{Copy, FileSearch, X};

use crate::ui::attachment::{
    Attachment, AttachmentAction, AttachmentActions, AttachmentContent, AttachmentDescription, AttachmentMedia,
    AttachmentTitle, AttachmentTrigger,
};
use crate::ui::dialog::{Dialog, DialogBody, DialogContent, DialogDescription, DialogHeader, DialogTitle};

#[component]
pub fn DemoAttachmentTrigger() -> Element {
    rsx! {
        div { class: "py-12 mx-auto w-full max-w-sm",
            Dialog { class: "w-full",
                Attachment { class: "w-full",
                    AttachmentMedia { FileSearch {} }
                    AttachmentContent {
                        AttachmentTitle { "research-summary.pdf" }
                        AttachmentDescription { "Open preview dialog" }
                    }
                    AttachmentActions {
                        AttachmentAction { aria_label: "Copy link", Copy {} }
                        AttachmentAction { aria_label: "Remove research-summary.pdf", X {} }
                    }
                    AttachmentTrigger { aria_label: "Preview research-summary.pdf" }
                }

                DialogContent { class: "sm:max-w-md",
                    DialogBody {
                        DialogHeader {
                            DialogTitle { "research-summary.pdf" }
                            DialogDescription {
                                "The attachment trigger fills the card and opens the dialog, while the actions stay independently clickable above it."
                            }
                        }
                    }
                }
            }
        }
    }
}
