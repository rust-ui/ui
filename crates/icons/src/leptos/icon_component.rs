use leptos::prelude::*;

use super::svg_icon::SvgIcon;
use crate::common::icon_registry_getter::get_icon_elements;
use crate::common::icon_type::IconType;

/// Single optimized Icon component that renders any icon via registry lookup
#[component]
pub fn LeptosIcon(icon: IconType, #[prop(into, optional)] class: String) -> impl IntoView {
    // Get elements from static registry, zero allocation
    let elements = get_icon_elements(icon).unwrap_or(&[]);

    view! {
        <SvgIcon class=class data_name=icon.to_string()>
            <title>{format!("Rust/UI Icons - {icon}")}</title>

            {elements.iter().map(|element| { element.to_leptos_view() }).collect::<Vec<_>>()}
        </SvgIcon>
    }
}
