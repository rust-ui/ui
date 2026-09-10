use dioxus::prelude::*;
use icons::FileText;

use crate::ui::attachment::{
    Attachment, AttachmentContent, AttachmentDescription, AttachmentMedia, AttachmentSize, AttachmentTitle,
};

#[component]
pub fn DemoAttachmentSizes() -> Element {
    rsx! {
        div { class: "flex flex-col gap-3 py-12 mx-auto w-full max-w-sm",
            Attachment { size: AttachmentSize::Default, class: "w-full",
                AttachmentMedia { FileText {} }
                AttachmentContent {
                    AttachmentTitle { "Default attachment" }
                    AttachmentDescription { "PDF · 2.4 MB" }
                }
            }
            Attachment { size: AttachmentSize::Sm, class: "w-full",
                AttachmentMedia { FileText {} }
                AttachmentContent {
                    AttachmentTitle { "Small attachment" }
                    AttachmentDescription { "PDF · 2.4 MB" }
                }
            }
            Attachment { size: AttachmentSize::Xs, class: "w-full",
                AttachmentMedia { FileText {} }
                AttachmentContent {
                    AttachmentTitle { "Extra small attachment" }
                }
            }
        }
    }
}
