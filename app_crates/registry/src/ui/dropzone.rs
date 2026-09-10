use dioxus::document::eval;
use dioxus::prelude::*;
use icons::{FileArchive, FileAudio, FileCode, FileImage, FileSpreadsheet, FileText, LayoutGrid, LayoutList};
use tw_merge::tw_merge;

// ── ViewMode ──────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq)]
pub enum ViewMode {
    List,
    Grid,
}

// ── File type ─────────────────────────────────────────────────────────────────

#[derive(Clone, PartialEq)]
pub struct DropzoneFile {
    pub name: String,
    pub size_bytes: u64,
    pub mime_type: String,
    /// Object URL for image/video preview (wasm32 only, None otherwise)
    pub preview_url: Option<String>,
}

impl DropzoneFile {
    pub fn size_display(&self) -> String {
        match self.size_bytes {
            b if b < 1_024 => format!("{b} B"),
            b if b < 1_048_576 => format!("{:.2} KB", b as f64 / 1_024.0),
            b if b < 1_073_741_824 => format!("{:.2} MB", b as f64 / 1_048_576.0),
            b => format!("{:.2} GB", b as f64 / 1_073_741_824.0),
        }
    }
}

// ── Context ───────────────────────────────────────────────────────────────────

#[derive(Clone, Copy)]
pub struct DropzoneCtx {
    pub is_dragging: Signal<bool>,
    pub files: Signal<Vec<DropzoneFile>>,
    pub view: Signal<ViewMode>,
    pub file_input_id: &'static str,
}

// ── WASM helper ───────────────────────────────────────────────────────────────

#[cfg(target_arch = "wasm32")]
fn collect_files(
    file_list: &web_sys::FileList,
    max_size_mb: Option<f64>,
    accept: &Option<Vec<String>>,
) -> Vec<DropzoneFile> {
    let mut out = Vec::new();
    for i in 0..file_list.length() {
        let Some(f) = file_list.item(i) else { continue };
        let mime = f.type_();
        if let Some(types) = accept {
            if !types.iter().any(|t| mime.starts_with(t.as_str())) {
                continue;
            }
        }
        if let Some(max) = max_size_mb {
            if f.size() as f64 > max * 1_048_576.0 {
                continue;
            }
        }
        let preview_url = if mime.starts_with("image/") || mime.starts_with("video/") {
            web_sys::Url::create_object_url_with_blob(&f).ok()
        } else {
            None
        };
        out.push(DropzoneFile { name: f.name(), size_bytes: f.size() as u64, mime_type: mime, preview_url });
    }
    out
}

// ── Dropzone (root) ───────────────────────────────────────────────────────────

