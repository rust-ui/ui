use dioxus::prelude::*;
use icons::{Check, Clock, FileText, FileWarning, RefreshCw, X};

use crate::ui::attachment::{
    Attachment, AttachmentAction, AttachmentActions, AttachmentContent, AttachmentDescription, AttachmentMedia,
    AttachmentState, AttachmentTitle,
};
use crate::ui::spinner::Spinner;

#[component]
pub fn DemoAttachmentStates() -> Element {
    rsx! {
        div { class: "flex flex-col gap-2 py-12 mx-auto w-full max-w-sm",
            Attachment { state: AttachmentState::Idle, class: "w-full",
                AttachmentMedia { Clock {} }
                AttachmentContent {
                    AttachmentTitle { "selected-file.pdf" }
                    AttachmentDescription { "Ready to upload" }
                }
                AttachmentActions {
                    AttachmentAction { aria_label: "Remove selected-file.pdf", X {} }
                }
            }
            Attachment { state: AttachmentState::Uploading, class: "w-full",
                AttachmentMedia { Spinner {} }
                AttachmentContent {
                    AttachmentTitle { "design-system.zip" }
                    AttachmentDescription { "Uploading · 64%" }
                }
                AttachmentActions {
                    AttachmentAction { aria_label: "Cancel upload", X {} }
                }
            }
            Attachment { state: AttachmentState::Processing, class: "w-full",
                AttachmentMedia { FileText {} }
                AttachmentContent {
                    AttachmentTitle { "market-research.pdf" }
                    AttachmentDescription { "Processing document" }
                }
                AttachmentActions {
                    AttachmentAction { aria_label: "Remove market-research.pdf", X {} }
                }
            }
            Attachment { state: AttachmentState::Error, class: "w-full",
                AttachmentMedia { FileWarning {} }
                AttachmentContent {
                    AttachmentTitle { "financial-model.xlsx" }
                    AttachmentDescription { "Upload failed. Try again." }
                }
                AttachmentActions {
                    AttachmentAction { aria_label: "Retry upload", RefreshCw {} }
                    AttachmentAction { aria_label: "Remove financial-model.xlsx", X {} }
                }
            }
            Attachment { state: AttachmentState::Done, class: "w-full",
                AttachmentMedia { Check {} }
                AttachmentContent {
                    AttachmentTitle { "uploaded-report.pdf" }
                    AttachmentDescription { "Uploaded · 1.8 MB" }
                }
                AttachmentActions {
                    AttachmentAction { aria_label: "Remove uploaded-report.pdf", X {} }
                }
            }
        }
    }
}
