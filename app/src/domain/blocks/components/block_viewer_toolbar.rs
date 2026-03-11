use icons::{Check, Copy, Fullscreen, Monitor, Share2, Smartphone, SvgIcon, Tablet, Terminal};
use leptos::prelude::*;
use leptos_ui::{clx, void};
use registry::hooks::use_copy_clipboard::use_copy_clipboard;
use registry::ui::button::{Button, ButtonSize, ButtonVariant};
use registry::ui::dialog::{
    Dialog, DialogBody, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle,
    DialogTrigger,
};
use registry::ui::input::Input;
use registry::ui::toggle_group::{ToggleGroup, ToggleGroupAction, ToggleGroupItem};
use strum::AsRefStr;
use wasm_bindgen::prelude::*;

use crate::domain::blocks::block_entry::BlockEntry;

#[derive(Default, Clone, Copy, PartialEq)]
pub enum BlockView {
    #[default]
    Preview,
    Code,
}

#[derive(Default, Clone, Copy, PartialEq, AsRefStr)]
pub enum ScreenSize {
    #[default]
    Desktop,
    Tablet,
    Phone,
}

const COPY_TIMEOUT_MS: i32 = 2000;

// Vanilla JS function to dispatch custom event
#[wasm_bindgen(inline_js = r#"
export function dispatchResizeEvent(instanceId, screenType) {
    document.dispatchEvent(new CustomEvent('resizable:resize_by_screen__interop', {
        detail: { instanceId, screenType }
    }));
}
"#)]
extern "C" {
    #[wasm_bindgen(js_name = dispatchResizeEvent)]
    fn dispatch_resize_event_js(instance_id: &str, screen_type: &str);
}

