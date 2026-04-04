use app_routes::BlockRoutes;
use leptos::prelude::*;
use leptos_router::hooks::use_location;
use registry::hooks::use_data_scrolled::DATA_SCROLL_TARGET;

pub const PAGE_OUTLET: &str = "page__outlet";

pub fn retrigger_page_fade() {
    if let Some(window) = web_sys::window()
        && let Some(document) = window.document()
        && let Some(el) = document.get_element_by_id(PAGE_OUTLET)
    {
        let cl = el.class_list();
        cl.remove_1("page__fade").ok();
        let _ = el.get_bounding_client_rect();
        cl.add_1("page__fade").ok();
    }
}

/// Scrolls the main scroll container to top on route changes.
/// Skips blocks pages to preserve scroll position.
#[component]
pub fn ScrollToTop() -> impl IntoView {
    let location = use_location();

    Effect::new(move |_| {
        let pathname = location.pathname.get();

        if pathname.starts_with(BlockRoutes::base_path()) {
            return;
        }

        if let Some(window) = web_sys::window()
            && let Some(document) = window.document()
            && let Some(element) = document.get_element_by_id(DATA_SCROLL_TARGET)
        {
            element.set_scroll_top(0);
        }
    });
}
