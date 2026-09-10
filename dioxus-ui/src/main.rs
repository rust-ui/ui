use ::registry::hooks::use_theme_mode::ThemeMode;
use ::registry::ui::toast_custom::toaster::{Toaster, provide_toaster};
use dioxus::prelude::*;

pub mod __registry__;
pub mod components;
mod domain;
pub mod markdown;
pub mod registry;
mod routes;
pub mod utils;

use domain::blocks::routing::blocks_layout::BlocksLayout;
use domain::blocks::routing::blocks_pages::{
    FaqBlocks, FootersBlocks, HeadersBlocks, IntegrationsBlocks, LoginBlocks, SidenavBlocks,
};
use domain::blocks::routing::sidenav_demo_layout::{SidenavDemoLayout, SidenavInsetRightLayout};
use domain::blocks::routing::sidenav_demo_pages::{
    SidenavDemoComponentPage, SidenavDemoComponents, SidenavDemoDocs, SidenavDemoHookPage, SidenavDemoHooks,
};
use domain::bug_report::page_bug_reports::PageBugReports;
use domain::charts::routing::charts_layout::ChartsLayout;
use domain::charts::routing::charts_pages::{
    AreaChartPage, BarChartPage, LineChartPage, PieChartPage, RadarChartPage, RadialChartPage,
};
use domain::create::page_create::PageCreate;
use domain::test::routing::test_layout::TestLayout;
use domain::test::routing::test_pages::TestPage;
use domain::views::view_router::ViewRouter;
use domain::workflows::routing::workflow_view_page::WorkflowViewPage;
use domain::workflows::routing::workflows_layout::WorkflowsLayout;
use domain::workflows::routing::workflows_pages::WorkflowsPage;
use routes::app_layout::AppLayout;
use routes::component_page::ComponentPage;
use routes::docs_index_page::{DocsComponentsIndexPage, DocsHooksIndexPage};
use routes::docs_layout::DocsLayout;
use routes::home_layout::HomeLayout;
use routes::home_page::Home;
use routes::hook_page::HookPage;
use routes::page_download::PageDownload;
use routes::page_icons::PageIcons;
use routes::page_not_found::PageNotFound;

const FAVICON: Asset = asset!("/public/favicon.ico");
const FAVICON_16: Asset = asset!("/public/icons/favicon-16x16.png");
const FAVICON_32: Asset = asset!("/public/icons/favicon-32x32.png");
const APPLE_TOUCH_ICON: Asset = asset!("/public/icons/apple-touch-icon.png");
const MANIFEST: Asset = asset!("/public/manifest.json");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const CHART_INIT_JS: Asset = asset!("/public/app_components/chart_init.js");
const RESIZABLE_JS: Asset = asset!("/public/app_components/resizable.js");

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(AppLayout)]
        #[layout(HomeLayout)]
            #[route("/")]
            Home {},
        #[end_layout]
        #[layout(DocsLayout)]
            #[redirect("/docs", || Route::DocsComponentsIndexPage {})]
            #[redirect("/components", || Route::DocsComponentsIndexPage {})]
            #[redirect("/components/:name", |name: String| Route::ComponentPage { name })]
            #[redirect("/hooks", || Route::DocsHooksIndexPage {})]
            #[redirect("/hooks/:name", |name: String| Route::HookPage { name })]
            #[route("/docs/components")]
            DocsComponentsIndexPage {},
            #[route("/docs/components/:name")]
            ComponentPage { name: String },
            #[route("/docs/hooks")]
            DocsHooksIndexPage {},
            #[route("/docs/hooks/:name")]
            HookPage { name: String },
        #[end_layout]
        #[layout(BlocksLayout)]
            #[redirect("/blocks", || Route::LoginBlocks {})]
            #[route("/blocks/login")]
            LoginBlocks {},
            #[route("/blocks/sidenav")]
            SidenavBlocks {},
            #[route("/blocks/headers")]
            HeadersBlocks {},
            #[route("/blocks/footers")]
            FootersBlocks {},
            #[route("/blocks/faq")]
            FaqBlocks {},
            #[route("/blocks/integrations")]
            IntegrationsBlocks {},
        #[end_layout]
        #[layout(ChartsLayout)]
            #[redirect("/charts", || Route::AreaChartPage {})]
            #[route("/charts/area-chart")]
            AreaChartPage {},
            #[route("/charts/bar-chart")]
            BarChartPage {},
            #[route("/charts/line-chart")]
            LineChartPage {},
            #[route("/charts/pie-chart")]
            PieChartPage {},
            #[route("/charts/radar-chart")]
            RadarChartPage {},
            #[route("/charts/radial-chart")]
            RadialChartPage {},
        #[end_layout]
        #[layout(WorkflowsLayout)]
            #[route("/workflows")]
            WorkflowsPage {},
        #[end_layout]
        #[layout(TestLayout)]
            #[route("/test-page")]
            TestPage {},
        #[end_layout]
        #[route("/icons")]
        PageIcons {},
        #[route("/download")]
        PageDownload {},
        #[route("/create")]
        PageCreate {},
        #[route("/bug-reports/d7f3a9c2e1b5")]
        PageBugReports {},
    #[end_layout]
    #[layout(SidenavDemoLayout)]
        #[layout(SidenavInsetRightLayout)]
            #[route("/view/:sidenav/docs")]
            SidenavDemoDocs { sidenav: String },
            #[route("/view/:sidenav/docs/components")]
            SidenavDemoComponents { sidenav: String },
            #[route("/view/:sidenav/docs/components/:name")]
            SidenavDemoComponentPage { sidenav: String, name: String },
            #[route("/view/:sidenav/docs/hooks")]
            SidenavDemoHooks { sidenav: String },
            #[route("/view/:sidenav/docs/hooks/:name")]
            SidenavDemoHookPage { sidenav: String, name: String },
        #[end_layout]
    #[end_layout]
    #[route("/view/:id")]
    WorkflowViewPage { id: String },
    #[route("/view/block/:id")]
    ViewRouter { id: String },
    #[route("/:..segments")]
    PageNotFound { segments: Vec<String> },
}

