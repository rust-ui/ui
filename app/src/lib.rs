#![recursion_limit = "256"]
pub mod __registry__;
pub mod components;
pub mod domain;
pub mod registry;
pub mod routes;
pub mod sidenav_shortfix_all_blocks;
pub mod utils;

pub mod app;
pub mod shell;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::App;
    // initializes logging using the `log` crate
    _ = console_log::init_with_level(log::Level::Debug);
    utils::client_diagnostic_handler::init();

    leptos::mount::hydrate_body(App);
}
