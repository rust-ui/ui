use app_config::{SeoMeta, SiteConfig};
use app_domain::icons::all_icons::ALL_ICONS;
use icons::RotateCw;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use registry::ui::card::{Card, CardContent, CardHeader, CardTitle};
use registry::ui::drawer::{Drawer, DrawerBody, DrawerClose, DrawerContent, DrawerHandle, DrawerTitle, DrawerTrigger};
use registry::ui::input::Input;
use registry::ui::scroll_area::ScrollArea;
use registry::ui::select_native::{LabelNative, SelectNative};
use registry::utils::query::{QUERY, QueryUtils};

use crate::components::navigation::header_docs::HeaderDocs;

// TODO later: Make it work without using `style` attribute.

#[component]
pub fn PageIcons() -> impl IntoView {
    const TARGET_ID: &str = "TARGET_NATIVE_GROUP";

    let query_size = QueryUtils::extract(QUERY::SIZE.to_string());
    let query_search = QueryUtils::extract(QUERY::SEARCH.to_string());
    let query_color = QueryUtils::extract(QUERY::COLOR.to_string());
    let navigate = use_navigate();

    let navigate_size = navigate.clone();
    let navigate_search = navigate.clone();
    let navigate_color = navigate.clone();
    let navigate_reset = navigate.clone();
    let navigate_escape = navigate;

    let search_text = RwSignal::new(query_search.get_untracked().unwrap_or_default());

    let class_color = RwSignal::new(query_color.get_untracked().unwrap_or_default());

    // Initialize display_size with current query value to prevent icon re-rendering on page load.
    // Using get_untracked() ensures synchronous initialization without creating reactive dependencies.
    let display_size = RwSignal::new(query_size.get_untracked().unwrap_or_else(|| "size-6".to_string()));

    // Create a separate signal for SelectNative that reads directly from query without delay
    let select_value = RwSignal::new(query_size.get_untracked().unwrap_or_else(|| "size-6".to_string()));

    // Watch for query changes to update signals when user navigates or changes URL
    Effect::watch(
        move || (query_size.get(), query_search.get(), query_color.get()),
        move |(size, search, color), _, _| {
            let new_size = size.clone().unwrap_or_else(|| "size-6".to_string());
            display_size.set(new_size.clone());
            select_value.set(new_size);

            let new_search = search.clone().unwrap_or_default();
            search_text.set(new_search);

            let new_color = color.clone().unwrap_or_default();
            class_color.set(new_color);
        },
        false,
    );

    // Size memo for icons - doesn't trigger re-render on color change
    let size_memo = Memo::new(move |_| display_size.get());

    // CSS variable for color - browser handles the update without re-rendering
    let container_style = Memo::new(move |_| {
        let color = class_color.get();
        if color.is_empty() { String::new() } else { format!("--icon-color: {}", color) }
    });

    let handle_color_change = Callback::new(move |color: String| {
        let current_search = query_search.get().unwrap_or_default();
        let current_size = query_size.get().unwrap_or_else(|| "size-6".to_string());
        let mut params = vec![format!("size={}", current_size)];
        if !current_search.is_empty() {
            params.push(format!("search={}", current_search));
        }
        params.push(format!("color={}", color.replace('#', "%23")));
        navigate_color(&format!("?{}", params.join("&")), Default::default());
    });

    let filtered_icons = Memo::new(move |_| {
        let search = search_text.get().to_lowercase();
        if search.is_empty() {
            ALL_ICONS.iter().collect::<Vec<_>>()
        } else {
            ALL_ICONS.iter().filter(|(_, name)| name.to_lowercase().contains(&search)).collect::<Vec<_>>()
        }
    });

    // ------- DRAWER --------- //
    let selected_icon_name = RwSignal::new(String::new());
    let selected_icon_function = RwSignal::new(None::<fn(&str) -> AnyView>);

    let open_drawer = move |icon_name: String, icon_func: fn(&str) -> AnyView| {
        selected_icon_name.set(icon_name);
        selected_icon_function.set(Some(icon_func));

        // Click the hidden trigger to let JavaScript handle the drawer opening
        let window = window();
        if let Some(document) = window.document()
            && let Ok(Some(trigger)) = document.query_selector("[data-name=\"DrawerTrigger\"]")
        {
            use wasm_bindgen::JsCast;
            if let Some(button) = trigger.dyn_ref::<web_sys::HtmlElement>() {
                button.click();
            }
        }
    };

    let title = format!("Leptos Rust UI Icons · 1600+ Lucide Icons | {}", SiteConfig::TITLE);
    let description = "Browse and customize 1,600+ Lucide icons for your Leptos Rust applications. Search, resize, change colors, and copy icons instantly.".to_string();
    let canonical_url = format!("{}/icons", SiteConfig::BASE_URL);
    let og_title = "Leptos Rust UI Icons · 1600+ Lucide Icons Library".to_string();

    view! {
        <SeoMeta title=title description=description canonical_url=canonical_url og_title=og_title />

        <HeaderDocs />

        <div class="flex overflow-hidden flex-1">
            <aside class="hidden overflow-y-auto p-4 md:block w-[270px] bg-muted">
                <Card class="flex flex-col gap-6 bg-background">
                    <CardHeader>
                        <div class="flex justify-between items-center">
                            <CardTitle>"Customizer"</CardTitle>

                            <button
                                class="group"
                                on:click=move |_| {
                                    let current_search = query_search.get().unwrap_or_default();
                                    if current_search.is_empty() {
                                        navigate_reset("?", Default::default());
                                    } else {
                                        navigate_reset(&format!("?search={}", current_search), Default::default());
                                    }
                                }
                            >
                                <RotateCw class="transition-transform duration-300 size-5 group-active:rotate-30" />
                            </button>
                        </div>
                    </CardHeader>

                    <CardContent class="flex flex-col gap-4">

                        <div class="flex flex-col gap-2">
                            <p>"Color"</p>

                            <div class="flex gap-2 items-center p-2 rounded-md bg-muted">
                                <div class="relative">
                                    <input
                                        type="color"
                                        prop:value=move || class_color.get()
                                        class="rounded-full border-2 appearance-none cursor-pointer size-8 border-border bg-background"
                                        style="background: transparent;"
                                        on:input=move |ev| {
                                            class_color.set(event_target_value(&ev));
                                        }
                                        on:change=move |ev| {
                                            handle_color_change.run(event_target_value(&ev));
                                        }
                                    />
                                    <div class="absolute inset-0 rounded-full border-2 pointer-events-none size-8 border-border"></div>
                                </div>
                                <span class="text-sm text-muted-foreground">"current"</span>
                            </div>
                        </div>

                        <div class="flex flex-col gap-2 min-w-[150px]">
                            <LabelNative r#for=TARGET_ID>"Size"</LabelNative>

                            <SelectNative
                                id=TARGET_ID
                                value=select_value.read_only()
                                on_change=leptos::prelude::Callback::new(move |ev| {
                                    let new_size = event_target_value(&ev);
                                    let current_search = query_search.get().unwrap_or_default();
                                    let current_color = query_color.get().unwrap_or_default();
                                    let mut params = vec![format!("size={}", new_size)];
                                    if !current_search.is_empty() {
                                        params.push(format!("search={}", current_search));
                                    }
                                    if !current_color.is_empty() {
                                        params.push(format!("color={}", current_color.replace('#', "%23")));
                                    }
                                    navigate_size(&format!("?{}", params.join("&")), Default::default());
                                })
                            >
                                <option value="size-4">"Size 4"</option>
                                <option value="size-6">"Size 6"</option>
                                <option value="size-8">"Size 8"</option>
                            </SelectNative>
                        </div>
                    </CardContent>
                </Card>
            </aside>

            <div class="container flex-1 px-2 mx-auto">
                // * Override ScrollAreaViewport to prevent horizontal scrolling from icon tooltips
                <ScrollArea class="h-full [&_[data-name='ScrollAreaViewport']]:overflow-y-auto [&_[data-name='ScrollAreaViewport']]:overflow-x-clip">
                    <div
                        class="py-4"
                        on:keydown=move |ev| {
                            let key = ev.key();
                            if key == "Escape" {
                                search_text.set(String::new());
                                let current_size = query_size.get().unwrap_or_else(|| "size-6".to_string());
                                let current_color = query_color.get().unwrap_or_default();
                                let mut params = vec![format!("size={}", current_size)];
                                if !current_color.is_empty() {
                                    params.push(format!("color={}", current_color.replace('#', "%23")));
                                }
                                navigate_escape(&format!("?{}", params.join("&")), Default::default());
                            }
                        }
                    >

                        <div class="px-4 mx-auto mb-6 w-full max-w-md sm:px-0">
                            <div class="flex gap-3 items-center">
                                <Input
                                    attr:r#type="search"
                                    attr:placeholder="Search icons... (Press Escape to clear)"
                                    prop:value=search_text
                                    on:input=move |ev| {
                                        let new_search = event_target_value(&ev);
                                        search_text.set(new_search.clone());
                                        let current_size = query_size.get().unwrap_or_else(|| "size-6".to_string());
                                        let current_color = query_color.get().unwrap_or_default();
                                        let mut params = vec![format!("size={}", current_size)];
                                        if !new_search.is_empty() {
                                            params.push(format!("search={}", new_search));
                                        }
                                        if current_color != "#FC0D1B" {
                                            params.push(format!("color={}", current_color.replace('#', "%23")));
                                        }
                                        navigate_search(&format!("?{}", params.join("&")), Default::default());
                                    }
                                />
                                <div class="text-sm whitespace-nowrap text-muted-foreground">
                                    {move || {
                                        let count = filtered_icons.get().len();
                                        let search = search_text.get();
                                        if search.is_empty() {
                                            format!("{count} icons")
                                        } else if count == 0 {
                                            "No icons found".to_string()
                                        } else {
                                            format!("{count} icons found")
                                        }
                                    }}
                                </div>
                            </div>
                        </div>

                        <div
                            class="grid grid-cols-5 gap-2 sm:grid-cols-6 md:grid-cols-8 lg:grid-cols-10 [&_svg]:transition-colors xl:grid-cols-13 2xl:grid-cols-15"
                            style=move || container_style.get()
                        >

                            {move || {
                                let size = size_memo.get();
                                filtered_icons
                                    .get()
                                    .into_iter()
                                    .map(|(icon, name)| {
                                        let size_clone = size.clone();
                                        let name_clone = name.to_string();
                                        let icon_func = *icon;
                                        view! {
                                            <button
                                                class="flex relative justify-center items-center p-4 rounded-md cursor-pointer bg-muted size-16 group hover:bg-muted/80"
                                                on:click=move |ev| {
                                                    ev.prevent_default();
                                                    open_drawer(name_clone.clone(), icon_func);
                                                }
                                            >
                                                // * 🚀 Performance: Apply content-visibility to inner wrapper to optimize rendering of large icon sets
                                                <div
                                                    class="[content-visibility:auto] [contain-intrinsic-size:4rem]"
                                                    style="color: var(--icon-color, currentColor)"
                                                >
                                                    {icon(&size_clone)}
                                                </div>
                                                <div class="absolute left-1/2 top-full z-10 py-1 px-2 -mt-3 text-xs whitespace-nowrap rounded border shadow-md opacity-0 transition-opacity duration-200 transform -translate-x-1/2 pointer-events-none group-hover:opacity-100 bg-popover text-popover-foreground">
                                                    {*name}
                                                </div>
                                            </button>
                                        }
                                    })
                                    .collect_view()
                            }}
                        </div>

                    </div>
                </ScrollArea>
            </div>
        </div>

        // Hidden trigger for JavaScript to wire up
        <DrawerTrigger class="hidden">"Open"</DrawerTrigger>

        <Drawer show_overlay=false lock_body_scroll=false>
            <DrawerContent>
                <DrawerHandle />
                <DrawerBody class="justify-center items-center">
                    <DrawerTitle>{move || selected_icon_name.get()}</DrawerTitle>

                    <div
                        class="flex justify-center items-center p-8 rounded-lg bg-muted w-fit"
                        style=move || container_style.get()
                    >
                        {move || {
                            if let Some(icon_func) = selected_icon_function.get() {
                                view! {
                                    <div style="color: var(--icon-color, currentColor)" class="size-32">
                                        {icon_func("size-32")}
                                    </div>
                                }
                                    .into_any()
                            } else {
                                view! { <div></div> }.into_any()
                            }
                        }}
                    </div>

                    <DrawerClose>Close</DrawerClose>
                </DrawerBody>
            </DrawerContent>
        </Drawer>
    }
}
