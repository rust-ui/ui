---
title: "Demo Bottom Nav"
name: "demo_bottom_nav"
cargo_dependencies: []
registry_dependencies: ["bottom_nav"]
type: "components:demos"
path: "demos/demo_bottom_nav.rs"
---

# Demo Bottom Nav

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_bottom_nav
```

## Component Code

```rust
use icons::{CircleUser, House, SlidersHorizontal, Wallet};
use leptos::prelude::*;
use strum::{Display, EnumIter, IntoEnumIterator};

use crate::components::ui::bottom_nav::{BottomNav, BottomNavButton, BottomNavGrid, BottomNavLabel};

// * See enum below.

#[component]
pub fn DemoBottomNav() -> impl IntoView {
    let active_page_signal = RwSignal::new(NavPage::default());

    view! {
        <div class="flex flex-col my-10 rounded-t-2xl border h-[300px] w-[400px]">
            <div class="flex-1 bg-gray-200 rounded-t-2xl"></div>

            <BottomNav>
                <BottomNavGrid>
                    {NavPage::iter()
                        .map(|page| {
                            view! {
                                <BottomNavButton
                                    on:click=move |_| active_page_signal.set(page)
                                    attr:aria-current=move || {
                                        if active_page_signal.get() == page { "page" } else { "" }
                                    }
                                >

                                    {page.icon()}
                                    <BottomNavLabel>{page.to_string()}</BottomNavLabel>
                                </BottomNavButton>
                            }
                        })
                        .collect_view()}
                </BottomNavGrid>
            </BottomNav>
        </div>
    }
}

/* ========================================================== */
/*                       ✨  ENUM  ✨                         */
/* ========================================================== */

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumIter, Default)]
enum NavPage {
    Home,
    #[default]
    Wallet,
    Settings,
    Profile,
}

impl NavPage {
    fn icon(self) -> impl IntoView {
        match self {
            NavPage::Home => view! { <House class="size-5" /> }.into_any(),
            NavPage::Wallet => view! { <Wallet class="size-5" /> }.into_any(),
            NavPage::Settings => view! { <SlidersHorizontal class="size-5" /> }.into_any(),
            NavPage::Profile => view! { <CircleUser class="size-5" /> }.into_any(),
        }
    }
}
```
