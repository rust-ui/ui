//! Composable rich-text editor primitives.
//!
//! V1 uses the browser's `contenteditable` and `execCommand` APIs. The DOM
//! bridge is deliberately kept behind `EditorHandle` so toolbar components do
//! not depend on browser implementation details.
//!
//! Lives under `domain::test` (not the `registry` crate) while this is a
//! demo-only, work-in-progress component — see `PLAN_MINIMAL_TIPTAP_EDITOR.md`.

use dioxus::document::eval;
use dioxus::prelude::*;
use icons::{Bold, Code, Heading1, Heading2, Italic, List, ListOrdered, Strikethrough, Underline};
use registry::ui::toolbar::{ToolbarButton, ToolbarSeparator, ToolbarToggleGroup, ToolbarToggleItem};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use tw_merge::tw_merge;

static NEXT_EDITOR_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[allow(clippy::struct_excessive_bools)] // Independent toolbar state mirrors browser selection.
pub struct EditorState {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strikethrough: bool,
    pub ordered_list: bool,
    pub bullet_list: bool,
    pub heading: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormatAction {
    Bold,
    Italic,
    Underline,
    Strikethrough,
    OrderedList,
    BulletList,
    Heading(u8),
    Code,
    ClearFormatting,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FormatActionMeta {
    pub label: &'static str,
    pub value: &'static str,
}

impl FormatAction {
    #[must_use]
    pub const fn meta(self) -> FormatActionMeta {
        match self {
            Self::Bold => FormatActionMeta {
                label: "Bold",
                value: "bold",
            },
            Self::Italic => FormatActionMeta {
                label: "Italic",
                value: "italic",
            },
            Self::Underline => FormatActionMeta {
                label: "Underline",
                value: "underline",
            },
            Self::Strikethrough => FormatActionMeta {
                label: "Strikethrough",
                value: "strikethrough",
            },
            Self::OrderedList => FormatActionMeta {
                label: "Ordered list",
                value: "ordered_list",
            },
            Self::BulletList => FormatActionMeta {
                label: "Bullet list",
                value: "bullet_list",
            },
            Self::Heading(level) => match level {
                1 => FormatActionMeta {
                    label: "Heading 1",
                    value: "heading_1",
                },
                2 => FormatActionMeta {
                    label: "Heading 2",
                    value: "heading_2",
                },
                3 => FormatActionMeta {
                    label: "Heading 3",
                    value: "heading_3",
                },
                4 => FormatActionMeta {
                    label: "Heading 4",
                    value: "heading_4",
                },
                5 => FormatActionMeta {
                    label: "Heading 5",
                    value: "heading_5",
                },
                _ => FormatActionMeta {
                    label: "Heading 6",
                    value: "heading_6",
                },
            },
            Self::Code => FormatActionMeta {
                label: "Code",
                value: "code",
            },
            Self::ClearFormatting => FormatActionMeta {
                label: "Clear formatting",
                value: "clear_formatting",
            },
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
struct EditorSnapshot {
    state: EditorState,
    html: String,
}

#[derive(Clone, PartialEq, Eq)]
pub struct EditorHandle {
    pub id: String,
    pub state: Signal<EditorState>,
    pub html: Signal<String>,
}

impl EditorHandle {
    fn new(initial_html: &str) -> Self {
        let id = format!("dioxus-editor-{}", NEXT_EDITOR_ID.fetch_add(1, Ordering::Relaxed));
        Self {
            id,
            state: Signal::new(EditorState::default()),
            html: Signal::new(initial_html.to_string()),
        }
    }

    pub fn execute(&self, action: FormatAction) {
        let command = match action {
            FormatAction::Bold => "document.execCommand('bold')".to_string(),
            FormatAction::Italic => "document.execCommand('italic')".to_string(),
            FormatAction::Underline => "document.execCommand('underline')".to_string(),
            FormatAction::Strikethrough => "document.execCommand('strikeThrough')".to_string(),
            FormatAction::OrderedList => "document.execCommand('insertOrderedList')".to_string(),
            FormatAction::BulletList => "document.execCommand('insertUnorderedList')".to_string(),
            FormatAction::Code => "document.execCommand('formatBlock', false, 'code')".to_string(),
            FormatAction::ClearFormatting => "document.execCommand('removeFormat')".to_string(),
            FormatAction::Heading(level) => format!("document.execCommand('formatBlock', false, 'h{level}')"),
        };
        self.run_js(&command);
    }

    fn run_js(&self, command: &str) {
        let id = serde_json::to_string(&self.id).unwrap_or_else(|_| "\"\"".to_string());
        let script = format!(
            r"const root = document.getElementById({id});
if (!root) return;
{command};
root.dispatchEvent(new Event('input', {{ bubbles: true }}));
 return snapshot(root)",
        );
        let mut ev = eval_with_helpers(&id, &script);
        let mut state = self.state;
        let mut html = self.html;
        spawn(async move {
            if let Ok(snapshot) = ev.recv::<EditorSnapshot>().await {
                state.set(snapshot.state);
                html.set(snapshot.html);
            }
        });
    }
}

fn eval_with_helpers(id: &str, body: &str) -> dioxus::document::Eval {
    eval(&format!(
        r"const rootId = {id};
const allowedTags = new Set(['P','BR','STRONG','B','EM','I','U','S','DEL','CODE','H1','H2','H3','H4','H5','H6','UL','OL','LI','BLOCKQUOTE','PRE','HR','A','IMG']);
const allowedAttrs = new Set(['href','target','rel','src','alt','title']);
function safeUrl(value) {{
  const raw = String(value || '').trim();
  if (!raw || /^(javascript|vbscript|file):/i.test(raw)) return '';
  if (/^data:/i.test(raw) && !/^data:image\/[a-z0-9.+-]+;base64,/i.test(raw)) return '';
  return raw;
}}
function sanitize(raw) {{
  const template = document.createElement('template');
  template.innerHTML = String(raw || '');
  const walk = (node) => {{
    [...node.children].forEach((child) => {{
      if (!allowedTags.has(child.tagName)) {{ child.replaceWith(...child.childNodes); return; }}
      [...child.attributes].forEach((attr) => {{
        if (!allowedAttrs.has(attr.name.toLowerCase()) || attr.name.toLowerCase().startsWith('on')) child.removeAttribute(attr.name);
      }});
      if (child.hasAttribute('href')) {{ const value = safeUrl(child.getAttribute('href')); value ? child.setAttribute('href', value) : child.removeAttribute('href'); }}
      if (child.hasAttribute('src')) {{ const value = safeUrl(child.getAttribute('src')); value ? child.setAttribute('src', value) : child.removeAttribute('src'); }}
      walk(child);
    }});
  }};
  walk(template.content);
  return template.innerHTML;
}}
function snapshot(root) {{
  const block = document.queryCommandValue('formatBlock').toLowerCase();
  return {{ html: sanitize(root.innerHTML), state: {{
    bold: document.queryCommandState('bold'), italic: document.queryCommandState('italic'),
    underline: document.queryCommandState('underline'), strikethrough: document.queryCommandState('strikeThrough'),
    ordered_list: document.queryCommandState('insertOrderedList'), bullet_list: document.queryCommandState('insertUnorderedList'),
    heading: /^h[1-6]$/.test(block) ? Number(block.slice(1)) : 0
  }}}};
}}
{body}",
    ))
}

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
        div { class: tw_merge!("flex w-full flex-col overflow-hidden rounded-md border bg-background shadow-sm", class.as_deref().unwrap_or("")),
            EditorToolbar { disabled }
            EditorContent { handle, initial_html, placeholder, disabled, on_change }
        }
    }
}

#[must_use]
pub fn use_editor(initial_html: &str) -> EditorHandle {
    let handle = use_hook(|| EditorHandle::new(initial_html));
    let id = handle.id.clone();
    let mut state = handle.state;
    let mut html = handle.html;
    use_effect(move || {
        let id = serde_json::to_string(&id).unwrap_or_else(|_| "\"\"".to_string());
        // The root's initial content is already server-rendered via
        // `dangerous_inner_html` (from the Rust markdown->HTML conversion), so
        // we only need to wire up listeners here, not re-inject the HTML from
        // JS. Doing so raced with hydration and could leave the root empty.
        let mut ev = eval_with_helpers(
            &id,
            &format!(
                r"const waitForRoot = () => new Promise((resolve) => {{
  const find = () => {{
    const root = document.getElementById({id});
    if (root) resolve(root);
    else requestAnimationFrame(find);
  }};
  find();
}});
const root = await waitForRoot();
const send = () => dioxus.send(snapshot(root));
root.addEventListener('input', send);
document.addEventListener('selectionchange', send);
send();
await new Promise(() => {{}});"
            ),
        );
        spawn(async move {
            while let Ok(snapshot) = ev.recv::<EditorSnapshot>().await {
                state.set(snapshot.state);
                html.set(snapshot.html);
            }
        });
    });
    handle
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
    use_effect(move || {
        if let Some(on_change) = &on_change {
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
        }
    }
}

#[component]
pub fn EditorToolbar(#[props(default = false)] disabled: bool) -> Element {
    rsx! {
        div { class: "flex flex-wrap items-center gap-1 border-b bg-muted/30 p-1", role: "toolbar", aria_label: "Text formatting",
            ToolbarSection { disabled }
        }
    }
}

#[component]
pub fn ToolbarSection(#[props(default = false)] disabled: bool) -> Element {
    let editor = use_context::<EditorHandle>();
    let state = (editor.state)();
    let execute = |action| {
        let editor = editor.clone();
        move |_| editor.execute(action)
    };
    rsx! {
        ToolbarToggleGroup {
            ToolbarToggleItem { title: "Bold", pressed: state.bold, disabled, onclick: execute(FormatAction::Bold), Bold {} }
            ToolbarToggleItem { title: "Italic", pressed: state.italic, disabled, onclick: execute(FormatAction::Italic), Italic {} }
            ToolbarToggleItem { title: "Underline", pressed: state.underline, disabled, onclick: execute(FormatAction::Underline), Underline {} }
            ToolbarToggleItem { title: "Strikethrough", pressed: state.strikethrough, disabled, onclick: execute(FormatAction::Strikethrough), Strikethrough {} }
        }
        ToolbarSeparator {}
        ToolbarButton { disabled, onclick: execute(FormatAction::Heading(1)), Heading1 {} "H1" }
        ToolbarButton { disabled, onclick: execute(FormatAction::Heading(2)), Heading2 {} "H2" }
        ToolbarSeparator {}
        ToolbarToggleGroup {
            ToolbarToggleItem { title: "Bullet list", pressed: state.bullet_list, disabled, onclick: execute(FormatAction::BulletList), List {} }
            ToolbarToggleItem { title: "Ordered list", pressed: state.ordered_list, disabled, onclick: execute(FormatAction::OrderedList), ListOrdered {} }
        }
        ToolbarSeparator {}
        ToolbarButton { disabled, onclick: execute(FormatAction::Code), Code {} "Code" }
        ToolbarButton { disabled, onclick: execute(FormatAction::ClearFormatting), "Clear" }
    }
}
