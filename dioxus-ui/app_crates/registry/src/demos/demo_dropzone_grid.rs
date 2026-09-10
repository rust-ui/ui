use dioxus::prelude::*;
use icons::Upload;

use crate::ui::dropzone::{Dropzone, DropzoneArea, DropzoneFileGrid, DropzoneHint, DropzoneIcon, DropzoneLabel};

#[component]
pub fn DemoDropzoneGrid() -> Element {
    rsx! {
        div { class: "max-w-[700px] mx-auto w-full",
            Dropzone {
                div { class: "space-y-4",
                    div { class: "space-y-1",
                        h2 { class: "text-base font-semibold", "Upload files" }
                        p { class: "text-sm text-muted-foreground",
                            "Files appear as a card grid with image/video previews."
                        }
                    }
                    DropzoneArea {
                        DropzoneIcon { Upload { class: "size-7" } }
                        DropzoneLabel { "Drag & drop files, or click to select" }
                        DropzoneHint { "Any file — up to 8 files, 8 MB each" }
                    }
                    DropzoneFileGrid {}
                }
            }
        }
    }
}
