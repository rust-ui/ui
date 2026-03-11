use app_components::{TabList, TabTrigger};
use icons::{Check, Code, Copy, EllipsisVertical, Eye, Terminal};
use leptos::prelude::*;
use markdown_crate::highlight_language::HighlightLanguage;
use markdown_crate::syntect_highlighter_code::SyntectHighlighterCode;
use registry::hooks::use_copy_clipboard::use_copy_clipboard;
use registry::ui::button::{Button, ButtonSize, ButtonVariant};
use registry::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuAlign, DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem, DropdownMenuLink,
    DropdownMenuTrigger,
};
use registry::ui::separator::Separator;

use crate::__registry__::static_md_registry::{MarkdownType, get_static_registry_entry};
use crate::domain::markdown_ui::components::resizable_wrapper::ResizableWrapper;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum Tab {
    #[default]
    Preview,
    Code,
}

/// Transform code for display by replacing internal registry paths with user-facing component paths
fn transform_code_for_display(code: &str) -> String {
    code.replace("use crate::registry::", "use crate::components::")
}

#[component]
pub fn StaticDemoWrapper(demo_type: MarkdownType, children: Children) -> impl IntoView {
    let current_tab = RwSignal::new(Tab::default());

    // Zero-allocation static lookup
    let Some(demo_data) = get_static_registry_entry(demo_type) else {
        return view! { <p>"Demo not found in static registry"</p> }.into_any();
    };

    // Separate copy signals for CLI command and demo code
    let (copy_to_clipboard_cli, copied_cli) = use_copy_clipboard(None);
    let (copy_to_clipboard_demo, copied_demo) = use_copy_clipboard(None);

    // All data is static - no cloning needed
    let rust_code = demo_data.raw_code; // ✅ &'static str
    let demo_name = demo_data.demo_name; // ✅ &'static str

    // Transform code for display (replacing internal paths with user-facing paths)
    let transformed_code = transform_code_for_display(rust_code);

    // Use format! for now (we can optimize this later with const_format)
    let cli_command = format!("ui add {}", demo_name);

    let handle_copy_demo = move |_| {
        copy_to_clipboard_demo(rust_code); // ✅ No clone needed
    };

    let handle_copy_cli = move |_| {
        copy_to_clipboard_cli(&cli_command);
    };

    // Store children at the top level so we can reference it reactively
    let children_view = children();

    view! {
        <div class="flex flex-col gap-2 w-full">
            <div class="flex justify-between items-center">
                <TabList attr:tabindex="0" attr:data-orientation="horizontal">
                    <TabTrigger
                        attr:data-state=move || {
                            if current_tab.get() == Tab::Preview { "active" } else { "inactive" }
                        }
                        attr:tabindex="0"
                        on:click=move |_| current_tab.set(Tab::Preview)
                    >
                        <Eye />
                        <span>Preview</span>
                    </TabTrigger>
                    <TabTrigger
                        attr:data-state=move || { if current_tab.get() == Tab::Code { "active" } else { "inactive" } }
                        attr:tabindex="0"
                        on:click=move |_| current_tab.set(Tab::Code)
                    >
                        <Code />
                        <span>Code</span>
                    </TabTrigger>
                </TabList>

                <div class="flex gap-2 items-center">
                    <Button
                        class="hidden sm:flex"
                        variant=ButtonVariant::Outline
                        size=ButtonSize::Sm
                        attr:title="Copy the command line"
                        on:click=handle_copy_cli
                    >
                        {move || {
                            if copied_cli.get() {
                                view! { <Check /> }.into_any()
                            } else {
                                view! { <Terminal /> }.into_any()
                            }
                        }}
                        <span>{format!("ui add {demo_name}")}</span>
                    </Button>

                    <DropdownMenu align=DropdownMenuAlign::End>
                        <DropdownMenuTrigger class="px-2 h-8">
                            <EllipsisVertical />
                        </DropdownMenuTrigger>

                        <DropdownMenuContent>
                            <DropdownMenuGroup>
                                <DropdownMenuItem on:click=handle_copy_demo>
                                    {move || {
                                        if copied_demo.get() {
                                            view! { <Check /> }.into_any()
                                        } else {
                                            view! { <Copy /> }.into_any()
                                        }
                                    }} <span>"Copy Demo"</span>
                                </DropdownMenuItem>
                            </DropdownMenuGroup>

                            <Separator class="my-1" />

                            <DropdownMenuGroup>
                                <DropdownMenuItem>
                                    <DropdownMenuLink
                                        attr:target="_blank"
                                        attr:rel="noopener noreferrer"
                                        attr:href=format!("/registry/styles/default/{}.md", demo_name)
                                    >
                                        "View as Markdown"
                                    </DropdownMenuLink>
                                </DropdownMenuItem>
                            </DropdownMenuGroup>
                        </DropdownMenuContent>
                    </DropdownMenu>
                </div>
            </div>

            // TODO 🚑 Show does not work at the moment. Using display as shortfix solution.
            <div style:display=move || { if current_tab.get() == Tab::Preview { "block" } else { "none" } }>
                <ResizableWrapper preview_class="px-4">{children_view}</ResizableWrapper>
            </div>

            <div style:display=move || { if current_tab.get() == Tab::Code { "block" } else { "none" } }>
                // ✅ Transformed code with syntax highlighting - server-only to avoid hydration issues
                <SyntectHighlighterCode code=transformed_code language=HighlightLanguage::Rust />
            </div>
        </div>
    }
    .into_any()
}
