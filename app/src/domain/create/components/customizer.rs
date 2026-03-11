use app_domain::themes::components::theme_selector::CopyCodeDialog;
use leptos::prelude::*;
use registry::hooks::use_copy_clipboard::use_copy_clipboard;

use super::color_theme_picker::{ColorTheme, ColorThemePicker};
use super::font_picker::{FontName, FontPicker};
use super::radius_picker::RadiusPicker;
// use super::theme_picker::{ThemeName, ThemePicker};  // Base Color picker — hidden for now
use super::theme_picker::ThemeName;
use crate::domain::create::preset::encode_preset;

#[component]
pub fn Customizer(
    theme: RwSignal<ThemeName>,
    radius: RwSignal<f32>,
    color_theme: RwSignal<ColorTheme>,
    font: RwSignal<FontName>,
) -> impl IntoView {
    let css_signal = Signal::derive(move || theme.get().css_string(radius.get(), color_theme.get(), font.get()));
    let preset_code = Signal::derive(move || encode_preset(theme.get(), radius.get(), color_theme.get(), font.get()));

    let (copy_preset, preset_copied) = use_copy_clipboard(None);

    view! {
        <aside class="flex sticky top-20 z-10 flex-col flex-shrink-0 gap-6 p-4 w-56 rounded-lg border border-border bg-card text-card-foreground h-fit">
            <div class="flex flex-col gap-1">
                <h2 class="text-sm font-semibold">"Customize"</h2>
                <p class="text-xs text-muted-foreground">"Pick a style and radius."</p>
            </div>

            // <ThemePicker theme=theme />  // Base Color picker — hidden for now
            <ColorThemePicker color_theme=color_theme />
            <RadiusPicker radius=radius />
            <FontPicker font=font />

            <div class="flex flex-col gap-2">
                <button
                    class="py-2 px-3 w-full font-mono text-xs text-left bg-transparent rounded-md border transition-colors border-border text-muted-foreground truncate hover:bg-accent hover:text-accent-foreground"
                    on:click=move |_| {
                        let code = preset_code.get();
                        copy_preset(&format!("--preset {code}"));
                    }
                >
                    {move || {
                        if preset_copied.get() {
                            "Copied!".to_string()
                        } else {
                            format!("--preset {}", preset_code.get())
                        }
                    }}
                </button>
                <CopyCodeDialog theme=css_signal />
            </div>
        </aside>
    }
}
