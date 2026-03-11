use app_components::{AppBottomNav, MyReactiveIndicator, MyTailwindIndicator};
use app_domain::constants::RoutePaths;
use app_domain::utils::PARAM;
use app_routes::{BlockRoutes, ChartRoutes, ComponentsRoutes, HooksRoutes};
use leptos::prelude::*;
use leptos_meta::{Html, provide_meta_context};
use leptos_router::components::{ParentRoute, Route, Router, Routes};
use leptos_router::hooks::use_location;
use leptos_router::{ParamSegment, StaticSegment, path};
use registry::blocks::sidenav01::Sidenav01Routes;
use registry::blocks::sidenav02::Sidenav02Routes;
use registry::blocks::sidenav03::Sidenav03Routes;
use registry::blocks::sidenav04::Sidenav04Routes;
use registry::blocks::sidenav05::Sidenav05Routes;
use registry::blocks::sidenav06::Sidenav06Routes;
use registry::blocks::sidenav07::Sidenav07Routes;
use registry::blocks::sidenav08::Sidenav08Routes;
use registry::blocks::sidenav09::Sidenav09Routes;
use registry::blocks::sidenav10::Sidenav10Routes;
use registry::blocks::sidenav11::Sidenav11Routes;
use registry::hooks::use_data_scrolled::DATA_SCROLL_TARGET;
use registry::hooks::use_theme_mode::ThemeMode;
use registry::ui::sonner::SonnerToaster;
use registry::ui::toast_custom::toaster::{Toaster, provide_toaster};

use crate::components::navigation::app_wrapper::AppWrapper;
use crate::domain::blocks::routing::blocks_layout::BlocksLayout;
use crate::domain::blocks::routing::blocks_pages::{
    FaqBlocks, FootersBlocks, HeadersBlocks, IntegrationsBlocks, LoginBlocks, SidenavBlocks,
};
use crate::domain::bug_report::page_bug_reports::PageBugReports;
use crate::domain::charts::routing::charts_layout::ChartsLayout;
use crate::domain::charts::routing::charts_pages::{
    AreaChartPage, BarChartPage, LineChartPage, PieChartPage, RadarChartPage, RadialChartPage,
};
use crate::domain::create::page_create::PageCreate;
use crate::domain::docs::routing::docs_layout::DocsLayout;
use crate::domain::docs::routing::page_all_demos::PageAllDemos;
use crate::domain::docs::routing::shared_routes_demo::SharedRoutesDemo;
use crate::domain::icons::page_icons::PageIcons;
use crate::domain::tests::page_test::PageTest;
use crate::domain::tests::page_to_fix::PageToFix;
use crate::domain::views::view_router::ViewRouter;
use crate::domain::views::views_layout::ViewsLayout;
use crate::routes::page_download::PageDownload;
use crate::routes::page_home::PageHome;
use crate::routes::page_not_found::PageNotFound;

