use dioxus::prelude::*;
use icons::{FileCode, FileText, Table, X};

use crate::ui::attachment::{
    Attachment, AttachmentAction, AttachmentActions, AttachmentContent, AttachmentDescription, AttachmentGroup,
    AttachmentMedia, AttachmentMediaVariant, AttachmentTitle,
};

#[component]
pub fn DemoAttachmentGroup() -> Element {
    rsx! {
        div { class: "py-12 mx-auto w-full max-w-sm",
            AttachmentGroup { class: "w-full",
                Attachment { class: "w-64",
                    AttachmentMedia { FileText {} }
                    AttachmentContent {
                        AttachmentTitle { "briefing-notes.pdf" }
                        AttachmentDescription { "PDF · 1.4 MB" }
                    }
                    AttachmentActions {
                        AttachmentAction { aria_label: "Remove briefing-notes.pdf", X {} }
                    }
                }
                Attachment { class: "w-64",
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
                    AttachmentActions {
                        AttachmentAction { aria_label: "Remove workspace.png", X {} }
                    }
                }
                Attachment { class: "w-64",
                    AttachmentMedia { Table {} }
                    AttachmentContent {
                        AttachmentTitle { "customers.csv" }
                        AttachmentDescription { "CSV · 18 KB" }
                    }
                    AttachmentActions {
                        AttachmentAction { aria_label: "Remove customers.csv", X {} }
                    }
                }
                Attachment { class: "w-64",
                    AttachmentMedia { FileCode {} }
                    AttachmentContent {
                        AttachmentTitle { "renderer.tsx" }
                        AttachmentDescription { "TSX · 12 KB" }
                    }
                    AttachmentActions {
                        AttachmentAction { aria_label: "Remove renderer.tsx", X {} }
                    }
                }
            }
        }
    }
}
