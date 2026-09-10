use dioxus::prelude::*;

use crate::domain::settings::page_settings::SettingsPage;

pub struct SettingsRoutes;

impl SettingsRoutes {
    pub const LABEL: &'static str = "Settings";
}

#[component]
pub fn Settings() -> Element {
    rsx! { SettingsPage {} }
}
