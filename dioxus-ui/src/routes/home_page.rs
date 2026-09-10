use app_config::SeoMeta;
use app_domain::constants::route_paths::RoutePaths;
use app_domain::themes::components::color_theme_picker::{ColorTheme, ColorThemePicker};
use app_domain::themes::components::theme_selector::CopyCodeDialog;
use app_domain::themes::theme_name::ThemeName;
use app_domain::themes::themes_blocks::ThemesBlocks;
use dioxus::prelude::*;
use icons::Download;
use registry::hooks::use_theme_mode::use_theme_mode;
use registry::ui::button::{Button, ButtonSize, ButtonVariant};

use crate::components::app_footer::AppFooter;
use crate::components::logos::ferris::Ferris;
use crate::routes::home_page_sparkles::{
    SparklesColor, SparklesDescription, SparklesEffect, SparklesHeader, SparklesSection,
};

#[component]
pub fn Home() -> Element {
    let title = "Dioxus Components · Rust UI Component Library | Rust/UI".to_string();
    let description = "Beautiful Rust UI components for Dioxus applications. Cross-platform component library for modern fullstack web apps - build once, deploy everywhere.".to_string();

    let color_theme = use_signal(ColorTheme::default);
    let theme_mode = use_theme_mode();

    let css_signal = use_memo(move || ThemeName::default().css_string(0.5, color_theme(), Default::default()));

    use_effect(move || {
        let ct = color_theme();
        let is_dark = theme_mode.is_dark();

        let vars: Vec<(&'static str, &'static str)> =
            if is_dark { ct.dark_vars().to_vec() } else { ct.light_vars().to_vec() };
        let label = ct.label();

        spawn(async move {
            let mut remove_js = String::from("(function() { var el = document.documentElement;");
            for key in ColorTheme::KEYS {
                remove_js.push_str(&format!("el.style.removeProperty('{}');", key));
            }
            remove_js.push_str("})();");
            dioxus::document::eval(&remove_js).await.ok();

            let mut set_js = String::from("(function() { var el = document.documentElement;");
            for (key, val) in &vars {
                set_js.push_str(&format!("el.style.setProperty('{}', '{}');", key, val));
            }
            set_js.push_str(&format!("el.setAttribute('data-color-theme', '{}');", label));
            set_js.push_str("})();");
            dioxus::document::eval(&set_js).await.ok();
        });
    });

    rsx! {
        SeoMeta {
            title: title,
            description: description,
        }

        div { class: "flex flex-col gap-6 items-center px-4 mx-auto w-full max-w-[1200px]",
            SectionHeader {}

            div { class: "flex gap-2 items-center self-end",
                div { class: "w-46",
                    ColorThemePicker { color_theme }
                }
                CopyCodeDialog {
                    theme: ReadSignal::from(css_signal),
                    trigger_variant: ButtonVariant::Outline,
                    trigger_size: ButtonSize::Icon,
                    icons::Copy {}
                }
            }

            ThemesBlocks {}
        }

        AppFooter {}
    }
}

#[component]
fn SectionHeader() -> Element {
    rsx! {
        SparklesSection { class: "max-w-7xl",
            SparklesEffect { color: SparklesColor::Orange,
                div {}
            }

            SparklesHeader { class: "-mt-[230px]",
                Ferris { width: 150, height: 150 }
                h1 { class: "text-3xl font-bold text-center lg:text-4xl text-pretty",
                    "Build once, run Everywhere."
                }
                SparklesDescription {
                    "Rust/UI is a cross-platform component registry for Dioxus and Rust fullstack applications. Build your UI once and deploy to iOS, Android, Desktop, and Web."
                }

                div { class: "flex flex-wrap gap-4 justify-center mt-4",
                    Button {
                        href: "/docs/components",
                        variant: ButtonVariant::Outline,
                        "Browse Components"
                    }
                    Button {
                        href: RoutePaths::DOWNLOAD,
                        Download {}
                        span { "Download Desktop" }
                    }
                }
            }
        }
    }
}
