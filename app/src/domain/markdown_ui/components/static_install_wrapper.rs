use app_components::{TabList, TabTrigger};
use leptos::prelude::*;
use markdown_crate::highlight_language::HighlightLanguage;
use markdown_crate::leptos::components::syntect_highlighter_code::SyntectHighlighterCode;
use markdown_crate::ui::steps::{Step, Steps};
use registry::hooks::use_copy_clipboard::use_copy_clipboard;
use registry::ui::button::{Button, ButtonVariant};

use crate::__registry__::static_md_registry::{MarkdownType, get_static_registry_entry};

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum Tab {
    #[default]
    Cli,
    Manual,
}

#[component]
pub fn StaticInstallWrapper(install_type: MarkdownType) -> impl IntoView {
    let current_tab = RwSignal::new(Tab::default());

    // Zero-allocation static lookup
    let Some(install_data) = get_static_registry_entry(install_type) else {
        return view! { <p>"Install not found in static registry"</p> }.into_any();
    };

    // TODO. Copy Button top right
    let (_copy_to_clipboard, _copied) = use_copy_clipboard(None);

    // CLI commands (could be optimized further with const_format)
    let cli_add_demo = format!("ui add {}", install_data.demo_name);
    let cli_add_component = format!("ui add {}", install_data.install_name);

    // Copy functionality can be added later if needed

    let expanded_signal = RwSignal::new(false);

    let handle_expand = move |_| {
        expanded_signal.set(!expanded_signal.get());
    };

    view! {
        <div class="flex flex-col w-full">
            <TabList attr:tabindex="0" attr:data-orientation="horizontal">
                <TabTrigger
                    attr:data-state=move || { if current_tab.get() == Tab::Cli { "active" } else { "inactive" } }
                    attr:tabindex="0"
                    on:click=move |_| current_tab.set(Tab::Cli)
                >
                    <span>"CLI"</span>
                </TabTrigger>
                <TabTrigger
                    attr:data-state=move || { if current_tab.get() == Tab::Manual { "active" } else { "inactive" } }
                    attr:tabindex="0"
                    on:click=move |_| current_tab.set(Tab::Manual)
                >
                    <span>"Manual"</span>
                </TabTrigger>
            </TabList>

            // CLI Tab Content
            <div style:display=move || { if current_tab.get() == Tab::Cli { "block" } else { "none" } }>
                <Steps>
                    <Step>"You can run either of the following commands:"</Step>
                    <SyntectHighlighterCode
                        code=format!("# cargo install ui-cli --force\n{cli_add_demo}\n{cli_add_component}")
                        language=HighlightLanguage::Bash
                        class="mt-4 leading-none"
                    />
                    <Step>"Update the imports to match your project setup."</Step>
                </Steps>
            </div>

            // Manual Tab Content
            <div style:display=move || { if current_tab.get() == Tab::Manual { "block" } else { "none" } }>
                <Steps>
                    <Step>"Copy and paste the following code into your project:"</Step>

                    <p class="leading-7 [&:not(:first-child)]:mt-6">
                        <code class="font-mono rounded bg-muted px-[0.3rem] py-[0.2rem] text-[0.75rem] sm:text-[0.8125rem]">
                            {format!("components/ui/{}.rs", install_data.install_name)}
                        </code>
                    </p>

                    <div
                        data-name="__ManualStepCodeWrapper"
                        data-expanded=move || expanded_signal.get().then_some("true")
                        class="overflow-hidden relative my-6 h-[100px] data-[expanded=true]:h-fit data-[expanded=true]:overflow-auto"
                    >
                        <SyntectHighlighterCode
                            code=install_data.raw_code.to_string()
                            language=HighlightLanguage::Rust
                            class="h-full max-h-[370px] no__scrollbar"
                            show_scrollbar_on_hover=false
                        />

                        <Button
                            variant=ButtonVariant::Secondary
                            class="absolute bottom-4 left-1/2 transform -translate-x-1/2 hover:bg-secondary"
                            on:click=handle_expand
                        >
                            {move || if expanded_signal.get() { "Collapse" } else { "Expand" }}
                        </Button>
                    </div>

                    <Step>"Update the imports to match your project setup."</Step>
                </Steps>
            </div>
        </div>
    }
    .into_any()
}
