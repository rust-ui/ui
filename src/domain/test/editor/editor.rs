use dioxus::prelude::*;
use icons::{Bold, Code, Heading1, Heading2, Italic, List, ListOrdered, Strikethrough, Underline};
use registry::ui::toolbar::{ToolbarButton, ToolbarSeparator, ToolbarToggleGroup, ToolbarToggleItem};
use tw_merge::tw_merge;

use crate::domain::test::editor::use_editor::{EditorHandle, FormatAction, use_editor};

#[component]
pub fn Editor(
    #[props(default)] initial_html: String,
    #[props(into, optional)] placeholder: Option<String>,
    #[props(into, optional)] class: Option<String>,
    #[props(default = false)] disabled: bool,
    #[props(optional)] on_change: Option<EventHandler<String>>,
) -> Element {
    let handle = use_editor(&initial_html);
    use_context_provider(|| handle.clone());

    rsx! {
        div {
            class: tw_merge!(
                "flex w-full flex-col overflow-hidden rounded-md border bg-background shadow-sm",
                class.as_deref().unwrap_or("")
            ),
            EditorToolbar { disabled }
            EditorContent {
                handle,
                initial_html,
                placeholder,
                disabled,
                on_change,
            }
        }
    }
}

#[component]
pub fn EditorContent(
    handle: EditorHandle,
    initial_html: String,
    #[props(into, optional)] placeholder: Option<String>,
    #[props(default = false)] disabled: bool,
    #[props(optional)] on_change: Option<EventHandler<String>>,
) -> Element {
    let placeholder = placeholder.unwrap_or_default();
    let initial_html = use_hook(|| initial_html);
    let html = (handle.html)();
    // `handle.html` is also re-set on selection-only DOM syncs (cursor moves,
    // no content change), which would otherwise re-fire `on_change` with an
    // identical value on every caret move. Skip when the value didn't change.
    let mut last_sent = use_signal(|| None::<String>);
    use_effect(move || {
        if let Some(on_change) = &on_change
            && last_sent.peek().as_deref() != Some(html.as_str())
        {
            last_sent.set(Some(html.clone()));
            on_change.call(html.clone());
        }
    });
    rsx! {
        div {
            id: "{handle.id}",
            contenteditable: (!disabled).to_string(),
            spellcheck: "true",
            role: "textbox",
            aria_multiline: "true",
            "data-placeholder": placeholder,
            class: "min-h-48 w-full p-4 text-sm leading-7 outline-none empty:before:pointer-events-none empty:before:text-muted-foreground empty:before:content-[attr(data-placeholder)] [&_h1]:mt-4 [&_h1]:mb-2 [&_h1]:text-2xl [&_h1]:font-bold [&_h1]:first:mt-0 [&_h2]:mt-4 [&_h2]:mb-2 [&_h2]:text-xl [&_h2]:font-semibold [&_h3]:mt-3 [&_h3]:mb-2 [&_h3]:text-lg [&_h3]:font-semibold [&_p]:mb-3 [&_ul]:mb-3 [&_ul]:list-disc [&_ul]:pl-6 [&_ol]:mb-3 [&_ol]:list-decimal [&_ol]:pl-6 [&_li]:mb-1 [&_a]:text-primary [&_a]:underline [&_a]:underline-offset-2 [&_blockquote]:border-l-2 [&_blockquote]:pl-3 [&_blockquote]:text-muted-foreground [&_pre]:mb-3 [&_pre]:overflow-x-auto [&_pre]:rounded-md [&_pre]:bg-muted [&_pre]:p-3 [&_pre]:text-xs [&_code]:font-mono [&_pre_code]:font-mono [&_img]:mb-3 [&_img]:max-w-full [&_img]:rounded-md",
            dangerous_inner_html: "{initial_html}",
            onmounted: move |_event: dioxus::prelude::MountedEvent| {
                #[cfg(target_arch = "wasm32")]
                if let Some(el) = _event.data().downcast::<web_sys::Element>() {
                    let mut element = handle.element;
                    element.set(Some(el.clone()));
                }
            },
        }
    }
}

#[component]
pub fn EditorToolbar(#[props(default = false)] disabled: bool) -> Element {
    let editor = use_context::<EditorHandle>();
    let state = (editor.state)();
    let execute = |action| {
        let editor = editor.clone();
        move |_| editor.execute(action)
    };
    rsx! {
        div {
            class: "flex flex-wrap items-center gap-1 border-b bg-muted/30 p-1",
            role: "toolbar",
            aria_label: "Text formatting",
            ToolbarToggleGroup {
                ToolbarToggleItem {
                    title: "Bold",
                    pressed: state.bold,
                    disabled,
                    onclick: execute(FormatAction::Bold),
                    Bold {}
                }
                ToolbarToggleItem {
                    title: "Italic",
                    pressed: state.italic,
                    disabled,
                    onclick: execute(FormatAction::Italic),
                    Italic {}
                }
                ToolbarToggleItem {
                    title: "Underline",
                    pressed: state.underline,
                    disabled,
                    onclick: execute(FormatAction::Underline),
                    Underline {}
                }
                ToolbarToggleItem {
                    title: "Strikethrough",
                    pressed: state.strikethrough,
                    disabled,
                    onclick: execute(FormatAction::Strikethrough),
                    Strikethrough {}
                }
            }
            ToolbarSeparator {}
            ToolbarButton { disabled, onclick: execute(FormatAction::Heading(1)),
                Heading1 {}
                "H1"
            }
            ToolbarButton { disabled, onclick: execute(FormatAction::Heading(2)),
                Heading2 {}
                "H2"
            }
            ToolbarSeparator {}
            ToolbarToggleGroup {
                ToolbarToggleItem {
                    title: "Bullet list",
                    pressed: state.bullet_list,
                    disabled,
                    onclick: execute(FormatAction::BulletList),
                    List {}
                }
                ToolbarToggleItem {
                    title: "Ordered list",
                    pressed: state.ordered_list,
                    disabled,
                    onclick: execute(FormatAction::OrderedList),
                    ListOrdered {}
                }
            }
            ToolbarSeparator {}
            ToolbarButton { disabled, onclick: execute(FormatAction::Code),
                Code {}
                "Code"
            }
            ToolbarButton { disabled, onclick: execute(FormatAction::ClearFormatting), "Clear" }
        }
    }
}
