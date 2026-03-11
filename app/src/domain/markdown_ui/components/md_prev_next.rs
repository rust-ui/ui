use std::sync::Arc;

use app_domain::markdown_config::RegistryEntry;
use icons::{ChevronLeft, ChevronRight};
use leptos::prelude::*;
use leptos_ui::clx;
use markdown_crate::use_prev_next_demos;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PrevNextSizeMode {
    Compact,
    FullSize,
}

// TODO later. In Compact, display a Tooltip.

// TODO 🚑 Since we can't have `a` tag inside `button`, I'm doing this, cause Button does'nt have `Slot` at the moment..
const SHORTFIX_BUTTON_CLASS: &str = "py-0 px-2 h-8   inline-flex justify-center items-center text-sm font-medium whitespace-nowrap rounded-md transition-colors w-fit focus-visible:outline-hidden focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50   [&_svg:not(:last-child)]:mr-2 [&_svg:not(:first-child)]:ml-2 [&_svg:not([class*='size-'])]:size-4  border bg-background border-input hover:bg-accent hover:text-accent-foreground";

#[component]
pub fn MdPrevNextDemos(
    all_demos: Arc<Vec<RegistryEntry>>,
    current_demo: RegistryEntry,
    base_path: Arc<String>,
    #[prop(default = PrevNextSizeMode::FullSize)] size_mode: PrevNextSizeMode,
) -> impl IntoView {
    let current_demo_name = current_demo.title.to_string();

    // 🪝
    let (prev_demo_name, next_demo_name, prev_demo_href, next_demo_href) =
        use_prev_next_demos(all_demos, current_demo_name, Arc::clone(&base_path));

    let div_class = match size_mode {
        PrevNextSizeMode::FullSize => "flex justify-between items-center mt-8",
        PrevNextSizeMode::Compact => "flex gap-1 items-center",
    };

    let is_full_size = matches!(size_mode, PrevNextSizeMode::FullSize);

    clx! {DemoNav, a, SHORTFIX_BUTTON_CLASS, "z-50" } // I add z-50 to fix issue with SonnerList (ol) that was preventing clicks.

    view! {
        <div class=div_class>
            <DemoNav attr:href=prev_demo_href>
                <ChevronLeft />
                {is_full_size.then(|| view! { <span>{prev_demo_name.clone()}</span> })}
            </DemoNav>

            <DemoNav attr:href=next_demo_href>
                {is_full_size.then(|| view! { <span>{next_demo_name.clone()}</span> })} <ChevronRight />
            </DemoNav>
        </div>
    }
}
