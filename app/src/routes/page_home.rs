use app_config::SeoMeta;
// use crate::components::newsletter_signup::NewsletterSignup;
use app_domain::constants::RoutePaths;
use app_domain::themes::components::theme_selector::CopyCodeDialog;
use app_routes::ComponentsRoutes;
use icons::{Copy, Download};
use leptos::prelude::*;
use registry::hooks::use_theme_mode::use_theme_mode;
use registry::ui::button::{Button, ButtonSize, ButtonVariant};
use wasm_bindgen::JsCast;

use crate::components::app_footer::AppFooter;
use crate::components::app_header::AppHeader;
use crate::components::logos::_ferris::Ferris;
use crate::domain::create::components::color_theme_picker::{ColorTheme, ColorThemePicker};
use crate::domain::create::components::theme_picker::ThemeName;
use crate::domain::themes::themes_blocks::ThemesBlocks;
use crate::routes::page_home_sparkles::{
    SparklesColor, SparklesDescription, SparklesEffect, SparklesHeader, SparklesSection,
};

#[component]
pub fn PageHome() -> impl IntoView {
    let title = "Leptos Components · Rust UI Component Library | Rust/UI".to_string();
    let description = "Beautiful Rust UI components for Leptos applications. Cross-platform component library for modern fullstack web apps - build once, deploy everywhere.".to_string();

    let color_theme = RwSignal::new(ColorTheme::default());
    let theme_mode = use_theme_mode();
    let css_signal =
        Signal::derive(move || ThemeName::default().css_string(0.5, color_theme.get(), Default::default()));

    Effect::new(move |_| {
        let ct = color_theme.get();
        let is_dark = theme_mode.is_dark();

        on_cleanup(move || {
            let Some(document) = window().document() else { return };
            let Some(root) = document.document_element() else { return };
            let Some(el) = root.dyn_ref::<web_sys::HtmlElement>() else { return };
            let style = el.style();
            for key in ColorTheme::KEYS {
                style.remove_property(key).ok();
            }
            root.remove_attribute("data-color-theme").ok();
        });

        let Some(document) = window().document() else { return };
        let Some(root) = document.document_element() else { return };
        let Some(el) = root.dyn_ref::<web_sys::HtmlElement>() else { return };
        let style = el.style();

        for key in ColorTheme::KEYS {
            style.remove_property(key).ok();
        }
        let vars = if is_dark { ct.dark_vars() } else { ct.light_vars() };
        for (key, val) in vars {
            style.set_property(key, val).ok();
        }
        root.set_attribute("data-color-theme", ct.label()).ok();
    });

    view! {
        <SeoMeta title=title description=description />

        <AppHeader />

        <div class="flex flex-col gap-6 items-center px-4 mx-auto w-full max-w-[1200px]">
            <SectionHeader />

            <div class="flex gap-2 items-center self-end">
                <div class="w-46">
                    <ColorThemePicker color_theme=color_theme />
                </div>
                <CopyCodeDialog theme=css_signal trigger_variant=ButtonVariant::Outline trigger_size=ButtonSize::Icon>
                    <Copy />
                </CopyCodeDialog>
            </div>

            <ThemesBlocks />
        </div>

        // TODO. This seems to trigger …cloudflare-static/email-decode.min.js(rust-ui.com) - 271 ms, 1,04 KiB
        // TODO. └─> PageSpeed Insights
        // <div class="px-4 mx-auto mt-14 w-full max-w-[1200px]">
        // <NewsletterSignup />
        // </div>

        <AppFooter />
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
fn SectionHeader() -> impl IntoView {
    view! {
        <SparklesSection class="max-w-7xl">
            <SparklesEffect color=SparklesColor::Orange>
                <div />
            </SparklesEffect>

            <SparklesHeader class="-mt-[230px]">
                <Ferris width=150 height=150 />
                <h1 class="text-3xl font-bold text-center lg:text-4xl text-pretty">"Build once, run Everywhere."</h1>
                <SparklesDescription>
                    "Rust/UI is a cross-platform component registry for Leptos and Rust fullstack applications. Build your UI once and deploy to iOS, Android, Desktop, and Web."
                </SparklesDescription>

                <div class="flex flex-wrap gap-4 justify-center mt-4">
                    <Button href=ComponentsRoutes::base_url() variant=ButtonVariant::Outline>
                        "Browse Components"
                    </Button>
                    <Button href=RoutePaths::DOWNLOAD>
                        <Download />
                        <span>"Download Desktop"</span>
                    </Button>
                </div>
            </SparklesHeader>
        </SparklesSection>
    }
}
