use app_config::{SeoMeta, SiteConfig};
use leptos::prelude::*;
use leptos_router::hooks::use_location;
use registry::hooks::use_theme_mode::use_theme_mode;
use registry::utils::query::QueryUtils;
use wasm_bindgen::JsCast;

use crate::components::navigation::header_docs::HeaderDocs;
use crate::domain::create::components::color_theme_picker::ColorTheme;
use crate::domain::create::components::customizer::Customizer;
use crate::domain::create::components::font_picker::FontName;
use crate::domain::create::components::theme_picker::ThemeName;
use crate::domain::create::preset::{decode_preset, encode_preset};
use crate::domain::themes::themes_blocks::ThemesBlocks;

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
pub fn PageCreate() -> impl IntoView {
    let title = format!("Create Theme · {}", SiteConfig::TITLE);
    let description =
        "Customize your Leptos Rust UI theme with preset color palettes and border radius. Live preview with instant CSS variable injection."
            .to_string();
    let canonical_url = format!("{}/create", SiteConfig::BASE_URL);
    let og_title = "Create Theme · RUST-UI Theme Customizer".to_string();

    // Read initial state from ?preset= query param (works on server + client).
    let location = use_location();
    let (init_theme, init_radius, init_color_theme, init_font) = location.query.with_untracked(|q| {
        q.get("preset").and_then(|code| decode_preset(&code)).unwrap_or((
            ThemeName::default(),
            0.5_f32,
            ColorTheme::default(),
            FontName::default(),
        ))
    });

    let theme = RwSignal::new(init_theme);
    let radius = RwSignal::new(init_radius);
    let color_theme = RwSignal::new(init_color_theme);
    let font = RwSignal::new(init_font);

    let theme_mode = use_theme_mode();

    // Inject CSS vars on the <html> element reactively.
    // on_cleanup is nested inside Effect::new so it only ever runs on the client.
    Effect::new(move |_| {
        let theme_name = theme.get();
        let radius_val = radius.get();
        let color_theme_val = color_theme.get();
        let font_val = font.get();
        let is_dark = theme_mode.is_dark();

        // Clean up injected vars when leaving this page (or before Effect re-runs).
        // Safe: on_cleanup nested inside Effect::new is client-only.
        on_cleanup(move || {
            let Some(document) = window().document() else { return };
            let Some(root) = document.document_element() else { return };
            let Some(el) = root.dyn_ref::<web_sys::HtmlElement>() else { return };
            let style = el.style();
            for key in CSS_VAR_KEYS {
                style.remove_property(key).ok();
            }
            root.remove_attribute("data-color-theme").ok();
        });

        let Some(document) = window().document() else { return };
        let Some(root) = document.document_element() else { return };
        let Some(el) = root.dyn_ref::<web_sys::HtmlElement>() else { return };
        let style = el.style();

        // Inject base color vars first, then overlay color theme vars on top.
        let vars = if is_dark { theme_name.dark_vars() } else { theme_name.light_vars() };
        for (key, val) in vars {
            style.set_property(key, val).ok();
        }
        style.set_property("--radius", &format!("{radius_val}rem")).ok();

        // Inject font.
        style.set_property("--font-sans", font_val.css_value()).ok();

        let color_vars = if is_dark { color_theme_val.dark_vars() } else { color_theme_val.light_vars() };
        for (key, val) in color_vars {
            style.set_property(key, val).ok();
        }

        // Set data-color-theme on <html> so chart_init.js MutationObserver detects the change
        // and re-initializes ApexCharts with the new --chart-1..5 CSS vars.
        root.set_attribute("data-color-theme", color_theme_val.label()).ok();

        // Sync state to URL as ?preset=<code> (replaceState — no history entry).
        QueryUtils::replace_param("preset", &encode_preset(theme_name, radius_val, color_theme_val, font_val));
    });

    view! {
        <SeoMeta title=title description=description canonical_url=canonical_url og_title=og_title />

        <HeaderDocs />

        <div class="flex gap-6 p-6 mx-auto max-w-screen-2xl min-h-screen">
            <Customizer theme=theme radius=radius color_theme=color_theme font=font />
            <div class="flex-1 min-w-0">
                <ThemesBlocks />
            </div>
        </div>
    }
}