#[component]
pub fn Dropzone(
    children: Element,
    #[props(optional)] max_files: Option<usize>,
    #[props(optional)] max_size_mb: Option<f64>,
    #[props(optional)] accept: Option<Vec<String>>,
) -> Element {
    let mut files = use_signal(Vec::<DropzoneFile>::new);
    let mut is_dragging = use_signal(|| false);
    let view = use_signal(|| ViewMode::List);

    use_context_provider(|| DropzoneCtx { files, is_dragging, view, file_input_id: "dz-file-input" });

    #[cfg(not(target_arch = "wasm32"))]
    return rsx! { div { {children} } };

    #[cfg(target_arch = "wasm32")]
    {
        let mut drag_count = use_signal(|| 0u32);

        let accept_for_root = accept.clone();
        let on_mounted = move |event: dioxus::prelude::MountedEvent| {
            use wasm_bindgen::JsCast;
            use wasm_bindgen::closure::Closure;

            let mounted = event.data();
            let Some(raw) = mounted.downcast::<web_sys::Element>() else {
                return;
            };
            let el = raw.clone();

            // window dragover — only to prevent browser from opening the file
            let win = web_sys::window().expect("no window");
            let on_dragover: Closure<dyn Fn(web_sys::DragEvent)> =
                Closure::new(|e: web_sys::DragEvent| e.prevent_default());
            win.add_event_listener_with_callback("dragover", on_dragover.as_ref().unchecked_ref()).ok();
            on_dragover.forget();

            let on_dragenter: Closure<dyn FnMut(web_sys::DragEvent)> = Closure::new(move |e: web_sys::DragEvent| {
                e.prevent_default();
                let count = *drag_count.read() + 1;
                drag_count.set(count);
                if count == 1 {
                    is_dragging.set(true);
                }
            });
            el.add_event_listener_with_callback("dragenter", on_dragenter.as_ref().unchecked_ref()).ok();
            on_dragenter.forget();

            let el2 = el.clone();
            let on_dragleave: Closure<dyn FnMut(web_sys::DragEvent)> = Closure::new(move |e: web_sys::DragEvent| {
                e.prevent_default();
                let count = drag_count.read().saturating_sub(1);
                drag_count.set(count);
                if count == 0 {
                    is_dragging.set(false);
                }
            });
            el2.add_event_listener_with_callback("dragleave", on_dragleave.as_ref().unchecked_ref()).ok();
            on_dragleave.forget();

            let el3 = el.clone();
            let accept_drop = accept_for_root.clone();
            let on_drop: Closure<dyn FnMut(web_sys::DragEvent)> = Closure::new(move |e: web_sys::DragEvent| {
                e.prevent_default();
                e.stop_propagation();
                drag_count.set(0);
                is_dragging.set(false);

                let Some(dt) = e.data_transfer() else { return };
                let Some(file_list) = dt.files() else { return };

                let new_files = collect_files(&file_list, max_size_mb, &accept_drop);
                let mut w = files.write();
                let remaining = max_files.map(|m| m.saturating_sub(w.len())).unwrap_or(usize::MAX);
                w.extend(new_files.into_iter().take(remaining));
            });
            el3.add_event_listener_with_callback("drop", on_drop.as_ref().unchecked_ref()).ok();
            on_drop.forget();
        };

        let accept_input = accept.clone();
        let on_mounted_input = move |event: dioxus::prelude::MountedEvent| {
            use wasm_bindgen::JsCast;
            use wasm_bindgen::closure::Closure;

            let mounted = event.data();
            let Some(raw) = mounted.downcast::<web_sys::Element>() else { return };
            let Ok(input_el) = raw.clone().dyn_into::<web_sys::HtmlInputElement>() else { return };

            let accept_for_change = accept_input.clone();
            let on_change: Closure<dyn FnMut(web_sys::Event)> = Closure::new(move |e: web_sys::Event| {
                let Some(target) = e.target() else { return };
                let Ok(input) = target.dyn_into::<web_sys::HtmlInputElement>() else { return };
                let Some(file_list) = input.files() else { return };

                let new_files = collect_files(&file_list, max_size_mb, &accept_for_change);
                let mut w = files.write();
                let remaining = max_files.map(|m| m.saturating_sub(w.len())).unwrap_or(usize::MAX);
                w.extend(new_files.into_iter().take(remaining));

                input.set_value("");
            });
            input_el.add_event_listener_with_callback("change", on_change.as_ref().unchecked_ref()).ok();
            on_change.forget();
        };

        rsx! {
            div { onmounted: on_mounted,
                input {
                    id: "dz-file-input",
                    r#type: "file",
                    multiple: true,
                    style: "display:none",
                    onmounted: on_mounted_input,
                }
                {children}
            }
        }
    }
}

// ── DropzoneOverlay ───────────────────────────────────────────────────────────

#[component]
pub fn DropzoneOverlay(#[props(into, optional)] class: Option<String>) -> Element {
    let ctx = use_context::<DropzoneCtx>();

    if !*ctx.is_dragging.read() {
        return rsx! {};
    }

    let merged = tw_merge!(
        "fixed inset-0 z-50 flex items-center justify-center backdrop-blur-sm bg-background/70 pointer-events-none",
        class.as_deref().unwrap_or("")
    );

    rsx! {
        div { class: "{merged}",
            div { class: "flex flex-col items-center gap-5 rounded-2xl border-2 border-dashed border-primary bg-background/80 px-20 py-14 shadow-2xl text-primary select-none",
                icons::Upload { class: "size-10 animate-bounce" }
                p { class: "text-xl font-semibold tracking-tight", "Drop files here" }
                p { class: "text-sm text-muted-foreground font-normal", "Release to upload" }
            }
        }
    }
}

// ── DropzoneArea ──────────────────────────────────────────────────────────────