/// Scrolls the main scroll container to top on route changes
#[component]
fn ScrollToTop() -> impl IntoView {
    let location = use_location();

    Effect::new(move |_| {
        // Track location changes
        let _ = location.pathname.get();

        // Scroll the custom scroll container to top
        if let Some(window) = web_sys::window()
            && let Some(document) = window.document()
            && let Some(element) = document.get_element_by_id(DATA_SCROLL_TARGET)
        {
            element.set_scroll_top(0);
        }
    });
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    provide_toaster();

    let theme_mode = ThemeMode::init();

    view! {
        <Html {..} class=move || if theme_mode.is_dark() { "dark" } else { "" } />

        <Router>
            <ScrollToTop />
            <Toaster />
            <SonnerToaster />
            <MyTailwindIndicator />
            <MyReactiveIndicator />

            <AppWrapper>
                <main id=DATA_SCROLL_TARGET class="overflow-y-auto flex-1 overflow-x-clip">
                    <Routes fallback=|| PageNotFound.into_view()>
                        // --------- DOCS --------- //
                        <ParentRoute path=StaticSegment("docs") view=DocsLayout>
                            <Route
                                path=StaticSegment(ComponentsRoutes::segment())
                                view=|| view! { <PageAllDemos segment="components" /> }
                            />
                            <Route
                                path=(StaticSegment(ComponentsRoutes::segment()), ParamSegment(PARAM::NAME))
                                view=|| view! { <SharedRoutesDemo route_path=ComponentsRoutes::base_url() /> }
                            />

                            <Route
                                path=StaticSegment(HooksRoutes::segment())
                                view=|| view! { <PageAllDemos segment="hooks" /> }
                            />
                            <Route
                                path=(StaticSegment(HooksRoutes::segment()), ParamSegment(PARAM::NAME))
                                view=|| view! { <SharedRoutesDemo route_path=HooksRoutes::base_url() /> }
                            />
                        </ParentRoute>

                        <Route path=StaticSegment(RoutePaths::HOME) view=PageHome />
                        <Route path=StaticSegment(RoutePaths::DOWNLOAD) view=PageDownload />
                        <Route path=path!("/icons") view=PageIcons />

                        // --------- CHARTS --------- //
                        <ParentRoute path=StaticSegment(ChartRoutes::base_segment()) view=ChartsLayout>
                            <Route path=StaticSegment("") view=AreaChartPage />
                            <Route path=StaticSegment(ChartRoutes::AreaChart.as_ref()) view=AreaChartPage />
                            <Route path=StaticSegment(ChartRoutes::BarChart.as_ref()) view=BarChartPage />
                            <Route path=StaticSegment(ChartRoutes::LineChart.as_ref()) view=LineChartPage />
                            <Route path=StaticSegment(ChartRoutes::PieChart.as_ref()) view=PieChartPage />
                            <Route path=StaticSegment(ChartRoutes::RadarChart.as_ref()) view=RadarChartPage />
                            <Route path=StaticSegment(ChartRoutes::RadialChart.as_ref()) view=RadialChartPage />
                        </ParentRoute>

                        // --------- BLOCKS --------- //
                        <ParentRoute path=StaticSegment(BlockRoutes::base_segment()) view=BlocksLayout>
                            <Route path=StaticSegment("") view=LoginBlocks />
                            <Route path=StaticSegment(BlockRoutes::Login.as_ref()) view=LoginBlocks />
                            <Route path=StaticSegment(BlockRoutes::Sidenav.as_ref()) view=SidenavBlocks />
                            <Route path=StaticSegment(BlockRoutes::Headers.as_ref()) view=HeadersBlocks />
                            <Route path=StaticSegment(BlockRoutes::Footers.as_ref()) view=FootersBlocks />
                            <Route path=StaticSegment(BlockRoutes::Faq.as_ref()) view=FaqBlocks />
                            <Route path=StaticSegment(BlockRoutes::Integrations.as_ref()) view=IntegrationsBlocks />
                        </ParentRoute>

                        // --------- VIEWS --------- //
                        <ParentRoute path=StaticSegment("view") view=ViewsLayout>
                            <Route path=ParamSegment(PARAM::NAME) view=ViewRouter />
                        </ParentRoute>

                        <Sidenav01Routes />
                        <Sidenav02Routes />
                        <Sidenav03Routes />
                        <Sidenav04Routes />
                        <Sidenav05Routes />
                        <Sidenav06Routes />
                        <Sidenav07Routes />
                        <Sidenav08Routes />
                        <Sidenav09Routes />
                        <Sidenav10Routes />
                        <Sidenav11Routes />

                        // --------- OTHERS --------- //
                        <Route path=StaticSegment(RoutePaths::TEST) view=PageTest />
                        <Route path=StaticSegment(RoutePaths::TO_FIX) view=PageToFix />
                        <Route path=StaticSegment(RoutePaths::CREATE) view=PageCreate />
                        <Route path=path!("/bug-reports/d7f3a9c2e1b5") view=PageBugReports />
                    // <Route path=StaticSegment(RoutePaths::CSS_TRICKS) view=PageCssTricks />
                    // <Route path=StaticSegment(RoutePaths::TAILWIND_LATEST) view=PageTailwindLatest />
                    </Routes>
                </main>
            </AppWrapper>

            <AppBottomNav />
        </Router>
    }
}
