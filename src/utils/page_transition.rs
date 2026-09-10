use dioxus::prelude::*;

pub const PAGE_OUTLET: &str = "page__outlet";

/// Re-plays the `page__fade` intro animation on the `#page__outlet` element.
///
/// Client-side navigation keeps the same DOM node, so the CSS animation only
/// runs on the first mount. Removing the class, forcing a reflow, then re-adding
/// it restarts the animation on every route change.
pub fn retrigger_page_fade() {
    #[cfg(target_arch = "wasm32")]
    {
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
}

/// Scrolls the main scroll container back to the top on route changes.
/// Skips blocks pages to preserve their scroll position.
#[component]
pub fn ScrollToTop() -> Element {
    // `use_route` subscribes this component to the router, so the body below
    // re-runs on every client-side navigation. The component renders nothing, so
    // running the scroll reset directly in the body (no effect) is enough.
    let _route = use_route::<crate::Route>();

    #[cfg(target_arch = "wasm32")]
    {
        use ::registry::hooks::use_data_scrolled::DATA_SCROLL_TARGET;
        use app_routes::BlockRoutes;

        let pathname = _route.to_string();
        if !pathname.starts_with(BlockRoutes::base_path())
            && let Some(window) = web_sys::window()
            && let Some(document) = window.document()
            && let Some(element) = document.get_element_by_id(DATA_SCROLL_TARGET)
        {
            element.set_scroll_top(0);
        }
    }

    rsx! {}
}
