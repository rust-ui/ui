use app_config::SeoMeta;
use app_domain::icons::all_icons::ALL_ICONS;
use dioxus::document::eval;
use dioxus::prelude::*;
use icons::RotateCw;
use registry::hooks::use_grid_virtual_scroll::use_grid_virtual_scroll;
use registry::ui::card::{Card, CardContent, CardHeader, CardTitle};
use registry::ui::drawer::{Drawer, DrawerBody, DrawerClose, DrawerContent, DrawerHandle, DrawerTitle, DrawerTrigger};
use registry::ui::input::{Input, InputType};

use crate::components::navigation::header_docs::HeaderDocs;

type IconFn = fn(&str) -> Element;

/// Cell size (px) of one icon button, gap included: button is `size-16`
/// (64px, fixed regardless of icon display size) plus the grid's `gap-2` (8px).
///
/// ~1500 icons rendered at once made the grid slow to build/diff on
/// iOS's native webview (VDOM cost, not paint cost, so CSS
/// content-visibility alone didn't fix it), so it's virtual-scrolled instead,
/// same `use_grid_virtual_scroll` hook the data grid demo uses.
const ICON_ITEM_SIZE: usize = 72;

#[component]
pub fn PageIcons() -> Element {
    const SIZE_ID: &str = "icon-size-select";

    let mut search_text = use_signal(String::new);
    let mut display_size = use_signal(|| "size-6".to_string());
    let mut class_color = use_signal(String::new);

    let mut selected_icon_name = use_signal(String::new);
    let mut selected_icon_fn: Signal<Option<IconFn>> = use_signal(|| None);

    let filtered_icons = use_memo(move || {
        let search = search_text().to_lowercase();
        if search.is_empty() {
            ALL_ICONS.iter().collect::<Vec<_>>()
        } else {
            ALL_ICONS
                .iter()
                .filter(|(_, name)| name.to_lowercase().contains(&search))
                .collect::<Vec<_>>()
        }
    });
    let total_icons = use_memo(move || filtered_icons().len());

    let mut grid_element: Signal<Option<web_sys::Element>> = use_signal(|| None);
    let grid_scroll = use_grid_virtual_scroll(grid_element.into(), total_icons.into(), ICON_ITEM_SIZE);

    let container_style = use_memo(move || {
        let color = class_color();
        if color.is_empty() {
            String::new()
        } else {
            format!("--icon-color: {color}")
        }
    });

    rsx! {
        SeoMeta {
            title: "Icons · Rust UI Component Library | Rust/UI".to_string(),
            description: "Browse and copy Rust UI's icon set for Dioxus applications. Search, customize size and color, and get the ready-to-use component code."
                .to_string(),
        }
        HeaderDocs {}
        div { class: "flex overflow-hidden flex-1",
            aside { class: "hidden overflow-y-auto p-4 md:block w-[270px] bg-muted",
                Card { class: "flex flex-col gap-6 bg-background",
                    CardHeader {
                        div { class: "flex justify-between items-center",
                            CardTitle { "Customizer" }
                            button {
                                class: "group",
                                onclick: move |_| {
                                    display_size.set("size-6".to_string());
                                    class_color.set(String::new());
                                },
                                RotateCw { class: "transition-transform duration-300 size-5 group-active:rotate-30" }
                            }
                        }
                    }
                    CardContent { class: "flex flex-col gap-4",
                        div { class: "flex flex-col gap-2",
                            p { "Color" }
                            div { class: "flex gap-2 items-center p-2 rounded-md bg-muted",
                                div { class: "relative",
                                    input {
                                        r#type: "color",
                                        class: "rounded-full border-2 appearance-none cursor-pointer size-8 border-border",
                                        style: "background: transparent;",
                                        value: class_color(),
                                        oninput: move |e| class_color.set(e.value()),
                                    }
                                    div { class: "absolute inset-0 rounded-full border-2 pointer-events-none size-8 border-border" }
                                }
                                span { class: "text-sm text-muted-foreground", "current" }
                            }
                        }
                        div { class: "flex flex-col gap-2 min-w-[150px]",
                            label { r#for: SIZE_ID, class: "text-sm font-medium leading-none",
                                "Size"
                            }
                            select {
                                id: SIZE_ID,
                                class: "flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-2 text-sm shadow-xs transition-[color,box-shadow] outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px]",
                                value: display_size(),
                                onchange: move |e: FormEvent| display_size.set(e.value()),
                                option { value: "size-4", "Size 4" }
                                option { value: "size-6", "Size 6" }
                                option { value: "size-8", "Size 8" }
                            }
                        }
                    }
                }
            }

            div { class: "container flex-1 px-2 mx-auto",
                div {
                    class: "overflow-y-auto py-4 h-full",
                    onmounted: move |event| {
                        if let Some(element) = event.data().downcast::<web_sys::Element>().cloned() {
                            grid_element.set(Some(element));
                        }
                    },
                    div { class: "px-4 mx-auto mb-6 w-full max-w-md sm:px-0",
                        div { class: "flex gap-3 items-center",
                            Input {
                                r#type: InputType::Search,
                                placeholder: "Search icons... (Press Escape to clear)",
                                value: search_text(),
                                oninput: move |e: FormEvent| search_text.set(e.value()),
                            }
                            div { class: "text-sm whitespace-nowrap text-muted-foreground",
                                {
                                    let count = filtered_icons().len();
                                    let search = search_text();
                                    if search.is_empty() {
                                        format!("{count} icons")
                                    } else if count == 0 {
                                        "No icons found".to_string()
                                    } else {
                                        format!("{count} icons found")
                                    }
                                }
                            }
                        }
                    }

                    div {
                        style: format!("position: relative; height: {}px;", (grid_scroll.total_height)()),
                        div {
                            class: "grid absolute inset-x-0 gap-2 [&_svg]:transition-colors",
                            style: {
                                let columns = (grid_scroll.columns)().max(1);
                                let row_offset = (grid_scroll.start_index)() / columns;
                                format!(
                                    "grid-template-columns: repeat({columns}, 1fr); transform: translateY({}px); {}",
                                    row_offset * ICON_ITEM_SIZE,
                                    container_style(),
                                )
                            },
                            {
                                let start = (grid_scroll.start_index)();
                                let end = (grid_scroll.end_index)();
                                filtered_icons().into_iter().skip(start).take(end - start).map(|(icon, name)| {
                                    let size = display_size();
                                    let name_str = name.to_string();
                                    let icon_fn = *icon;
                                    rsx! {
                                        button {
                                            key: "{name}",
                                            class: "flex relative justify-center items-center p-4 rounded-md cursor-pointer bg-muted size-16 group hover:bg-muted/80",
                                            onclick: move |_| {
                                                selected_icon_name.set(name_str.clone());
                                                selected_icon_fn.set(Some(icon_fn));
                                                spawn(async move {
                                                    let _ = eval(r#"
                                                        const t = document.querySelector('[data-name="DrawerTrigger"]');
                                                        if (t) t.click();
                                                    "#).await;
                                                });
                                            },
                                            div {
                                                style: "content-visibility: auto; contain-intrinsic-size: 4rem; color: var(--icon-color, currentColor)",
                                                {icon(&size)}
                                            }
                                            div { class: "absolute left-1/2 top-full z-10 py-1 px-2 -mt-3 text-xs whitespace-nowrap rounded border shadow-md opacity-0 transition-opacity duration-200 transform -translate-x-1/2 pointer-events-none group-hover:opacity-100 bg-popover text-popover-foreground",
                                                "{name}"
                                            }
                                        }
                                    }
                                })
                            }
                        }
                    }
                }
            }
        }

        Drawer {
            DrawerTrigger { class: "hidden", "Open" }
            DrawerContent {
                DrawerHandle {}
                DrawerBody { class: "justify-center items-center",
                    DrawerTitle { {selected_icon_name()} }
                    div {
                        class: "flex justify-center items-center p-8 rounded-lg bg-muted w-fit",
                        style: container_style(),
                        {
                            selected_icon_fn().map_or_else(
                                || rsx! { div {} },
                                |icon_fn| rsx! {
                                    div {
                                        style: "color: var(--icon-color, currentColor)",
                                        class: "size-32",
                                        {icon_fn("size-32")}
                                    }
                                },
                            )
                        }
                    }
                    DrawerClose { "Close" }
                }
            }
        }
    }
}
