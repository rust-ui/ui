use leptos::prelude::*;

use crate::common::icon_registry_getter::{get_animated_icon_elements, get_icon_css};
use crate::common::icon_type::IconTypeAnimated;
use crate::svg_icon::SvgIcon;

/// Leptos component for rendering animated SVG icons. Zero-allocation O(1) lookup.
#[component]
pub fn AnimatedIcon(icon: IconTypeAnimated, #[prop(into, optional)] class: String) -> impl IntoView {
    let elements = get_animated_icon_elements(icon).unwrap_or(&[]);

    let icon_css = get_icon_css(icon);

    view! {
        {if let Some(css) = icon_css { view! { <style>{css}</style> }.into_any() } else { ().into_any() }}

        <SvgIcon class=class data_name=icon.as_ref().to_string()>
            <title>{format!("Rust/UI Animated Icons - {}", icon.as_ref())}</title>

            {elements.iter().map(|element| { element.to_leptos_view() }).collect::<Vec<_>>()}
        </SvgIcon>
    }
}