#[component]
pub fn DropzoneArea(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let ctx = use_context::<DropzoneCtx>();
    let dragging = *ctx.is_dragging.read();
    let input_id = ctx.file_input_id;

    let base = if dragging {
        "w-full min-h-[200px] border border-dashed border-primary bg-primary/5 rounded-xl py-12 px-10 flex flex-col items-center justify-center gap-3 transition-colors cursor-pointer"
    } else {
        "w-full min-h-[200px] border border-dashed border-border/60 bg-accent/40 rounded-xl py-12 px-10 flex flex-col items-center justify-center gap-3 transition-colors cursor-pointer hover:border-border hover:bg-accent/60"
    };

    rsx! {
        div {
            class: "{tw_merge!(base, class.as_deref().unwrap_or(\"\"))}",
            onclick: move |_| { eval(&format!("document.getElementById('{}').click()", input_id)); },
            {children}
        }
    }
}

// ── DropzoneIcon ──────────────────────────────────────────────────────────────

#[component]
pub fn DropzoneIcon(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let ctx = use_context::<DropzoneCtx>();
    let dragging = *ctx.is_dragging.read();
    let anim = if dragging { "animate-bounce" } else { "" };
    let merged = tw_merge!("text-muted-foreground", anim, class.as_deref().unwrap_or(""));
    rsx! { div { class: "{merged}", {children} } }
}

// ── DropzoneLabel ─────────────────────────────────────────────────────────────

#[component]
pub fn DropzoneLabel(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("text-sm font-semibold text-foreground text-center", class.as_deref().unwrap_or(""));
    rsx! { p { class: "{merged}", {children} } }
}

// ── DropzoneHint ──────────────────────────────────────────────────────────────

#[component]
pub fn DropzoneHint(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("text-xs text-muted-foreground text-center", class.as_deref().unwrap_or(""));
    rsx! { p { class: "{merged}", {children} } }
}

// ── FileKind ──────────────────────────────────────────────────────────────────

enum FileKind {
    Image,
    Audio,
    Pdf,
    Archive,
    Code,
    Spreadsheet,
    Text,
    Other,
}

impl FileKind {
    fn from_mime(mime: &str) -> Self {
        match mime {
            m if m.starts_with("image/") => Self::Image,
            m if m.starts_with("audio/") => Self::Audio,
            "application/pdf" => Self::Pdf,
            "application/zip"
            | "application/x-tar"
            | "application/gzip"
            | "application/x-7z-compressed"
            | "application/x-rar-compressed" => Self::Archive,
            "text/javascript" | "text/typescript" | "text/x-rust" | "text/html" | "text/css" | "application/json"
            | "application/xml" => Self::Code,
            m if m.contains("spreadsheet") || m.contains("excel") || m == "text/csv" => Self::Spreadsheet,
            m if m.starts_with("text/") => Self::Text,
            _ => Self::Other,
        }
    }

    fn icon(&self) -> Element {
        self.icon_sized("size-4 text-muted-foreground")
    }

    fn icon_lg(&self) -> Element {
        self.icon_sized("size-6 text-muted-foreground")
    }

    fn icon_sized(&self, class: &'static str) -> Element {
        match self {
            Self::Image => rsx! { FileImage { class } },
            Self::Audio => rsx! { FileAudio { class } },
            Self::Pdf => rsx! { FileText { class } },
            Self::Archive => rsx! { FileArchive { class } },
            Self::Code => rsx! { FileCode { class } },
            Self::Spreadsheet => rsx! { FileSpreadsheet { class } },
            Self::Text | Self::Other => rsx! { FileText { class } },
        }
    }
}

// ── DropzoneFileList ──────────────────────────────────────────────────────────

#[component]
pub fn DropzoneFileList(#[props(into, optional)] class: Option<String>) -> Element {
    let ctx = use_context::<DropzoneCtx>();
    let files = ctx.files.read();

    if files.is_empty() {
        return rsx! {};
    }

    let merged = tw_merge!("divide-y", class.as_deref().unwrap_or(""));

    rsx! {
        div { class: "{merged}",
            for (idx, file) in files.iter().enumerate() {
                div { class: "flex items-center gap-3 py-3",
                    if let Some(url) = &file.preview_url {
                        if file.mime_type.starts_with("video/") {
                            video {
                                src: "{url}",
                                class: "size-10 rounded object-cover shrink-0 bg-muted",
                                preload: "metadata",
                            }
                        } else {
                            img {
                                src: "{url}",
                                class: "size-10 rounded object-cover shrink-0 bg-muted",
                            }
                        }
                    } else {
                        div { class: "size-10 rounded bg-muted flex items-center justify-center shrink-0 relative",
                            {FileKind::from_mime(&file.mime_type).icon()}
                            if file.mime_type == "application/pdf" {
                                span { class: "absolute -bottom-1 -right-1 text-[8px] font-bold bg-red-500 text-white rounded px-[3px] leading-tight",
                                    "PDF"
                                }
                            }
                        }
                    }
                    div { class: "flex flex-col flex-1 min-w-0",
                        span { class: "text-sm font-medium truncate", "{file.name}" }
                        span { class: "text-xs text-muted-foreground", "{file.size_display()}" }
                    }
                    button {
                        class: "shrink-0 size-5 rounded flex items-center justify-center text-muted-foreground hover:text-foreground hover:bg-accent transition-colors text-base leading-none",
                        onclick: move |_| {
                            let mut files = ctx.files;
                            files.write().remove(idx);
                        },
                        "×"
                    }
                }
            }
        }
    }
}

