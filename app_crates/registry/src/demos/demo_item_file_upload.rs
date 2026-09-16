use dioxus::prelude::*;
use icons::File;

use crate::ui::item::{
    Item, ItemActions, ItemContent, ItemGroup, ItemMedia, ItemMediaVariant, ItemSeparator, ItemSize, ItemTitle,
};
use crate::ui::progress::Progress;

struct UploadFile {
    name: &'static str,
    progress: f64,
    time_remaining: &'static str,
}

const FILES: &[UploadFile] = &[
    UploadFile { name: "document.pdf", progress: 45.0, time_remaining: "2m 30s" },
    UploadFile { name: "presentation.pptx", progress: 78.0, time_remaining: "45s" },
    UploadFile { name: "spreadsheet.xlsx", progress: 12.0, time_remaining: "5m 12s" },
    UploadFile { name: "image.jpg", progress: 100.0, time_remaining: "Complete" },
];

#[component]
pub fn DemoItemFileUpload() -> Element {
    rsx! {
        ItemGroup { class: "w-full max-w-md rounded-md border",
            for (idx, file) in FILES.iter().enumerate() {
                Item { size: ItemSize::Xs, class: "px-4",
                    ItemMedia { variant: ItemMediaVariant::Icon,
                        File { class: "size-5" }
                    }
                    ItemContent { class: "min-w-0 truncate",
                        ItemTitle { class: "inline truncate", "{file.name}" }
                    }
                    ItemContent { class: "flex-none",
                        Progress { value: file.progress, class: "w-24" }
                    }
                    ItemActions { class: "justify-end w-16",
                        span { class: "text-xs text-muted-foreground", "{file.time_remaining}" }
                    }
                }
                if idx + 1 < FILES.len() {
                    ItemSeparator {}
                }
            }
        }
    }
}
