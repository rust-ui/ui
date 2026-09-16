#![cfg_attr(not(target_arch = "wasm32"), allow(clippy::missing_const_for_fn))]

use dioxus::prelude::*;

pub const PAGE_QUERY_KEY: &str = "page";
const FIRST_PAGE: u32 = 1;

#[derive(Clone, Copy)]
pub struct PaginationContext {
    pub current_page: Signal<u32>,
    pub page_href: Callback<u32, String>,
    pub prev_href: Memo<String>,
    pub next_href: Memo<String>,
    pub is_first_page: Memo<bool>,
    pub aria_current: Callback<u32, &'static str>,
    pub go_to_page: Callback<u32>,
}

fn get_page_from_query() -> u32 {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(search) = window.location().search() {
                let search = search.trim_start_matches('?');
                for pair in search.split('&') {
                    let mut parts = pair.splitn(2, '=');
                    if let (Some(key), Some(val)) = (parts.next(), parts.next()) {
                        if key == PAGE_QUERY_KEY {
                            if let Ok(n) = val.parse::<u32>() {
                                return n;
                            }
                        }
                    }
                }
            }
        }
    }
    FIRST_PAGE
}

fn build_page_href(page: u32) -> String {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(search) = window.location().search() {
                let search = search.trim_start_matches('?');
                let mut params: Vec<String> = search
                    .split('&')
                    .filter(|s| !s.is_empty())
                    .filter(|pair| {
                        let key = pair.split('=').next().unwrap_or("");
                        key != PAGE_QUERY_KEY
                    })
                    .map(|s| s.to_string())
                    .collect();
                params.push(format!("{}={}", PAGE_QUERY_KEY, page));
                return format!("?{}", params.join("&"));
            }
        }
    }
    format!("?{PAGE_QUERY_KEY}={page}")
}

#[cfg(target_arch = "wasm32")]
fn push_page_to_history(href: &str) {
    if let Some(window) = web_sys::window() {
        if let Ok(history) = window.history() {
            let _ = history.push_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(href));
        }
    }
}

pub fn use_pagination() -> PaginationContext {
    let mut current_page = use_signal(get_page_from_query);

    let page_href = Callback::new(move |page: u32| build_page_href(page));

    let prev_href = use_memo(move || {
        let current = current_page();
        if current > FIRST_PAGE { page_href.call(current - 1) } else { "#".to_string() }
    });

    let next_href = use_memo(move || page_href.call(current_page() + 1));

    let is_first_page = use_memo(move || current_page() <= FIRST_PAGE);

    let aria_current = Callback::new(move |page: u32| if current_page() == page { PAGE_QUERY_KEY } else { "" });

    let go_to_page = Callback::new(move |page: u32| {
        #[cfg(target_arch = "wasm32")]
        push_page_to_history(&page_href.call(page));
        current_page.set(page);
    });

    PaginationContext { current_page, page_href, prev_href, next_href, is_first_page, aria_current, go_to_page }
}
