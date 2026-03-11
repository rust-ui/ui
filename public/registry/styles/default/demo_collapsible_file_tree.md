---
title: "Demo Collapsible File Tree"
name: "demo_collapsible_file_tree"
cargo_dependencies: []
registry_dependencies: ["card", "collapsible", "tabs"]
type: "components:demos"
path: "demos/demo_collapsible_file_tree.rs"
---

# Demo Collapsible File Tree

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_collapsible_file_tree
```

## Component Code

```rust
use icons::{ChevronRight, File, Folder};
use leptos::prelude::*;

use crate::components::ui::card::{Card, CardContent, CardHeader};
use crate::components::ui::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};
use crate::components::ui::tabs::{Tabs, TabsList, TabsTrigger};

/* ========================================================== */
/*                       DATA TYPES                            */
/* ========================================================== */

#[derive(Clone)]
pub enum FileTreeItem {
    File { name: &'static str },
    Folder { name: &'static str, items: Vec<FileTreeItem> },
}

/* ========================================================== */
/*                     FILE TREE NODE                          */
/* ========================================================== */

#[component]
fn FileTreeNode(item: FileTreeItem) -> impl IntoView {
    match item {
        FileTreeItem::File { name } => view! {
            <div class="flex gap-2 items-center py-1 px-2 w-full text-sm rounded-md transition-colors text-foreground hover:bg-accent hover:text-accent-foreground">
                <File class="size-4 shrink-0" />
                <span>{name}</span>
            </div>
        }
        .into_any(),

        FileTreeItem::Folder { name, items } => view! {
            <Collapsible>
                <CollapsibleTrigger class="flex gap-2 items-center py-1 px-2 w-full text-sm rounded-md transition-colors group hover:bg-accent hover:text-accent-foreground">
                    <ChevronRight class="transition-transform duration-200 size-4 shrink-0 group-data-[state=open]:rotate-90" />
                    <Folder class="size-4 shrink-0" />
                    {name}
                </CollapsibleTrigger>
                <CollapsibleContent class="flex flex-col gap-1 mt-1 ml-5">
                    <For
                        each=move || items.clone()
                        key=|item| match item {
                            FileTreeItem::File { name } => *name,
                            FileTreeItem::Folder { name, .. } => *name,
                        }
                        children=move |child| view! { <FileTreeNode item=child /> }
                    />
                </CollapsibleContent>
            </Collapsible>
        }
        .into_any(),
    }
}

/* ========================================================== */
/*                          DEMO                               */
/* ========================================================== */

#[component]
pub fn DemoCollapsibleFileTree() -> impl IntoView {
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

    view! {
        <Card class="gap-2 mx-auto w-full max-w-[16rem]">
            <CardHeader>
                <Tabs default_value="explorer">
                    <TabsList class="w-full">
                        <TabsTrigger value="explorer">"Explorer"</TabsTrigger>
                        <TabsTrigger value="outline">"Outline"</TabsTrigger>
                    </TabsList>
                </Tabs>
            </CardHeader>
            <CardContent>
                <div class="flex flex-col gap-1">
                    <For
                        each=move || file_tree.clone()
                        key=|item| match item {
                            FileTreeItem::File { name } => *name,
                            FileTreeItem::Folder { name, .. } => *name,
                        }
                        children=move |item| view! { <FileTreeNode item=item /> }
                    />
                </div>
            </CardContent>
        </Card>
    }
}
```
