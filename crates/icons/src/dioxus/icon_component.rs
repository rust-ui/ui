use dioxus::prelude::*;

use super::svg_icon::SvgIcon;
use crate::common::icon_registry_getter::get_icon_elements;
use crate::common::icon_type::IconType;

/// Single optimized Icon component that renders any icon via registry lookup
#[component]
pub fn DioxusIcon(icon: IconType, class: Option<String>) -> Element {
    // Get elements from static registry, zero allocation
    let elements = get_icon_elements(icon).unwrap_or(&[]);

    rsx! {
        SvgIcon { class, data_name: "{icon}",
            title { "Rust/UI Icons - {icon}" }

            for element in elements.iter() {
                {element.to_dioxus_element()}
            }
        }
    }
}
