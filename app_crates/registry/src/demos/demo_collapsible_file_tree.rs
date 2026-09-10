use dioxus::prelude::*;
use icons::{ChevronRight, File, Folder};

use crate::ui::card::{Card, CardContent, CardHeader};
use crate::ui::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};
use crate::ui::tabs::{Tabs, TabsList, TabsTrigger};

#[derive(Clone, PartialEq)]
pub enum FileTreeItem {
    File { name: &'static str },
    Folder { name: &'static str, items: Vec<FileTreeItem> },
}

#[component]
fn FileTreeNode(item: FileTreeItem) -> Element {
    match item {
        FileTreeItem::File { name } => rsx! {
            div { class: "flex gap-2 items-center py-1 px-2 w-full text-sm rounded-md transition-colors text-foreground hover:bg-accent hover:text-accent-foreground",
                File { class: "size-4 shrink-0" }
                span { {name} }
            }
        },
        FileTreeItem::Folder { name, items } => rsx! {
            Collapsible {
                CollapsibleTrigger { class: "flex gap-2 items-center py-1 px-2 w-full text-sm rounded-md transition-colors group hover:bg-accent hover:text-accent-foreground",
                    ChevronRight { class: "transition-transform duration-200 size-4 shrink-0 group-data-[state=open]:rotate-90" }
                    Folder { class: "size-4 shrink-0" }
                    {name}
                }
                CollapsibleContent { class: "flex flex-col gap-1 mt-1 ml-5",
                    for child in items {
                        FileTreeNode { item: child }
                    }
                }
            }
        },
    }
}

#[component]
pub fn DemoCollapsibleFileTree() -> Element {
    let file_tree: Vec<FileTreeItem> = vec![
        FileTreeItem::Folder {
            name: "components",
            items: vec![
                FileTreeItem::Folder {
                    name: "ui",
                    items: vec![
                        FileTreeItem::File { name: "button.tsx" },
                        FileTreeItem::File { name: "card.tsx" },
                        FileTreeItem::File { name: "dialog.tsx" },
                        FileTreeItem::File { name: "input.tsx" },
                        FileTreeItem::File { name: "select.tsx" },
                    ],
                },
                FileTreeItem::File { name: "login-form.tsx" },
                FileTreeItem::File { name: "register-form.tsx" },
            ],
        },
        FileTreeItem::Folder {
            name: "lib",
            items: vec![
                FileTreeItem::File { name: "utils.ts" },
                FileTreeItem::File { name: "cn.ts" },
                FileTreeItem::File { name: "api.ts" },
            ],
        },
        FileTreeItem::Folder {
            name: "hooks",
            items: vec![
                FileTreeItem::File { name: "use-media-query.ts" },
                FileTreeItem::File { name: "use-debounce.ts" },
                FileTreeItem::File { name: "use-local-storage.ts" },
            ],
        },
        FileTreeItem::File { name: "app.tsx" },
        FileTreeItem::File { name: "layout.tsx" },
        FileTreeItem::File { name: "globals.css" },
        FileTreeItem::File { name: "package.json" },
    ];

    rsx! {
        Card { class: "gap-2 mx-auto w-full max-w-[16rem]",
            CardHeader {
                Tabs { default_value: "explorer",
                    TabsList { class: "w-full",
                        TabsTrigger { value: "explorer", "Explorer" }
                        TabsTrigger { value: "outline", "Outline" }
                    }
                }
            }
            CardContent {
                div { class: "flex flex-col gap-1",
                    for item in file_tree {
                        FileTreeNode { item: item }
                    }
                }
            }
        }
    }
}
