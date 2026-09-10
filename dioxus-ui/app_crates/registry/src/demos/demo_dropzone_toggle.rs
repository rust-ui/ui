use dioxus::prelude::*;
use icons::Upload;

use crate::ui::dropzone::{
    Dropzone, DropzoneArea, DropzoneCtx, DropzoneFileGrid, DropzoneFileList, DropzoneHint, DropzoneIcon, DropzoneLabel,
    DropzoneViewToggle, ViewMode,
};

#[component]
pub fn DemoDropzoneToggle() -> Element {
    rsx! {
        div { class: "max-w-[700px] mx-auto w-full",
            Dropzone {
                div { class: "space-y-4",
                    div { class: "flex items-center justify-between",
                        div { class: "space-y-1",
                            h2 { class: "text-base font-semibold", "Upload files" }
                            p { class: "text-sm text-muted-foreground",
                                "Toggle between list and grid view after dropping files."
                            }
                        }
                        DropzoneViewToggle {}
                    }
                    DropzoneArea {
                        DropzoneIcon { Upload { class: "size-7" } }
                        DropzoneLabel { "Drag & drop files, or click to select" }
                        DropzoneHint { "Any file — up to 8 files, 8 MB each" }
                    }
                    ViewSwitcher {}
                }
            }
        }
    }
}

#[component]
fn ViewSwitcher() -> Element {
    let ctx = use_context::<DropzoneCtx>();
    let view = *ctx.view.read();
    match view {
        ViewMode::List => rsx! { DropzoneFileList {} },
        ViewMode::Grid => rsx! { DropzoneFileGrid {} },
    }
}
