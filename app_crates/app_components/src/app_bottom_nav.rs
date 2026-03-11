use icons::{Blocks, ChartSpline, Compass, House, Search};
use leptos::prelude::*;
use leptos_router::hooks::{use_location, use_navigate};
use registry::ui::bottom_nav::{BottomNav, BottomNavButton, BottomNavGrid, BottomNavLabel};
use strum::{Display, EnumIter, IntoEnumIterator};

#[component]
pub fn AppBottomNav() -> impl IntoView {
    let location = use_location();
    let navigate = use_navigate();

    let is_active = move |path: &'static str| -> &'static str {
        let pathname = location.pathname.get();
        let matches = if path == "/" { pathname == "/" } else { pathname.starts_with(path) };
        if matches { "page" } else { "" }
    };

    view! {
        <BottomNav class="fixed right-0 bottom-0 left-0 sm:hidden">
            <BottomNavGrid>
                {NavPage::iter()
                    .map(|page| {
                        let path = page.path();
                        let navigate = navigate.clone();
                        view! {
                            <BottomNavButton
                                on:click=move |_| {
                                    navigate(path, Default::default());
                                }

                                attr:aria-current=move || is_active(path)
                            >
                                {page.icon()}
                                <BottomNavLabel>{page.to_string()}</BottomNavLabel>
                            </BottomNavButton>
                        }
                    })
                    .collect_view()}
            </BottomNavGrid>
        </BottomNav>
    }
}

/* ========================================================== */
/*                       ✨  ENUM  ✨                         */
/* ========================================================== */

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumIter, Default)]
enum NavPage {
    #[default]
    Home,
    Components,
    Hooks,
    Icons,
    Charts,
}

impl NavPage {
    fn path(self) -> &'static str {
        match self {
            NavPage::Home => "/",
            NavPage::Components => "/docs/components",
            NavPage::Hooks => "/docs/hooks",
            NavPage::Icons => "/icons",
            NavPage::Charts => "/charts/area-chart",
        }
    }

    fn icon(self) -> impl IntoView {
        match self {
            NavPage::Home => view! { <House class="size-5" /> }.into_any(),
            NavPage::Components => view! { <Blocks class="size-5" /> }.into_any(),
            NavPage::Hooks => view! { <Compass class="size-5" /> }.into_any(),
            NavPage::Icons => view! { <Search class="size-5" /> }.into_any(),
            NavPage::Charts => view! { <ChartSpline class="size-5" /> }.into_any(),
        }
    }
}
