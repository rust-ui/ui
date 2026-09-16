use dioxus::prelude::*;
use registry::ui::editor::Editor;

#[component]
pub fn DemoEditor() -> Element {
    let mut html = use_signal(|| {
        "<h2>Write something</h2><p>Select text, then use toolbar. Changes appear below.</p>".to_string()
    });

    rsx! {
        div { class: "flex flex-col gap-3",
            Editor {
                initial_html: html(),
                placeholder: "Write here...",
                on_change: move |next| html.set(next),
            }
            div { class: "rounded-md border bg-muted/30 p-3",
                p { class: "mb-2 text-xs font-medium text-muted-foreground", "HTML output" }
                pre { class: "max-h-48 overflow-auto whitespace-pre-wrap break-words text-xs", "{html}" }
            }
        }
    }
}