#[component]
pub fn BlockViewerToolbar(
    block_entry: BlockEntry,
    screen_size: RwSignal<ScreenSize>,
    block_view: RwSignal<BlockView>,
    #[prop(into)] instance_id: String,
) -> impl IntoView {
    clx! {Tabs, div, "flex flex-col gap-2"}
    clx! {Tablist, div, "grid grid-cols-2 justify-center items-center p-1 h-8 rounded-md bg-muted text-muted-foreground w-fit *:data-[name=tabs-trigger]:h-6 *:data-[name=TabTrigger]:rounded-sm *:data-[name=TabTrigger]:px-2 *:data-[name=TabTrigger]:text-xs"}
    clx! {TabTrigger, button, "inline-flex flex-1 gap-1.5 justify-center items-center py-1 px-2 text-sm font-medium whitespace-nowrap rounded-md border border-transparent disabled:opacity-50 disabled:pointer-events-none data-[state=active]:bg-background text-foreground h-[calc(100%-1px)] transition-[color,box-shadow] data-[state=active]:shadow-sm [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4 dark:data-[state=active]:text-foreground dark:data-[state=active]:border-input dark:data-[state=active]:bg-input/30 dark:text-muted-foreground focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:outline-ring focus-visible:ring-[3px] focus-visible:outline-1"}

    void! {Separator, div, "mx-2 bg-border shrink-0 data-[orientation=horizontal]:h-px data-[orientation=horizontal]:w-full data-[orientation=vertical]:h-full data-[orientation=vertical]:w-px !h-4"}

    let block_id_kebab = block_entry.block_id_kebab.to_string();
    let block_id = block_entry.block_id_kebab;

    let share_url_rwsignal = {
        let route = block_entry.block_route.as_ref();
        RwSignal::new(format!("https://rust-ui.com/blocks/{route}#{block_id_kebab}"))
    };

    let (copy_fn, copied) = use_copy_clipboard(Some(COPY_TIMEOUT_MS));
    let copy_to_clipboard = move |_| {
        copy_fn(&share_url_rwsignal.get());
    };

    // Helper function to dispatch resize events using vanilla JS
    let dispatch_resize_event = |size: ScreenSize, instance_id: &str| {
        dispatch_resize_event_js(instance_id, size.as_ref());
    };

    view! {
        <div class="hidden gap-2 items-center pl-2 w-full md:pr-6 lg:flex">
            <Tabs attr:dir="ltr" attr:data-orientation="horizontal">
                <Tablist
                    attr:role="tablist"
                    attr:data-orientation="horizontal"
                    attr:tabindex="0"
                    attr:style="outline: none;"
                >
                    <TabTrigger
                        attr:role="tab"
                        attr:aria-selected=move || (block_view.get() == BlockView::Preview).to_string()
                        attr:data-state=move || {
                            if block_view.get() == BlockView::Preview { "active" } else { "inactive" }
                        }
                        attr:tabindex="-1"
                        attr:data-orientation="horizontal"
                        on:click=move |_| block_view.set(BlockView::Preview)
                    >
                        Preview
                    </TabTrigger>
                    <TabTrigger
                        attr:role="tab"
                        attr:aria-selected=move || (block_view.get() == BlockView::Code).to_string()
                        attr:data-state=move || if block_view.get() == BlockView::Code { "active" } else { "inactive" }
                        attr:tabindex="-1"
                        attr:data-orientation="horizontal"
                        on:click=move |_| block_view.set(BlockView::Code)
                    >
                        Code
                    </TabTrigger>
                </Tablist>
            </Tabs>
            <Separator attr:data-orientation="vertical" attr:role="none" />
            <div class="w-full md:w-fit">
                <a
                    href=format!("#{block_id_kebab}")
                    class="flex-1 text-sm font-medium text-center md:flex-auto md:text-left hover:underline underline-offset-2"
                >
                    {block_entry.block_id_kebab.to_title()}
                </a>
            </div>

            <div class="flex gap-2 items-center ml-auto">
                <div class="flex gap-1.5 items-center p-1 h-8 rounded-md border shadow-none">
                    <ToggleGroup attr:role="group" attr:dir="ltr" attr:tabindex="0" attr:style="outline: none;">
                        <ToggleGroupItem
                            class="flex-none px-0 w-6 h-6"
                            attr:data-state=move || {
                                if screen_size.get() == ScreenSize::Desktop { "on" } else { "off" }
                            }
                            attr:aria-checked=move || (screen_size.get() == ScreenSize::Desktop).to_string()
                            title="Desktop size"
                            on:click={
                                let instance_id = instance_id.clone();
                                move |_| {
                                    screen_size.set(ScreenSize::Desktop);
                                    dispatch_resize_event(ScreenSize::Desktop, &instance_id);
                                }
                            }
                        >
                            <Monitor />
                        </ToggleGroupItem>
                        <ToggleGroupItem
                            class="flex-none px-0 w-6 h-6"
                            attr:data-state=move || {
                                if screen_size.get() == ScreenSize::Tablet { "on" } else { "off" }
                            }
                            attr:aria-checked=move || (screen_size.get() == ScreenSize::Tablet).to_string()
                            title="Tablet size"
                            on:click={
                                let instance_id = instance_id.clone();
                                move |_| {
                                    screen_size.set(ScreenSize::Tablet);
                                    dispatch_resize_event(ScreenSize::Tablet, &instance_id);
                                }
                            }
                        >
                            <Tablet />
                        </ToggleGroupItem>
                        <ToggleGroupItem
                            class="flex-none px-0 w-6 h-6"
                            attr:data-state=move || {
                                if screen_size.get() == ScreenSize::Phone { "on" } else { "off" }
                            }
                            attr:aria-checked=move || (screen_size.get() == ScreenSize::Phone).to_string()
                            title="Phone size"
                            on:click={
                                let instance_id = instance_id.clone();
                                move |_| {
                                    screen_size.set(ScreenSize::Phone);
                                    dispatch_resize_event(ScreenSize::Phone, &instance_id);
                                }
                            }
                        >
                            <Smartphone />
                        </ToggleGroupItem>

                        <Separator attr:data-orientation="vertical" attr:role="none" class="mx-0" />
                        <ToggleGroupAction
                            attr:target="_blank"
                            attr:href=block_entry.block_id_kebab.to_full_view_url()
                            attr:title="Open in New Tab"
                        >
                            <span class="hidden">Open in New Tab</span>
                            <Fullscreen />
                        </ToggleGroupAction>
                        <Separator attr:data-orientation="vertical" attr:role="none" class="mx-0" />
                        <Dialog>
                            <DialogTrigger class="p-0 text-sm font-medium whitespace-nowrap rounded-sm border-none transition-all outline-none disabled:opacity-50 disabled:pointer-events-none [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 shrink-0 [&_svg]:shrink-0 size-6 dark:aria-invalid:ring-destructive/40 dark:hover:bg-accent/50 hover:bg-accent hover:text-accent-foreground focus-visible:border-ring focus-visible:ring-[3px]">
                                <Share2 />
                            </DialogTrigger>

                            <DialogContent class="w-[425px]">
                                <DialogBody>
                                    <DialogHeader>
                                        <div class="flex gap-2 items-center">
                                            <DialogTitle>"Share Block"</DialogTitle>
                                            <Share2 class="size-5" />
                                        </div>

                                        <DialogDescription>
                                            "Copy the URL below to share this block with others."
                                        </DialogDescription>
                                    </DialogHeader>

                                    <div class="flex gap-2">
                                        <Input prop:value=share_url_rwsignal attr:readonly=true class="flex-1" />

                                        <Button variant=ButtonVariant::Outline on:click=copy_to_clipboard>
                                            {move || {
                                                if copied.get() {
                                                    view! { <Check /> }.into_any()
                                                } else {
                                                    view! { <Copy /> }.into_any()
                                                }
                                            }}
                                        </Button>
                                    </div>

                                    <DialogFooter>
                                        <DialogClose>"Close"</DialogClose>
                                    </DialogFooter>
                                </DialogBody>
                            </DialogContent>
                        </Dialog>
                    </ToggleGroup>
                </div>
                <Separator attr:data-orientation="vertical" attr:role="none" />
                // TODO. Be able to copy the command line.
                <Button
                    variant=ButtonVariant::Outline
                    size=ButtonSize::Sm
                    attr:title="Copy the command line"
                    attr:disabled=true
                >
                    <Terminal />
                    <span>{format!("(Soon) ui add {block_id}")}</span>
                </Button>
                <Separator attr:data-orientation="vertical" attr:role="none" />
                <Button
                    attr:target="_blank"
                    attr:rel="noopener noreferrer"
                    size=ButtonSize::Sm
                    href=format!("/registry/blocks/{}", block_id.to_md())
                >
                    <span>"View Md"</span>
                    <SvgIcon>
                        <title>Markdown logo</title>
                        <path
                            fill-rule="evenodd"
                            clip-rule="evenodd"
                            d="M19.5 2.25H2.5C1.80964 2.25 1.25 2.80964 1.25 3.5V12.5C1.25 13.1904 1.80964 13.75 2.5 13.75H19.5C20.1904 13.75 20.75 13.1904 20.75 12.5V3.5C20.75 2.80964 20.1904 2.25 19.5 2.25ZM2.5 1C1.11929 1 0 2.11929 0 3.5V12.5C0 13.8807 1.11929 15 2.5 15H19.5C20.8807 15 22 13.8807 22 12.5V3.5C22 2.11929 20.8807 1 19.5 1H2.5ZM3 4.5H4H4.25H4.6899L4.98715 4.82428L7 7.02011L9.01285 4.82428L9.3101 4.5H9.75H10H11V5.5V11.5H9V7.79807L7.73715 9.17572L7 9.97989L6.26285 9.17572L5 7.79807V11.5H3V5.5V4.5ZM15 8V4.5H17V8H19.5L17 10.5L16 11.5L15 10.5L12.5 8H15Z"
                            fill="currentColor"
                        ></path>
                    </SvgIcon>
                </Button>
            </div>
        </div>
    }
}