fn main() {
    // Web/wasm build (and any non-server build): the stock launch path. The
    // client-side `Router::<Route>` handles every URL once the page is loaded,
    // so the bug documented on `mod server` below never manifests here.
    #[cfg(not(feature = "server"))]
    dioxus::launch(App);

    // Server build: use our own axum router instead of the one
    // `dioxus::launch` would build. `dioxus::serve` is the documented 0.7
    // extension point for this ("Serve a fullstack dioxus application with a
    // custom axum router"). The closure is called once per (re)build so
    // hot-patching still swaps the router in dev.
    #[cfg(feature = "server")]
    dioxus::serve(|| async { Ok(server::router()) });
}

/// Server bootstrap. Hand-rolled replacement for `dioxus::launch`'s server
/// router.
///
/// ## The bug this works around
///
/// Symptom: in-app SPA navigation to `/docs/components/alert` (or any
/// `/docs/*` / `/icons` page) works, but a **hard refresh / direct hit** of the
/// same URL returns HTTP 404. `/components/alert`, `/create`, `/charts/*` etc.
/// are fine. Dev (`dx serve`) and any served-SSR deployment are affected;
/// a fully pre-rendered static export is not (each route is a real file on
/// disk there).
///
/// Root cause: `dioxus-server` 0.7.10 `DioxusRouterExt::serve_static_assets`
/// -> `serve_dir_cached` (see
/// `~/.cargo/registry/.../dioxus-server-0.7.10/src/server.rs`). For every
/// top-level entry in `public/` it does, in debug builds:
///
/// ```ignore
/// router = router.nest_service(&route, ServeDir::new(&path));
/// ```
///
/// `nest_service("/docs", ...)` makes axum hand the **entire `/docs/*`
/// subtree** to that `ServeDir`, and `ServeDir` has no not-found service, so a
/// path with no matching file (`public/docs/components/alert` does not exist,
/// only `alert.md` does) terminates with 404. The request never reaches
/// `.fallback(render_handler)`, i.e. the SSR renderer. Our routes
/// `/docs/components/*`, `/docs/hooks/*` and `/icons` collide with the
/// real directories `public/docs/` and `public/icons/` and are therefore
/// unreachable on a cold request.
///
/// Why the client router still works: it never issues these HTTP requests, it
/// resolves `Route` in-memory.
///
/// Why not just move the directories: `public/docs/*.md` is intentionally
/// served as raw markdown (LLM/tooling consumers, parity with `leptos-ui`),
/// and `public/icons/*` holds real favicons/logos referenced by URL. Renaming
/// them only relocates the same collision and diverges from `leptos-ui`.
///
/// ## The fix
///
/// Build the router so `ServeDir` is the **fallback**, not a set of nested
/// services: a real file under `public/` still wins, everything else falls
/// through to the Dioxus SSR handler. This is the exact shape `leptos_axum`
/// uses (`ServeDir::new(dir).fallback(handler)`), which is why the Leptos site
/// with the identical `public/` layout does not have this bug.
///
/// Trade-off: we duplicate the small amount of glue
/// `serve_dioxus_application` normally does (server-fn registration + SSR
/// fallback + state) and skip `apply_base_path` (this deployment configures no
/// base path). Revisit / delete this module if `dioxus-server` gains a
/// not-found fallback on its static handler upstream.
///
/// See `BUGFIX_docs_routes_404_on_refresh.md` for the full write-up.
#[cfg(feature = "server")]
mod server {
    use dioxus::server::axum::Router;
    use dioxus::server::axum::routing::get;
    use dioxus::server::{DioxusRouterExt, FullstackState, ServeConfig};
    use tower_http::services::ServeDir;

