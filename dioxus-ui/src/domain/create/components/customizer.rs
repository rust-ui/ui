use app_domain::themes::components::color_theme_picker::{ColorTheme, ColorThemePicker};
use app_domain::themes::components::font_picker::{FontName, FontPicker};
use app_domain::themes::components::theme_selector::CopyCodeDialog;
use app_domain::themes::theme_name::ThemeName;
use dioxus::prelude::*;
use registry::hooks::use_copy_clipboard::use_copy_clipboard;

use super::radius_picker::RadiusPicker;
// use super::theme_picker::ThemePicker;  // Base Color picker — hidden for now
use crate::domain::create::preset::encode_preset;

#[component]
pub fn Customizer(
    theme: Signal<ThemeName>,
    radius: Signal<f32>,
    color_theme: Signal<ColorTheme>,
    font: Signal<FontName>,
) -> Element {
    let css_signal = use_memo(move || theme().css_string(radius(), color_theme(), font()));
    let preset_code = use_memo(move || encode_preset(theme(), radius(), color_theme(), font()));

    let (copy_preset, preset_copied) = use_copy_clipboard(None);

    rsx! {
        aside { class: "flex sticky top-20 z-10 flex-col flex-shrink-0 gap-6 p-4 w-56 rounded-lg border border-border bg-card text-card-foreground h-fit",
            div { class: "flex flex-col gap-1",
                h2 { class: "text-sm font-semibold", "Customize" }
                p { class: "text-xs text-muted-foreground", "Pick a style and radius." }
            }

            // <ThemePicker theme=theme />  // Base Color picker — hidden for now
            ColorThemePicker { color_theme }
            RadiusPicker { radius }
            FontPicker { font }

            div { class: "flex flex-col gap-2",
                button {
                    class: "py-2 px-3 w-full font-mono text-xs text-left bg-transparent rounded-md border transition-colors border-border text-muted-foreground truncate hover:bg-accent hover:text-accent-foreground",
                    onclick: move |_| {
                        let code = preset_code();
                        copy_preset(&format!("--preset {code}"));
                    },
                    if preset_copied() {
                        "Copied!"
                    } else {
                        "--preset {preset_code()}"
                    }
                }
                CopyCodeDialog { theme: ReadSignal::from(css_signal) }
            }
        }
    }
}
