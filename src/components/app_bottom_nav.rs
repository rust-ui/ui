use dioxus::prelude::*;
use icons::{Blocks, ChartSpline, Compass, House, Search};
use registry::ui::bottom_nav::{BottomNav, BottomNavButton, BottomNavGrid, BottomNavLabel};

use crate::Route;

#[derive(Clone, Copy, PartialEq, Eq, strum::IntoStaticStr)]
enum NavPage {
    Home,
    Components,
    Hooks,
    Icons,
    Charts,
}

const PAGES: &[NavPage] = &[
    NavPage::Home,
    NavPage::Components,
    NavPage::Hooks,
    NavPage::Icons,
    NavPage::Charts,
];

impl NavPage {
    fn label(self) -> &'static str {
        self.into()
    }

    fn target(self) -> Route {
        match self {
            Self::Home => Route::Home {},
            Self::Components => Route::DocsComponentsIndexPage {},
            Self::Hooks => Route::DocsHooksIndexPage {},
            Self::Icons => Route::PageIcons {},
            Self::Charts => Route::AreaChartPage {},
        }
    }

    #[allow(clippy::missing_errors_doc)] // Private Dioxus helper; framework owns rendering errors.
    fn icon(self) -> Element {
        match self {
            Self::Home => rsx! { House { class: "size-5" } },
            Self::Components => rsx! { Blocks { class: "size-5" } },
            Self::Hooks => rsx! { Compass { class: "size-5" } },
            Self::Icons => rsx! { Search { class: "size-5" } },
            Self::Charts => rsx! { ChartSpline { class: "size-5" } },
        }
    }

    fn is_active(self, route: &Route) -> bool {
        match self {
            Self::Home => matches!(route, Route::Home {}),
            Self::Components => {
                matches!(route, Route::DocsComponentsIndexPage {} | Route::ComponentPage { .. })
            }
            Self::Hooks => matches!(route, Route::DocsHooksIndexPage {} | Route::HookPage { .. }),
            Self::Icons => matches!(route, Route::PageIcons {}),
            // All concrete chart routes. Add here if a new *ChartPage is introduced.
            Self::Charts => matches!(
                route,
                Route::AreaChartPage {}
                    | Route::BarChartPage {}
                    | Route::LineChartPage {}
                    | Route::PieChartPage {}
                    | Route::RadarChartPage {}
                    | Route::RadialChartPage {}
            ),
        }
    }
}

/// App-level mobile bottom navigation. Shown only below the `sm` breakpoint,
/// mounted once in `AppLayout`. Ported from the Leptos site; reuses the
/// `registry::ui::bottom_nav` primitives (which carry the iOS/WebKit hacks).
#[component]
pub fn AppBottomNav() -> Element {
    let route = use_route::<Route>();
    let navigator = use_navigator();

    rsx! {
        BottomNav { class: "fixed inset-x-0 bottom-0 sm:hidden",
            BottomNavGrid {
                for page in PAGES {
                    {
                        let page = *page;
                        let active = page.is_active(&route);
                        rsx! {
                            BottomNavButton {
                                aria_current: if active { "page" } else { "" },
                                onclick: move |_| {
                                    navigator.push(page.target());
                                },
                                {page.icon()}
                                BottomNavLabel { {page.label()} }
                            }
                        }
                    }
                }
            }
        }
    }
}
