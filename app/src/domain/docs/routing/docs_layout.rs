use std::sync::Arc;

use app_domain::utils::{PARAM, ParamsUtils};
use app_routes::{ComponentsRoutes, HooksRoutes};
use leptos::prelude::*;
use leptos_router::components::Outlet;
use leptos_router::hooks::use_location;

use crate::__registry__::demos_sidenav::{get_all_arc_demos_for_components, get_all_arc_demos_for_hooks};
use crate::components::navigation::header_docs::HeaderDocs;
use crate::components::shared_sidenav_demos::SharedSidenavDemos;
use crate::components::table_of_contents::{TableOfContents, TocItem};

#[derive(Clone)]
pub struct DocsTocContext {
    pub toc_items: RwSignal<Vec<TocItem>>,
}

#[component]
pub fn DocsLayout() -> impl IntoView {
    let location = use_location();
    let params_demo_name = ParamsUtils::extract(PARAM::NAME);
    let params_demo_name_arc = Arc::new(params_demo_name);

    let toc_items = RwSignal::new(Vec::<TocItem>::new());
    let toc_context = DocsTocContext { toc_items };
    provide_context(toc_context.clone());

    // Create a memo for section detection that's reactive to location changes
    let section_data = Memo::new(move |_| {
        let pathname = location.pathname.get();

        if pathname.contains("/hooks/") || pathname.ends_with("/hooks") {
            (Arc::new(HooksRoutes::base_url().to_string()), get_all_arc_demos_for_hooks())
        } else {
            // Default to components (includes both /docs/components and fallback)
            (Arc::new(ComponentsRoutes::base_url().to_string()), get_all_arc_demos_for_components())
        }
    });

    view! {
        <HeaderDocs />

        <section data-name="__MainContainer" class="flex-1">
            <div class="container flex items-start">
                {move || {
                    let (base_path, all_arc_demos) = section_data.get();
                    view! {
                        <SharedSidenavDemos
                            base_path=base_path
                            params_demo_name=Arc::clone(&params_demo_name_arc)
                            all_arc_demos=all_arc_demos
                        />
                    }
                }} // TOC Sidenav (right side)
                <Outlet />
                {move || {
                    view! { <TableOfContents toc_items=toc_context.toc_items.get() /> }
                }}
            </div>
        </section>
    }
}
