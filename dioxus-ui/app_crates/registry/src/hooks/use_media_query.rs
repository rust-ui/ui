use dioxus::prelude::*;

pub fn use_media_query(query: &str) -> ReadSignal<bool> {
    let is_match = use_signal(|| false);
    let _query = query.to_string();

    use_effect(move || {
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;

            let Some(window) = web_sys::window() else { return };
            let Ok(Some(mql)) = window.match_media(&_query) else { return };

            *is_match.write_unchecked() = mql.matches();

            let is_match_clone = is_match;
            let mql_clone = mql.clone();
            let closure = wasm_bindgen::closure::Closure::<dyn FnMut(wasm_bindgen::JsValue)>::new(
                move |_: wasm_bindgen::JsValue| {
                    *is_match_clone.write_unchecked() = mql_clone.matches();
                },
            );
            let _ = mql.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref());
            closure.forget();
        }
    });

    is_match.into()
}
