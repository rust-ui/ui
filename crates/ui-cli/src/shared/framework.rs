use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString};

#[derive(
    Debug, Clone, Copy, Default, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Display, EnumString, AsRefStr,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum Framework {
    #[default]
    Leptos,
    Dioxus,
}

impl Framework {
    pub fn registry_base_url(self) -> &'static str {
        match self {
            Framework::Leptos => "https://www.rust-ui.com/registry",
            Framework::Dioxus => "https://dioxus.rust-ui.com/registry",
        }
    }

    pub fn site_url(self) -> &'static str {
        match self {
            Framework::Leptos => "https://www.rust-ui.com",
            Framework::Dioxus => "https://dioxus.rust-ui.com",
        }
    }
}
