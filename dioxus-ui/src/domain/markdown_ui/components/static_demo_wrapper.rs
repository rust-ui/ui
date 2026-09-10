use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::document::eval;
use dioxus::prelude::*;
use icons::{Check, Code, Copy, EllipsisVertical, Eye, Terminal};
use registry::hooks::use_copy_clipboard::use_copy_clipboard;
use registry::ui::button::{Button, ButtonSize, ButtonVariant};
use registry::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuAlign, DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem, DropdownMenuLink,
    DropdownMenuTrigger,
};
use registry::ui::separator::Separator;
use tw_merge::tw_merge;

use crate::__registry__::static_md_registry::{MarkdownType, get_static_registry_entry};

static DEMO_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Copy, PartialEq)]
enum DemoTab {
    Preview,
    Code,
}

/// Transform code for display by replacing internal registry paths with user-facing component paths
fn transform_code_for_display(code: &str) -> String {
    code.replace("use crate::registry::", "use crate::components::")
}

#[component]
pub fn StaticDemoWrapper(
    demo_type: MarkdownType,
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let Some(demo_data) = get_static_registry_entry(demo_type) else {
        return rsx! {
            p { "Demo not found in static registry" }
        };
    };

    let raw_code: &'static str = demo_data.raw_code;
    let demo_name: &'static str = demo_data.demo_name;
    let transformed_code = transform_code_for_display(raw_code);
    let highlighted = crate::markdown::highlight_code::highlight_code(&transformed_code, Some("rust"), None);

    let cli_command = format!("ui add {demo_name}");
    let view_md_href = format!("/registry/styles/default/{demo_name}.md");

    // Separate copy signals for CLI command and demo code (mirrors leptos)
    let (copy_cli, copied_cli) = use_copy_clipboard(None);
    let (copy_demo, copied_demo) = use_copy_clipboard(None);

    let preview_classes = tw_merge!(
        "flex items-center justify-center flex-[1_1_auto] min-w-[150px] min-h-[370px] bg-background p-4",
        class.as_deref().unwrap_or("")
    );
    let id = use_hook(|| DEMO_COUNTER.fetch_add(1, Ordering::Relaxed));
    let container_id = format!("demo-content-{id}");
    let handle_id = format!("demo-handle-{id}");
    let bg_id = format!("demo-bg-{id}");

    let mut tab = use_signal(|| DemoTab::Preview);

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

    let cli_for_click = cli_command.clone();

    rsx! {
        div { class: "flex flex-col gap-2 w-full",
            div { class: "flex justify-between items-center",
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

                div { class: "flex gap-2 items-center",
                    Button {
                        class: "hidden sm:flex",
                        variant: ButtonVariant::Outline,
                        size: ButtonSize::Sm,
                        title: "Copy the command line",
                        onclick: move |_| copy_cli(&cli_for_click),
                        if *copied_cli.read() {
                            Check {}
                        } else {
                            Terminal {}
                        }
                        span { "{cli_command}" }
                    }

                    DropdownMenu { align: DropdownMenuAlign::End,
                        DropdownMenuTrigger { class: "px-2 h-8",
                            EllipsisVertical {}
                        }
                        DropdownMenuContent {
                            DropdownMenuGroup {
                                DropdownMenuItem {
                                    onclick: move |_| copy_demo(raw_code),
                                    if *copied_demo.read() {
                                        Check {}
                                    } else {
                                        Copy {}
                                    }
                                    span { "Copy Demo" }
                                }
                            }
                            Separator { class: "my-1" }
                            DropdownMenuGroup {
                                DropdownMenuItem {
                                    DropdownMenuLink {
                                        href: view_md_href,
                                        target: "_blank",
                                        rel: "noopener noreferrer",
                                        "View as Markdown"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            div { style: "{preview_display()}",
                div { class: "border rounded-xl flex flex-row md:touch-none w-full",
                    div {
                        id: "{container_id}",
                        "data-name": "Preview",
                        class: "{preview_classes}",
                        {children}
                    }
                    div {
                        id: "{handle_id}",
                        class: "hidden md:flex relative justify-center items-center w-3 -mr-2 translate-x-2 bg-transparent cursor-col-resize select-none touch-none z-10",
                        div { class: "h-8 w-1.5 rounded-full bg-neutral-300 dark:bg-neutral-600 transition-all" }
                    }
                    div {
                        id: "{bg_id}",
                        class: "flex-[0_0_auto] bg-muted",
                        style: "width: 0px; background-image: radial-gradient(circle, color-mix(in srgb, currentColor 25%, transparent) 1px, transparent 1px); background-size: 20px 20px; background-attachment: fixed;",
                    }
                }
            }

            div { style: "{code_display()}",
                div { class: "group/scrollbar-on-hover",
                    pre {
                        "data-name": "__SyntectHighlighterCode",
                        class: "scrollbar__on_hover h-full max-h-[370px] overflow-y-auto whitespace-pre-wrap p-4 [&_span]:text-xs rounded-md bg-muted",
                        code { dangerous_inner_html: "{highlighted}" }
                    }
                }
            }
        }
    }
}
