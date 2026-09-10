use dioxus::prelude::*;
use icons::Upload;

use crate::ui::dropzone::{Dropzone, DropzoneArea, DropzoneFileList, DropzoneHint, DropzoneIcon, DropzoneLabel};

#[component]
pub fn DemoDropzone() -> Element {
    rsx! {
        div { class: "max-w-[700px] mx-auto w-full",
            Dropzone {
                div { class: "space-y-4",
                    div { class: "space-y-1",
                        h2 { class: "text-base font-semibold", "Upload files" }
                        p { class: "text-sm text-muted-foreground",
                            "Drag and drop your files here or click to browse."
                        }
                    }
                    DropzoneArea {
                        DropzoneIcon { Upload { class: "size-7" } }
                        DropzoneLabel { "Drag & drop files, or click to select" }
                        DropzoneHint { "Any file — up to 8 files, 8 MB each" }
                    }
                    DropzoneFileList {}
                }
            }
        }
    }
}