    use super::App;

    /// Directory the CLI bundles static assets into. Mirrors the private
    /// `dioxus_server::public_path()`: honour `DIOXUS_PUBLIC_PATH` if set,
    /// otherwise `<exe dir>/public` (what `dx serve` and the prod binary use).
    fn public_path() -> std::path::PathBuf {
        if let Ok(path) = std::env::var("DIOXUS_PUBLIC_PATH") {
            return path.into();
        }
        std::env::current_exe().expect("current_exe").parent().expect("exe has a parent directory").join("public")
    }

    /// Equivalent of `Router::new().serve_dioxus_application(cfg, App)` but with
    /// static-file serving as a fallback instead of per-directory
    /// `nest_service`s. Order matters:
    ///   1. `register_server_functions()`: POST `/api/*` handlers.
    ///   2. `fallback_service(ServeDir.fallback(ssr))`: try a real file under
    ///      `public/`, and on miss render the route server-side. This is the
    ///      line that fixes the `/docs/*` and `/icons` 404-on-refresh.
    ///   3. `with_state(state)`: supply `FullstackState` to the server fns.
    pub fn router() -> Router {
        let cfg = ServeConfig::new();
        let state = FullstackState::new(cfg, App);

        // SSR renderer as a leaf service, with its own state applied so it can
        // stand alone as `ServeDir`'s not-found target.
        let ssr = get(dioxus::server::render_handler).with_state(state.clone());

        // `append_index_html_on_directories(false)`: without it, a request whose
        // path maps to a real directory under `public/` (`/docs`, `/docs/components`,
        // `/docs/hooks`, `/icons` all have a matching dir) gets a 307 to the
        // trailing-slash form before falling through. Disabling it makes `ServeDir`
        // return "not found" for a bare directory hit, so those paths fall straight
        // to the SSR handler and render at their canonical (no trailing slash) URL.
        let static_files = ServeDir::new(public_path()).append_index_html_on_directories(false).fallback(ssr);

        Router::new().register_server_functions().fallback_service(static_files).with_state(state)
    }
}

#[component]
fn App() -> Element {
    #[cfg(target_arch = "wasm32")]
    ::registry::hooks::use_scroll_lock::init();

    #[cfg(target_arch = "wasm32")]
    utils::client_diagnostic_handler::init();

    let theme_mode = ThemeMode::init();
    provide_toaster();

    // TODO: replace with <Html class=...> once Dioxus supports reactive html-element attributes (like leptos_meta `<Html {..} class=...>`)
    use_effect(move || {
        let is_dark = theme_mode.is_dark();
        spawn(async move {
            let js = if is_dark {
                "document.documentElement.classList.add('dark');"
            } else {
                "document.documentElement.classList.remove('dark');"
            };
            dioxus::document::eval(js).await.ok();
        });
    });

    rsx! {
        document::Title { "Rust/UI" }
        // TODO: Dioxus injects a <div id="main"> between <body> and the App component.
        // Leptos doesn't have this wrapper, so the h-full chain works natively:
        //   html(100dvh) → body(h-full) → AppWrapper div(h-full) → main(flex-1 overflow-y-auto)
        // In Dioxus the chain is broken because #main has no height, so flex-1 on <main> collapses.
        // Ideal fix: find a way to pass h-full to the Dioxus mount div without a style tag.
        // For now we inject it inline so it wins over any stylesheet ordering issues.
        document::Style { "#main {{ height: 100%; }}" }
        // Page intro fade, replayed on every route change via `retrigger_page_fade`.
        // `backwards` keeps the from-state applied during the pre-animation delay.
        document::Style {
            "@keyframes page__fade_in{{from{{opacity:0;transform:translateY(6px)}}to{{opacity:1}}}} .page__fade{{animation:page__fade_in 200ms ease-out backwards}}"
        }
        document::Link { rel: "icon", r#type: "image/png", sizes: "32x32", href: FAVICON_32 }
        document::Link { rel: "icon", r#type: "image/png", sizes: "16x16", href: FAVICON_16 }
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "apple-touch-icon", href: APPLE_TOUCH_ICON }
        document::Link { rel: "manifest", href: MANIFEST }
        document::Stylesheet { href: TAILWIND_CSS }
        document::Script { src: CHART_INIT_JS }
        document::Script { src: RESIZABLE_JS }
        Toaster {}
        Router::<Route> {}
    }
}
