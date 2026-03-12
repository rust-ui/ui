use icons::{Check, ChevronRight, Copy, File as FileIcon, Folder as FolderIcon};
use leptos::prelude::*;
use markdown_crate::highlight_code::highlight_code;
use registry::hooks::use_copy_clipboard::use_copy_clipboard;
use registry::ui::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};

use crate::domain::blocks::block_entry::{BlockFile, BlockFileTreeItem};

const COPY_TIMEOUT_MS: i32 = 2000;

#[component]
pub fn BlockCodePanel(files: &'static [BlockFile], tree: Vec<BlockFileTreeItem>) -> impl IntoView {
    let Some(_file) = files.first() else {
        return view! { <div /> }.into_any();
    };

    let active_idx = RwSignal::new(0_usize);

    let highlighted = Memo::new(move |_| {
        let Some(f) = files.get(active_idx.get()) else {
            return String::new();
        };
        highlight_code(f.content, Some(f.language), Some(f.name))
    });

    view! {
        <div class="overflow-hidden rounded-xl border">
            <div class="flex flex-row">
                // Left: file tree
                <div class="w-52 border-r shrink-0 bg-muted/30">
                    <BlockFileTree tree=tree active_idx=active_idx />
                </div>
                // Right: code panel
                <div class="flex flex-col flex-1 min-w-0">
                    <BlockCodeHeader files=files active_idx=active_idx />
                    <pre class="overflow-auto p-4 text-xs leading-relaxed whitespace-pre-wrap break-all max-h-[600px] bg-muted">
                        <code inner_html=move || highlighted.get() />
                    </pre>
                </div>
            </div>
        </div>
    }
    .into_any()
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
fn BlockCodeHeader(files: &'static [BlockFile], active_idx: RwSignal<usize>) -> impl IntoView {
    let (copy_fn, copied) = use_copy_clipboard(Some(COPY_TIMEOUT_MS));

    view! {
        <div class="flex gap-2 items-center px-4 h-10 border-b bg-card">
            <span class="font-mono text-xs text-muted-foreground">
                {move || files.get(active_idx.get()).map(|f| f.target).unwrap_or_default()}
            </span>
            <div class="ml-auto">
                <button
                    class="inline-flex justify-center items-center rounded-md transition-colors size-7 hover:bg-accent"
                    on:click=move |_| copy_fn(files.get(active_idx.get()).map(|f| f.content).unwrap_or_default())
                    title="Copy code"
                >
                    {move || {
                        if copied.get() {
                            view! { <Check class="size-3.5" /> }.into_any()
                        } else {
                            view! { <Copy class="size-3.5" /> }.into_any()
                        }
                    }}
                </button>
            </div>
        </div>
    }
}

#[component]
fn BlockFileTree(tree: Vec<BlockFileTreeItem>, active_idx: RwSignal<usize>) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-0.5 p-2">
            <p class="py-1 px-2 text-xs font-medium text-muted-foreground">"Files"</p>
            <For
                each=move || tree.clone()
                key=|item| item.name()
                children=move |item| {
                    view! { <BlockFileTreeNode item=item active_idx=active_idx depth=0 /> }
                }
            />
        </div>
    }
}

#[component]
fn BlockFileTreeNode(item: BlockFileTreeItem, active_idx: RwSignal<usize>, depth: usize) -> impl IntoView {
    let padding_left = format!("calc(0.5rem + {}px)", depth * 12);

    match item {
        BlockFileTreeItem::File { name, index } => view! {
            <button
                data-name="BlockFileTreeItem"
                class="flex gap-1.5 items-center py-1 px-2 w-full text-xs text-left rounded-md transition-colors hover:bg-accent"
                class=("bg-accent", move || active_idx.get() == index)
                class=("font-medium", move || active_idx.get() == index)
                style=format!("padding-left: {}", padding_left)
                on:click=move |_| active_idx.set(index)
            >
                <FileIcon class="size-3.5 shrink-0" />
                {name}
            </button>
        }
        .into_any(),

        BlockFileTreeItem::Folder { name, items } => view! {
            <Collapsible default_open=true>
                <CollapsibleTrigger class="flex gap-1.5 items-center py-1 px-2 w-full text-xs rounded-md group hover:bg-accent">
                    <ChevronRight class="transition-transform duration-200 size-3.5 shrink-0 group-data-[state=open]:rotate-90" />
                    <FolderIcon class="size-3.5 shrink-0" />
                    {name}
                </CollapsibleTrigger>
                <CollapsibleContent>
                    <For
                        each=move || items.clone()
                        key=|item| item.name()
                        children=move |child| {
                            view! { <BlockFileTreeNode item=child active_idx=active_idx depth=depth + 1 /> }
                        }
                    />
                </CollapsibleContent>
            </Collapsible>
        }
        .into_any(),
    }
}
