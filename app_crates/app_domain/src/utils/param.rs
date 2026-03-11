use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use leptos_router::params::Params;

#[derive(Params, PartialEq, Clone)]
pub struct ParamsUtils {}

impl ParamsUtils {
    pub fn extract(param: &'static str) -> Memo<String> {
        let params = use_params_map();
        Memo::new(move |_| params.with(|p| p.get(param).unwrap_or_default()))
    }
}

pub struct PARAM {}

impl PARAM {
    pub const NAME: &str = "name";
}
