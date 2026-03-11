use std::str::FromStr;

use app_config::SiteConfig;
use app_domain::utils::PARAM;
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_params_map;

use crate::__registry__::all_blocks::BlockIdKebab;
use crate::routes::page_not_found::PageNotFound;

#[component]
pub fn ViewRouter() -> impl IntoView {
    let params = use_params_map();

    view! {
        {move || {
            let view_name = params.get().get(PARAM::NAME).unwrap_or_default();
            match BlockIdKebab::from_str(&view_name) {
                Ok(view_type) => {
                    let title = format!("{} · {}", view_type.to_title(), SiteConfig::TITLE);
                    let component = view_type.to_component();
                    let meta = view_type.meta();
                    view! {
                        <section data-name="__ViewRouter" class=meta.container_class>
                            <Title text=title />
                            {component}
                        </section>
                    }
                        .into_any()
                }
                Err(_) => view! { <PageNotFound /> }.into_any(),
            }
        }}
    }
}
