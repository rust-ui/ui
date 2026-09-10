use dioxus::prelude::*;
use icons::{Check, ChevronRight, Copy, File as FileIcon, Folder as FolderIcon};
use registry::hooks::use_copy_clipboard::use_copy_clipboard;
use registry::ui::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};

use crate::domain::blocks::block_entry::{BlockFile, BlockFileTreeItem};
use crate::markdown::highlight_code::highlight_code;

#[component]
pub fn BlockCodePanel(files: &'static [BlockFile], tree: Vec<BlockFileTreeItem>) -> Element {
    if files.is_empty() {
        return rsx! { div {} };
    }

    let active_idx = use_signal(|| 0_usize);

    let highlighted = use_memo(move || {
        let Some(f) = files.get(active_idx()) else {
            return String::new();
        };
        highlight_code(f.content, Some(f.language), Some(f.target))
    });

    rsx! {
        div { class: "overflow-hidden rounded-xl border",
            div { class: "flex flex-row",
                // Left: file tree
                div { class: "w-52 border-r shrink-0 bg-muted/30",
                    BlockFileTree { tree, active_idx }
                }
                // Right: code
                div { class: "flex flex-col flex-1 min-w-0",
                    BlockCodeHeader { files, active_idx }
                    pre { class: "overflow-auto p-4 text-xs leading-relaxed whitespace-pre-wrap break-all max-h-[600px] bg-muted",
                        code { dangerous_inner_html: "{highlighted()}" }
                    }
                }
            }
        }
    }
}

#[component]
fn BlockCodeHeader(files: &'static [BlockFile], active_idx: Signal<usize>) -> Element {
    let (copy_fn, copied) = use_copy_clipboard(Some(2000));
    let copy_fn2 = copy_fn.clone();

    rsx! {
        div { class: "flex gap-2 items-center px-4 h-10 border-b bg-card",
            span { class: "font-mono text-xs text-muted-foreground",
                {files.get(active_idx()).map(|f| f.target).unwrap_or_default()}
            }
            div { class: "ml-auto",
                button {
                    class: "inline-flex justify-center items-center rounded-md transition-colors size-7 hover:bg-accent",
                    title: "Copy code",
                    onclick: move |_| {
                        if let Some(f) = files.get(active_idx()) {
                            copy_fn2(f.content);
                        }
                    },
                    if copied() {
                        Check { class: "size-3.5" }
                    } else {
                        Copy { class: "size-3.5" }
                    }
                }
            }
        }
    }
}

#[component]
fn BlockFileTree(tree: Vec<BlockFileTreeItem>, active_idx: Signal<usize>) -> Element {
    rsx! {
        div { class: "flex flex-col gap-0.5 p-2",
            p { class: "py-1 px-2 text-xs font-medium text-muted-foreground", "Files" }
            for item in tree {
                BlockFileTreeNode { item, active_idx, depth: 0 }
            }
        }
    }
}

#[component]
fn BlockFileTreeNode(item: BlockFileTreeItem, active_idx: Signal<usize>, depth: usize) -> Element {
    let padding_left = format!("calc(0.5rem + {}px)", depth * 12);

    match item {
        BlockFileTreeItem::File { name, index } => {
            let is_active = active_idx() == index;
            rsx! {
                button {
                    "data-name": "BlockFileTreeItem",
                    class: "flex gap-1.5 items-center py-1 px-2 w-full text-xs text-left rounded-md transition-colors hover:bg-accent",
                    class: if is_active { "bg-accent font-medium" },
                    style: "padding-left: {padding_left}",
                    onclick: move |_| active_idx.set(index),
                    FileIcon { class: "size-3.5 shrink-0" }
                    {name}
                }
            }
        }
        BlockFileTreeItem::Folder { name, items } => {
            rsx! {
                Collapsible { default_open: true,
                    CollapsibleTrigger { class: "flex gap-1.5 items-center py-1 px-2 w-full text-xs rounded-md group hover:bg-accent",
                        ChevronRight { class: "transition-transform duration-200 size-3.5 shrink-0 group-data-[state=open]:rotate-90" }
                        FolderIcon { class: "size-3.5 shrink-0" }
                        {name}
                    }
                    CollapsibleContent {
                        for child in items {
                            BlockFileTreeNode { item: child, active_idx, depth: depth + 1 }
                        }
                    }
                }
            }
        }
    }
}
