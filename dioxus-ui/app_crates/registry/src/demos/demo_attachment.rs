use dioxus::prelude::*;
use icons::{FileCode, X};

use crate::ui::attachment::{
    Attachment, AttachmentAction, AttachmentActions, AttachmentContent, AttachmentDescription, AttachmentGroup,
    AttachmentMedia, AttachmentMediaVariant, AttachmentOrientation, AttachmentState, AttachmentTitle,
};
use crate::ui::spinner::Spinner;

#[component]
pub fn DemoAttachment() -> Element {
    rsx! {
        div { class: "flex flex-col gap-3 py-12 mx-auto w-full max-w-sm",
            AttachmentGroup {
                Attachment { orientation: AttachmentOrientation::Vertical,
                    AttachmentMedia { variant: AttachmentMediaVariant::Image,
                        img {
                            src: "https://images.unsplash.com/photo-1497366754035-f200968a6e72?w=900&auto=format&fit=crop&q=80",
                            alt: "Workspace",
                        }
                    }
                    AttachmentContent {
                        AttachmentTitle { "workspace.png" }
                        AttachmentDescription { "PNG · 820 KB" }
                    }
                }
                Attachment { orientation: AttachmentOrientation::Vertical,
                    AttachmentMedia { variant: AttachmentMediaVariant::Image,
                        img {
                            src: "https://images.unsplash.com/photo-1497215728101-856f4ea42174?w=900&auto=format&fit=crop&q=80",
                            alt: "Desk",
                        }
                    }
                    AttachmentContent {
                        AttachmentTitle { "desk-reference.jpg" }
                        AttachmentDescription { "JPG · 1.1 MB" }
                    }
                }
                Attachment { orientation: AttachmentOrientation::Vertical,
                    AttachmentMedia { variant: AttachmentMediaVariant::Image,
                        img {
                            src: "https://images.unsplash.com/photo-1497366811353-6870744d04b2?w=900&auto=format&fit=crop&q=80",
                            alt: "Office",
                        }
                    }
                    AttachmentContent {
                        AttachmentTitle { "office-reference.jpg" }
                        AttachmentDescription { "JPG · 940 KB" }
                    }
                }
            }
            Attachment { state: AttachmentState::Uploading, class: "w-full",
                AttachmentMedia { Spinner {} }
                AttachmentContent {
                    AttachmentTitle { "sales-dashboard.pdf" }
                    AttachmentDescription { "Uploading · 64%" }
                }
                AttachmentActions {
                    AttachmentAction { aria_label: "Cancel upload", X {} }
                }
            }
            Attachment { class: "w-full",
                AttachmentMedia { FileCode {} }
                AttachmentContent {
                    AttachmentTitle { "message-renderer.tsx" }
                    AttachmentDescription { "TypeScript · 12 KB" }
                }
                AttachmentActions {
                    AttachmentAction { aria_label: "Remove message-renderer.tsx", X {} }
                }
            }
        }
    }
}