// ── DropzoneViewToggle ────────────────────────────────────────────────────────

#[component]
pub fn DropzoneViewToggle(#[props(into, optional)] class: Option<String>) -> Element {
    let mut ctx = use_context::<DropzoneCtx>();
    let view = *ctx.view.read();

    let btn = |active: bool| {
        if active {
            "inline-flex items-center justify-center size-7 rounded-md bg-accent text-foreground transition-colors"
        } else {
            "inline-flex items-center justify-center size-7 rounded-md text-muted-foreground hover:bg-accent hover:text-foreground transition-colors"
        }
    };

    let merged = tw_merge!("flex items-center gap-0.5", class.as_deref().unwrap_or(""));

    rsx! {
        div { class: "{merged}",
            button {
                r#type: "button",
                "aria-label": "List view",
                class: "{btn(view == ViewMode::List)}",
                onclick: move |_| ctx.view.set(ViewMode::List),
                LayoutList { class: "size-4" }
            }
            button {
                r#type: "button",
                "aria-label": "Grid view",
                class: "{btn(view == ViewMode::Grid)}",
                onclick: move |_| ctx.view.set(ViewMode::Grid),
                LayoutGrid { class: "size-4" }
            }
        }
    }
}

// ── DropzoneFileGrid ──────────────────────────────────────────────────────────

#[component]
pub fn DropzoneFileGrid(#[props(into, optional)] class: Option<String>) -> Element {
    let ctx = use_context::<DropzoneCtx>();
    let files = ctx.files.read();

    if files.is_empty() {
        return rsx! {};
    }

    let merged = tw_merge!("flex flex-wrap gap-3", class.as_deref().unwrap_or(""));

    rsx! {
        div { class: "{merged}",
            for (idx, file) in files.iter().enumerate() {
                div { class: "relative group w-32 h-32 rounded-xl overflow-hidden bg-muted shrink-0",
                    if let Some(url) = &file.preview_url {
                        if file.mime_type.starts_with("video/") {
                            video {
                                src: "{url}",
                                class: "absolute inset-0 w-full h-full object-cover",
                                preload: "metadata",
                            }
                        } else {
                            img {
                                src: "{url}",
                                class: "absolute inset-0 w-full h-full object-cover",
                            }
                        }
                    } else {
                        div { class: "absolute inset-0 flex flex-col items-center justify-center gap-2",
                            div { class: "size-10 rounded-lg bg-background/60 flex items-center justify-center",
                                {FileKind::from_mime(&file.mime_type).icon_lg()}
                            }
                            if file.mime_type == "application/pdf" {
                                span { class: "text-[9px] font-bold bg-red-500 text-white rounded px-1.5 py-0.5 leading-tight",
                                    "PDF"
                                }
                            }
                        }
                    }
                    div { class: "absolute inset-x-0 bottom-0 bg-gradient-to-t from-black/70 to-transparent pt-6 pb-2 px-2 translate-y-full group-hover:translate-y-0 transition-transform duration-200",
                        p { class: "text-xs font-medium text-white truncate", "{file.name}" }
                        p { class: "text-[10px] text-white/70", "{file.size_display()}" }
                    }
                    button {
                        class: "absolute top-1.5 right-1.5 size-5 rounded-full bg-destructive text-destructive-foreground flex items-center justify-center text-xs font-bold opacity-0 group-hover:opacity-100 transition-opacity duration-200 leading-none",
                        onclick: move |_| {
                            let mut files = ctx.files;
                            files.write().remove(idx);
                        },
                        "×"
                    }
                }
            }
        }
    }
}
