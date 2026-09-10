use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::document::eval;
use dioxus::prelude::*;
use icons::{Code, Eye};
use tw_merge::tw_merge;

use crate::__registry__::source_map::get_demo_source;

static DEMO_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Copy, PartialEq)]
enum DemoTab {
    Preview,
    Code,
}

#[component]
pub fn DemoWrapper(
    #[props(into, optional)] demo_name: Option<String>,
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let preview_classes = tw_merge!(
        "flex items-center justify-center flex-[1_1_auto] min-w-[150px] min-h-[370px] bg-background p-4",
        class.as_deref().unwrap_or("")
    );
    let id = use_hook(|| DEMO_COUNTER.fetch_add(1, Ordering::Relaxed));
    let container_id = format!("demo-content-{id}");
    let handle_id = format!("demo-handle-{id}");
    let bg_id = format!("demo-bg-{id}");

    let mut tab = use_signal(|| DemoTab::Preview);

    // Client-side nav reuses this component instance, so `tab` survives route
    // changes. Reset to Preview during render when the demo changes (no effect).
    let mut prev_demo = use_signal(|| demo_name.clone());
    if prev_demo() != demo_name {
        prev_demo.set(demo_name.clone());
        tab.set(DemoTab::Preview);
    }

    // Drag handle: grows the bg div (starts at w-0) leftward, content shrinks naturally (flex-[1_1_auto])
    let hid = handle_id.clone();
    let bgid = bg_id.clone();
    use_effect(move || {
        let js = format!(
            r#"
            (function() {{
                const handle = document.getElementById('{hid}');
                const bg = document.getElementById('{bgid}');
                if (!handle || !bg) return;

                let startX, startBgW;

                handle.addEventListener('mousedown', function(e) {{
                    startX = e.clientX;
                    startBgW = bg.offsetWidth;
                    document.addEventListener('mousemove', onMove);
                    document.addEventListener('mouseup', onUp);
                    e.preventDefault();
                }});

                function onMove(e) {{
                    const dx = e.clientX - startX;
                    // Dragging left (negative dx) grows bg, dragging right shrinks it
                    const newBgW = Math.max(0, startBgW - dx);
                    bg.style.width = newBgW + 'px';
                }}

                function onUp() {{
                    document.removeEventListener('mousemove', onMove);
                    document.removeEventListener('mouseup', onUp);
                }}
            }})();
            "#
        );
        spawn(async move {
            let _ = eval(&js).await;
        });
    });

    let tab_base = "inline-flex items-center gap-1.5 rounded-sm px-3 py-1.5 text-sm font-medium transition-colors";
    let active = "bg-background text-foreground shadow-sm";
    let inactive = "text-muted-foreground hover:text-foreground";

    let preview_cls = move || format!("{tab_base} {}", if tab() == DemoTab::Preview { active } else { inactive });
    let code_cls = move || format!("{tab_base} {}", if tab() == DemoTab::Code { active } else { inactive });
    let preview_display = move || {
        if tab() == DemoTab::Preview { "display:block" } else { "display:none" }
    };
    let code_display = move || {
        if tab() == DemoTab::Code { "display:block" } else { "display:none" }
    };

    rsx! {
        div { class: "flex flex-col gap-2 w-full",

            // Toolbar — self-start prevents flex-col from stretching it full width
            div { class: "self-start inline-flex h-9 items-center rounded-md bg-muted p-1 text-muted-foreground",
                button {
                    class: "{preview_cls()}",
                    onclick: move |_| tab.set(DemoTab::Preview),
                    Eye { class: "size-3.5" }
                    "Preview"
                }
                button {
                    class: "{code_cls()}",
                    onclick: move |_| tab.set(DemoTab::Code),
                    Code { class: "size-3.5" }
                    "Code"
                }
            }

            // Preview panel
            div { style: "{preview_display()}",
                // No overflow-hidden — allows handle to translate outside the border (matches RUST-UI)
                div { class: "border rounded-xl flex flex-row md:touch-none w-full",
                    // Content area: flex-[1_1_auto] — takes all space, shrinks when bg grows
                    div {
                        id: "{container_id}",
                        "data-name": "Preview",
                        class: "{preview_classes}",
                        {children}
                    }
                    // Drag handle — visual pill, mousedown triggers JS resize
                    div {
                        id: "{handle_id}",
                        class: "hidden md:flex relative justify-center items-center w-3 -mr-2 translate-x-2 bg-transparent cursor-col-resize select-none touch-none z-10",
                        div { class: "h-8 w-1.5 rounded-full bg-neutral-300 dark:bg-neutral-600 transition-all" }
                    }
                    // Background: starts at w-0, grows when handle is dragged left
                    div {
                        id: "{bg_id}",
                        class: "flex-[0_0_auto] bg-muted",
                        style: "width: 0px; background-image: radial-gradient(circle, color-mix(in srgb, currentColor 25%, transparent) 1px, transparent 1px); background-size: 20px 20px; background-attachment: fixed;",
                    }
                }
            }

            // Code panel
            div { style: "{code_display()}",
                {
                    let source = demo_name.as_deref().and_then(get_demo_source);
                    if let Some(code) = source {
                        let code = code.to_string();
                        let highlighted = crate::markdown::highlight_code::highlight_code(&code, Some("rust"), None);
                        let copy_id = format!("copy-btn-{id}");
                        let cid = copy_id.clone();
                        rsx! {
                            div { class: "relative rounded-xl border bg-muted overflow-hidden",
                                // Copy button
                                button {
                                    id: "{copy_id}",
                                    class: "absolute top-3 right-3 z-10 inline-flex items-center gap-1.5 rounded-md border bg-background px-2.5 py-1.5 text-xs font-medium text-foreground shadow-xs hover:bg-muted transition-colors",
                                    onclick: move |_| {
                                        let code = code.clone();
                                        let cid = cid.clone();
                                        spawn(async move {
                                            let js = format!(
                                                r#"navigator.clipboard.writeText({code:?}).then(() => {{
                                                                                                                    const btn = document.getElementById('{cid}');
                                                                                                                    if (btn) {{ btn.textContent = 'Copied!'; setTimeout(() => btn.textContent = 'Copy', 1500); }}
                                                                                                                }})"#,
                                            );
                                            let _ = eval(&js).await;
                                        });
                                    },
                                    "Copy"
                                }
                                pre { class: "overflow-x-auto py-3.5 px-4 text-xs min-h-[370px] font-mono",
                                    dangerous_inner_html: "{highlighted}",
                                }
                            }
                        }
                    } else {
                        rsx! {
                            div { class: "rounded-xl border bg-muted flex items-center justify-center min-h-[370px]",
                                p { class: "text-sm text-muted-foreground", "Source not available." }
                            }
                        }
                    }
                }
            }
        }
    }
}
