use icons::{ChevronRight, File as FileIcon, Folder as FolderIcon};
use leptos::prelude::*;
use leptos_ui::clx;
use markdown_crate::highlight_code::highlight_code;
use registry::hooks::use_random::use_random_id;

pub const TARGET_FILE_RENDERER: &str = "target___FileRenderer";
pub const TARGET_FILE_RENDERER_HIGHLIGHT: &str = "target___FileRenderer__Highlight";
pub const SYNTECT_HIGHLIGHTER_CODE: &str = "__SyntectHighlighterCode";

mod components {
    use super::*;
    clx! {Tree, div, "rounded-md border not-prose bg-card w-[240px] border-border"}
    clx! {FolderTrigger, label, "flex flex-row gap-2 items-center py-1.5 px-2 w-full text-sm rounded-md cursor-pointer [&_svg:not([class*='size-'])]:size-4 hover:bg-accent hover:text-accent-foreground"}
    clx! {FileTrigger, label, "flex flex-row gap-2 items-center py-1.5 px-2 ml-3 w-full text-sm rounded-md cursor-pointer focus:outline-none [>_svg]:size-4 peer-checked:bg-accent peer-checked:font-medium hover:peer-checked:bg-accent hover:bg-accent hover:text-accent-foreground [&_svg:not([class*='size-'])]:size-4"}
    clx! {FolderContent, div, "grid overflow-hidden transition-all duration-400 grid-rows-[0fr] peer-checked:grid-rows-[1fr]"}
    clx! {FileList, ul, "flex flex-col pl-2 ml-6 relative before:content-[''] before:absolute before:-left-2 before:top-0 before:bottom-0 before:border-l before:border-muted-foreground/30 min-h-[0]"}
}

pub use components::*;

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn Folder(
    #[prop(into)] name: &'static str,
    #[prop(default = false)] open: bool,
    children: Children,
) -> impl IntoView {
    let folder_id = use_random_id();

    view! {
        <div data-name="Folder" class="flex flex-col [&:has(>input:checked)>label>svg:first-child]:rotate-90">
            // <input id=folder_id.clone() type="checkbox" class="sr-only peer" checked=open />
            <input id=folder_id.clone() type="checkbox" class="peer" checked=open />

            <FolderTrigger attr:r#for=folder_id>
                <ChevronRight class="transition-transform duration-200 ease-in-out origin-center" />
                <FolderIcon />
                <span>{name}</span>
            </FolderTrigger>

            <FolderContent>
                <FileList>{children()}</FileList>
            </FolderContent>
        </div>
    }
}

#[component]
pub fn File(#[prop(into)] name: &'static str, #[prop(default = false)] checked: bool) -> impl IntoView {
    let file_id = use_random_id();

    view! {
        <li data-name="File" class="flex flex-row -ml-4">
            // <input id=file_id.clone() type="radio" name="file-selection" class="sr-only peer" checked=checked />
            <input id=file_id.clone() type="radio" name="file-selection" class="peer" checked=checked />
            <FileTrigger attr:r#for=file_id attr:tabindex="0">
                <FileIcon />
                <span>{name}</span>
            </FileTrigger>
        </li>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn FileRenderer(
    #[prop(into)] name: &'static str,
    #[prop(into)] content: String,
    #[prop(default = false)] checked: bool,
) -> impl IntoView {
    let file_id = use_random_id();

    view! {
        <li data-name="FileRenderer" class="flex flex-row -ml-4">
            <input
                id=file_id.clone()
                type="radio"
                name="file-selection"
                // sr-only
                class="peer"
                checked=checked
                on:change=move |_| {
                    let html = format!(
                        "<div><h3 class='font-semibold mb-2'>{}</h3><pre class='text-xs bg-muted p-4 rounded-md overflow-x-auto'><code>{}</code></pre></div>",
                        name,
                        content,
                    );
                    render_content_to_target(TARGET_FILE_RENDERER, html);
                }
            />

            <FileTrigger attr:r#for=file_id attr:tabindex="0">
                <FileIcon />
                <span>{name}</span>
            </FileTrigger>
        </li>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn FileRendererHighlight(
    #[prop(into)] name: &'static str,
    #[prop(into)] content: String,
    #[prop(default = false)] checked: bool,
    #[prop(optional)] language: Option<&'static str>,
) -> impl IntoView {
    let file_id = use_random_id();

    view! {
        <li data-name="FileRendererHighlight" class="flex flex-row -ml-4">
            <input
                id=file_id.clone()
                type="radio"
                name="file-selection"
                // sr-only
                class="peer"
                checked=checked
                on:change=move |_| {
                    const HIGHLIGHT_CLASS: &str = "h-full max-h-[370px] overflow-y-auto overscroll-y-contain whitespace-pre-wrap p-4 [&_span]:text-xs rounded-md bg-muted";
                    let highlighted_content = highlight_code(&content, language, Some(name));
                    let html = format!(
                        "<div><h3 class='font-semibold mb-2'>{name}</h3><pre class='{HIGHLIGHT_CLASS}' data-name='{SYNTECT_HIGHLIGHTER_CODE}'><code>{highlighted_content}</code></pre></div>",
                    );
                    render_content_to_target(TARGET_FILE_RENDERER_HIGHLIGHT, html);
                }
            />

            <FileTrigger attr:r#for=file_id attr:tabindex="0">
                <FileIcon />
                <span>{name}</span>
            </FileTrigger>
        </li>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

fn render_content_to_target(target_id: &str, html: String) {
    if let Some(content_div) = window().document().and_then(|doc| doc.get_element_by_id(target_id)) {
        content_div.set_inner_html(&html);
    }
}
