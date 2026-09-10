use app_domain::themes::components::color_theme_picker::ColorTheme;
use app_domain::themes::components::font_picker::FontName;
use app_domain::themes::theme_name::ThemeName;
use app_domain::themes::themes_blocks::ThemesBlocks;
use dioxus::prelude::*;
use registry::hooks::use_theme_mode::use_theme_mode;

use crate::components::navigation::header_docs::HeaderDocs;
use crate::domain::create::components::customizer::Customizer;
#[cfg(target_arch = "wasm32")]
use crate::domain::create::preset::{decode_preset, encode_preset};

#[cfg(target_arch = "wasm32")]
const CSS_VAR_KEYS: &[&str] = &[
    "--background",
    "--foreground",
    "--card",
    "--card-foreground",
    "--popover",
    "--popover-foreground",
    "--primary",
    "--primary-foreground",
    "--secondary",
    "--secondary-foreground",
    "--muted",
    "--muted-foreground",
    "--accent",
    "--accent-foreground",
    "--border",
    "--input",
    "--ring",
    "--radius",
    "--font-sans",
    "--chart-1",
    "--chart-2",
    "--chart-3",
    "--chart-4",
    "--chart-5",
    "--sidebar-primary",
    "--sidebar-primary-foreground",
];

#[component]
pub fn PageCreate() -> Element {
    // Read initial state from ?preset= query param synchronously on mount.
    let (init_theme, init_radius, init_ct, init_font) = use_hook(|| {
        #[cfg(target_arch = "wasm32")]
        if let Some(search) = web_sys::window().and_then(|w| w.location().search().ok()) {
            let code =
                search.trim_start_matches('?').split('&').find_map(|p| p.strip_prefix("preset=").map(str::to_owned));
            if let Some(decoded) = code.as_deref().and_then(decode_preset) {
                return decoded;
            }
        }
        (ThemeName::default(), 0.5_f32, ColorTheme::default(), FontName::default())
    });

    let theme = use_signal(move || init_theme);
    let radius = use_signal(move || init_radius);
    let color_theme = use_signal(move || init_ct);
    let font = use_signal(move || init_font);

    let _theme_mode = use_theme_mode();

    // Remove injected CSS vars when navigating away from this page.
    use_drop(move || {
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;
            let Some(document) = web_sys::window().and_then(|w| w.document()) else { return };
            let Some(root) = document.document_element() else { return };
            let Some(el) = root.dyn_ref::<web_sys::HtmlElement>() else { return };
            let style = el.style();
            for key in CSS_VAR_KEYS {
                style.remove_property(key).ok();
            }
            root.remove_attribute("data-color-theme").ok();
        }
    });

    // Inject CSS vars on the <html> element reactively.
    use_effect(move || {
        #[cfg(target_arch = "wasm32")]
        {
            let theme_name = theme();
            let is_dark = _theme_mode.is_dark();
            let radius_val = radius();
            let color_theme_val = color_theme();
            let font_val = font();
            use wasm_bindgen::JsCast;
            let Some(document) = web_sys::window().and_then(|w| w.document()) else { return };
            let Some(root) = document.document_element() else { return };
            let Some(el) = root.dyn_ref::<web_sys::HtmlElement>() else { return };
            let style = el.style();

            // Inject base color vars first, then overlay color theme vars on top.
            let vars = if is_dark { theme_name.dark_vars() } else { theme_name.light_vars() };
            for (key, val) in vars {
                style.set_property(key, val).ok();
            }
            style.set_property("--radius", &format!("{radius_val}rem")).ok();
            style.set_property("--font-sans", font_val.css_value()).ok();

            let color_vars = if is_dark { color_theme_val.dark_vars() } else { color_theme_val.light_vars() };
            for (key, val) in color_vars {
                style.set_property(key, val).ok();
            }

            // Set data-color-theme so chart_init.js MutationObserver re-initializes ApexCharts.
            root.set_attribute("data-color-theme", color_theme_val.label()).ok();

            // Sync state to URL as ?preset=<code> (replaceState — no history entry).
            let preset = encode_preset(theme_name, radius_val, color_theme_val, font_val);
            let url = format!("/create?preset={preset}");
            if let Ok(history) = web_sys::window().and_then(|w| w.history().ok()).ok_or(()) {
                history.replace_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(&url)).ok();
            }
        }
    });

    rsx! {
        HeaderDocs {}

        div { class: "flex gap-6 p-6 mx-auto max-w-screen-2xl min-h-screen",
            Customizer { theme, radius, color_theme, font }
            div { class: "flex-1 min-w-0",
                ThemesBlocks {}
            }
        }
    }
}
