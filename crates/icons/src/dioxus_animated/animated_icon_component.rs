use dioxus::prelude::*;

use crate::common::icon_registry_getter::{get_animated_icon_elements, get_icon_css};
use crate::common::icon_type::IconTypeAnimated;
use crate::dioxus::svg_icon::SvgIcon;

#[component]
pub fn AnimatedIcon(icon: IconTypeAnimated, class: Option<String>) -> Element {
    let elements = get_animated_icon_elements(icon).unwrap_or(&[]);
    let icon_css = get_icon_css(icon);
    let icon_name = icon.as_ref().to_string();

    rsx! {
        if let Some(css) = icon_css {
            style { "{css}" }
        }
        SvgIcon { class, data_name: icon_name.clone(),
            title { "Rust/UI Animated Icons - {icon_name}" }
            for element in elements.iter() {
                {element.to_dioxus_element()}
            }
        }
    }
}
