//! `EditorHandle` state/DOM-bridge logic, extracted out of `editor.rs` so the
//! components stay presentation-only.
//!
//! V1 uses the browser's `contenteditable` and `execCommand` APIs. The DOM
//! bridge is deliberately kept behind `EditorHandle` so toolbar components do
//! not depend on browser implementation details.

use dioxus::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::closure::Closure;

static NEXT_EDITOR_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
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

#[derive(Clone, PartialEq)]
pub struct EditorHandle {
    pub id: String,
    pub state: Signal<EditorState>,
    pub html: Signal<String>,
    /// The live contenteditable root, captured via `onmounted`. Looking this
    /// up by `id` through `document.get_element_by_id` instead is unreliable
    /// after SSR hydration: `NEXT_EDITOR_ID` is a process-local counter, so the
    /// id baked into the server-rendered HTML can differ from the id this
    /// handle computes when the component re-runs on the client, and the
    /// lookup then silently finds nothing.
    #[cfg(target_arch = "wasm32")]
    pub element: Signal<Option<web_sys::Element>>,
}

impl EditorHandle {
    fn new(initial_html: &str) -> Self {
        let id = format!("rust-ui-editor-{}", NEXT_EDITOR_ID.fetch_add(1, Ordering::Relaxed));
        Self {
            id,
            state: Signal::new(EditorState::default()),
            html: Signal::new(initial_html.to_string()),
            #[cfg(target_arch = "wasm32")]
            element: Signal::new(None),
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn execute(&self, action: FormatAction) {
        let Some(document) = web_sys::window().and_then(|w| w.document()) else {
            return;
        };
        let Some(root) = (*self.element.read()).clone() else {
            return;
        };
        let html_document: &web_sys::HtmlDocument = document.unchecked_ref();
        let _ = match action {
            FormatAction::Bold => html_document.exec_command("bold"),
            FormatAction::Italic => html_document.exec_command("italic"),
            FormatAction::Underline => html_document.exec_command("underline"),
            FormatAction::Strikethrough => html_document.exec_command("strikeThrough"),
            FormatAction::OrderedList => html_document.exec_command("insertOrderedList"),
            FormatAction::BulletList => html_document.exec_command("insertUnorderedList"),
            FormatAction::ClearFormatting => html_document.exec_command("removeFormat"),
            FormatAction::Code => html_document.exec_command_with_show_ui_and_value("formatBlock", false, "pre"),
            FormatAction::Heading(level) => {
                html_document.exec_command_with_show_ui_and_value("formatBlock", false, &format!("h{level}"))
            }
        };
        self.sync_from_dom(&document, &root);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[allow(clippy::missing_const_for_fn)]
    pub fn execute(&self, _action: FormatAction) {}

    #[cfg(target_arch = "wasm32")]
    fn sync_from_dom(&self, document: &web_sys::Document, root: &web_sys::Element) {
        sanitize_element(root);
        let html_document: &web_sys::HtmlDocument = document.unchecked_ref();
        let block = html_document
            .query_command_value("formatBlock")
            .unwrap_or_default()
            .to_lowercase();
        let heading = if block.len() == 2 {
            block.strip_prefix('h').and_then(|n| n.parse::<u8>().ok()).unwrap_or(0)
        } else {
            0
        };
        let state = EditorState {
            bold: html_document.query_command_state("bold").unwrap_or(false),
            italic: html_document.query_command_state("italic").unwrap_or(false),
            underline: html_document.query_command_state("underline").unwrap_or(false),
            strikethrough: html_document.query_command_state("strikeThrough").unwrap_or(false),
            ordered_list: html_document.query_command_state("insertOrderedList").unwrap_or(false),
            bullet_list: html_document
                .query_command_state("insertUnorderedList")
                .unwrap_or(false),
            heading,
        };
        let mut state_signal = self.state;
        let mut html_signal = self.html;
        state_signal.set(state);
        html_signal.set(root.inner_html());
    }
}

#[cfg(target_arch = "wasm32")]
const ALLOWED_TAGS: &[&str] = &[
    "P",
    "BR",
    "STRONG",
    "B",
    "EM",
    "I",
    "U",
    "S",
    "DEL",
    "CODE",
    "H1",
    "H2",
    "H3",
    "H4",
    "H5",
    "H6",
    "UL",
    "OL",
    "LI",
    "BLOCKQUOTE",
    "PRE",
    "HR",
    "A",
    "IMG",
];
#[cfg(target_arch = "wasm32")]
const ALLOWED_ATTRS: &[&str] = &["href", "target", "rel", "src", "alt", "title"];

/// Mirrors the previous JS `sanitize()`: one shallow unwrap pass per disallowed
/// element (promoted children are not re-visited in the same pass), operating
/// directly on the live contenteditable DOM instead of a detached template.
#[cfg(target_arch = "wasm32")]
fn sanitize_element(root: &web_sys::Element) {
    let list = root.child_nodes();
    let mut nodes = Vec::with_capacity(list.length() as usize);
    for idx in 0..list.length() {
        if let Some(node) = list.item(idx) {
            nodes.push(node);
        }
    }
    for node in nodes {
        let Some(el) = node.dyn_ref::<web_sys::Element>() else {
            continue;
        };
        if !ALLOWED_TAGS.contains(&el.tag_name().as_str()) {
            unwrap_element(el);
            continue;
        }
        sanitize_attrs(el);
        if let Some(href) = el.get_attribute("href") {
            set_or_remove_attr(el, "href", safe_url(&href));
        }
        if let Some(src) = el.get_attribute("src") {
            set_or_remove_attr(el, "src", safe_url(&src));
        }
        sanitize_element(el);
    }
}

#[cfg(target_arch = "wasm32")]
fn unwrap_element(el: &web_sys::Element) {
    let Some(parent) = el.parent_node() else {
        return;
    };
    while let Some(child) = el.first_child() {
        let _ = parent.insert_before(&child, Some(el.as_ref()));
    }
    let _ = parent.remove_child(el);
}

#[cfg(target_arch = "wasm32")]
fn sanitize_attrs(el: &web_sys::Element) {
    let attrs = el.attributes();
    let mut to_remove = Vec::new();
    for idx in 0..attrs.length() {
        if let Some(attr) = attrs.item(idx) {
            let name = attr.name();
            let lower = name.to_lowercase();
            if !ALLOWED_ATTRS.contains(&lower.as_str()) || lower.starts_with("on") {
                to_remove.push(name);
            }
        }
    }
    for name in to_remove {
        let _ = el.remove_attribute(&name);
    }
}

#[cfg(target_arch = "wasm32")]
fn set_or_remove_attr(el: &web_sys::Element, attr: &str, value: String) {
    if value.is_empty() {
        let _ = el.remove_attribute(attr);
    } else {
        let _ = el.set_attribute(attr, &value);
    }
}

#[cfg(target_arch = "wasm32")]
fn safe_url(value: &str) -> String {
    let raw = value.trim();
    if raw.is_empty() {
        return String::new();
    }
    let lower = raw.to_lowercase();
    if lower.starts_with("javascript:") || lower.starts_with("vbscript:") || lower.starts_with("file:") {
        return String::new();
    }
    if lower.starts_with("data:") && !lower.starts_with("data:image/") {
        return String::new();
    }
    raw.to_string()
}

#[must_use]
pub fn use_editor(initial_html: &str) -> EditorHandle {
    let handle = use_hook(|| EditorHandle::new(initial_html));
    // The root's initial content is already server-rendered via
    // `dangerous_inner_html` (from the Rust markdown->HTML conversion), so
    // mount only wires up native `input`/`selectionchange` listeners here.
    // Listeners outlive the component for the app's lifetime (`Closure::forget`);
    // acceptable for this demo since the editor is mounted once per page.
    #[cfg(target_arch = "wasm32")]
    use_effect({
        let handle = handle.clone();
        move || {
            // Reactive read: reruns once `onmounted` (in `EditorContent`) sets
            // `handle.element`, which is the only reliable way to get the root
            // after hydration (see the doc comment on `EditorHandle::element`).
            let Some(root) = (*handle.element.read()).clone() else {
                return;
            };
            let Some(document) = web_sys::window().and_then(|w| w.document()) else {
                return;
            };
            handle.sync_from_dom(&document, &root);

            let input_handle = handle.clone();
            let input_root = root.clone();
            let input_closure = Closure::<dyn FnMut(web_sys::Event)>::new(move |_| {
                if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                    input_handle.sync_from_dom(&document, &input_root);
                }
            });
            let _ = root.add_event_listener_with_callback("input", input_closure.as_ref().unchecked_ref());
            input_closure.forget();

            let selection_handle = handle.clone();
            let selection_root = root.clone();
            let selection_closure = Closure::<dyn FnMut(web_sys::Event)>::new(move |_| {
                if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                    selection_handle.sync_from_dom(&document, &selection_root);
                }
            });
            let _ = document
                .add_event_listener_with_callback("selectionchange", selection_closure.as_ref().unchecked_ref());
            selection_closure.forget();
        }
    });
    handle
}
