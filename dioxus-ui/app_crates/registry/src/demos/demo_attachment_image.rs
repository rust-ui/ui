use dioxus::prelude::*;
use icons::X;

use crate::ui::attachment::{
    Attachment, AttachmentAction, AttachmentActions, AttachmentContent, AttachmentDescription, AttachmentGroup,
    AttachmentMedia, AttachmentMediaVariant, AttachmentOrientation, AttachmentTitle, AttachmentTrigger,
};

#[component]
pub fn DemoAttachmentImage() -> Element {
    rsx! {
        div { class: "py-12 mx-auto w-full max-w-sm",
            AttachmentGroup { class: "w-full",
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
                    AttachmentActions {
                        AttachmentAction { aria_label: "Remove workspace.png", X {} }
                    }
                    AttachmentTrigger {
                        href: "https://images.unsplash.com/photo-1497366754035-f200968a6e72?w=900&auto=format&fit=crop&q=80",
                        aria_label: "Open workspace.png",
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
                    AttachmentActions {
                        AttachmentAction { aria_label: "Remove desk-reference.jpg", X {} }
                    }
                    AttachmentTrigger {
                        href: "https://images.unsplash.com/photo-1497215728101-856f4ea42174?w=900&auto=format&fit=crop&q=80",
                        aria_label: "Open desk-reference.jpg",
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
                    AttachmentActions {
                        AttachmentAction { aria_label: "Remove office-reference.jpg", X {} }
                    }
                    AttachmentTrigger {
                        href: "https://images.unsplash.com/photo-1497366811353-6870744d04b2?w=900&auto=format&fit=crop&q=80",
                        aria_label: "Open office-reference.jpg",
                    }
                }
            }
        }
    }
}
