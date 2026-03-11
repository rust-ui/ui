use leptos::prelude::*;
use leptos_router::NavigateOptions;
use leptos_router::hooks::{use_location, use_navigate};
use time::Date;
use web_sys::wasm_bindgen;

pub struct QUERY;

impl QUERY {
    pub const SIZE: &str = "size";
    pub const SEARCH: &str = "search";
    pub const COLOR: &str = "color";
    pub const START_DATE: &str = "start_date";
    pub const END_DATE: &str = "end_date";
    pub const PAGE: &str = "page";
}

/* ========================================================== */
/*                       🧬 STRUCT 🧬                         */
/* ========================================================== */

pub struct QueryUtils;

impl QueryUtils {
    pub fn extract(query_key: String) -> Memo<Option<String>> {
        let location = use_location();

        Memo::new(move |_| location.query.with(|q| q.get(&query_key)))
    }

    /// Core method to handle URL updates with search parameter modifications
    fn update_url_with_params<F>(param_modifier: F)
    where
        F: FnOnce(&web_sys::UrlSearchParams),
    {
        let location = window().location();
        let Ok(current_search) = location.search() else { return };
        let navigate = use_navigate();

        // Parse existing query parameters
        let Ok(url_search_params) = web_sys::UrlSearchParams::new_with_str(&current_search) else { return };

        // Apply the parameter modifications
        param_modifier(&url_search_params);

        let Ok(pathname) = location.pathname() else { return };

        // Build new URL
        let new_url = if url_search_params.to_string().as_string().unwrap_or_default().is_empty() {
            pathname
        } else {
            format!("{}?{}", pathname, url_search_params.to_string().as_string().unwrap_or_default())
        };

        // Update browser history and navigate
        let Ok(history) = window().history() else { return };
        let _ = history.push_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(&new_url));

        let options = NavigateOptions { scroll: false, ..NavigateOptions::default() };
        navigate(&new_url, options);
    }

    pub fn update_dates_url(start: Option<Date>, end: Option<Date>) {
        Self::update_url_with_params(|params| {
            // Format: YYYY-MM-DD
            if let Some(start_date) = start {
                params.set(QUERY::START_DATE, &start_date.to_string());
            } else {
                params.delete(QUERY::START_DATE);
            }

            if let Some(end_date) = end {
                params.set(QUERY::END_DATE, &end_date.to_string());
            } else {
                params.delete(QUERY::END_DATE);
            }
        });
    }

    pub fn remove_from_url(query_key: &str) {
        Self::update_url_with_params(|params| {
            params.delete(query_key);
        });
    }

    /// Silently update a single query param using replaceState (no history entry, no router re-render).
    /// Use this for reactive UI state like theme/preset that should be bookmarkable but not navigable.
    pub fn replace_param(query_key: &str, value: &str) {
        let location = window().location();
        let Ok(current_search) = location.search() else { return };
        let Ok(params) = web_sys::UrlSearchParams::new_with_str(&current_search) else { return };
        params.set(query_key, value);
        let Ok(pathname) = location.pathname() else { return };
        let new_url = format!("{}?{}", pathname, params.to_string().as_string().unwrap_or_default());
        let Ok(history) = window().history() else { return };
        let _ = history.replace_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(&new_url));
    }
}
