use leptos::prelude::*;
use registry::ui::select::{Select, SelectContent, SelectGroup, SelectLabel, SelectOption, SelectTrigger};

/* ========================================================== */
/*                       ✨ TYPES ✨                          */
/* ========================================================== */

#[derive(Clone, Copy, PartialEq, Default)]
pub enum FontName {
    // Sans (indices 0-9)
    #[default]
    Inter,
    Geist,
    Roboto,
    NotoSans,
    DmSans,
    NunitoSans,
    Raleway,
    Outfit,
    Figtree,
    PublicSans,
    // Mono (indices 10-11)
    JetBrainsMono,
    GeistMono,
    // Serif (indices 12-15)
    Lora,
    Merriweather,
    PlayfairDisplay,
    NotoSerif,
}

impl FontName {
    pub const SANS: &'static [FontName] = &[
        FontName::Inter,
        FontName::Geist,
        FontName::Roboto,
        FontName::NotoSans,
        FontName::DmSans,
        FontName::NunitoSans,
        FontName::Raleway,
        FontName::Outfit,
        FontName::Figtree,
        FontName::PublicSans,
    ];

    pub const MONO: &'static [FontName] = &[FontName::JetBrainsMono, FontName::GeistMono];

    pub const SERIF: &'static [FontName] =
        &[FontName::Lora, FontName::Merriweather, FontName::PlayfairDisplay, FontName::NotoSerif];

    pub fn label(&self) -> &'static str {
        match self {
            FontName::Inter => "Inter",
            FontName::Geist => "Geist",
            FontName::Roboto => "Roboto",
            FontName::NotoSans => "Noto Sans",
            FontName::DmSans => "DM Sans",
            FontName::NunitoSans => "Nunito Sans",
            FontName::Raleway => "Raleway",
            FontName::Outfit => "Outfit",
            FontName::Figtree => "Figtree",
            FontName::PublicSans => "Public Sans",
            FontName::JetBrainsMono => "JetBrains Mono",
            FontName::GeistMono => "Geist Mono",
            FontName::Lora => "Lora",
            FontName::Merriweather => "Merriweather",
            FontName::PlayfairDisplay => "Playfair Display",
            FontName::NotoSerif => "Noto Serif",
        }
    }

    /// The CSS font-family value to set on --font-sans.
    pub fn css_value(&self) -> &'static str {
        match self {
            FontName::Inter => "'Inter', sans-serif",
            FontName::Geist => "'Geist', sans-serif",
            FontName::Roboto => "'Roboto', sans-serif",
            FontName::NotoSans => "'Noto Sans', sans-serif",
            FontName::DmSans => "'DM Sans', sans-serif",
            FontName::NunitoSans => "'Nunito Sans', sans-serif",
            FontName::Raleway => "'Raleway', sans-serif",
            FontName::Outfit => "'Outfit', sans-serif",
            FontName::Figtree => "'Figtree', sans-serif",
            FontName::PublicSans => "'Public Sans', sans-serif",
            FontName::JetBrainsMono => "'JetBrains Mono', monospace",
            FontName::GeistMono => "'Geist Mono', monospace",
            FontName::Lora => "'Lora', serif",
            FontName::Merriweather => "'Merriweather', serif",
            FontName::PlayfairDisplay => "'Playfair Display', serif",
            FontName::NotoSerif => "'Noto Serif', serif",
        }
    }

    pub fn from_label(label: &str) -> Option<Self> {
        match label {
            "Inter" => Some(FontName::Inter),
            "Geist" => Some(FontName::Geist),
            "Roboto" => Some(FontName::Roboto),
            "Noto Sans" => Some(FontName::NotoSans),
            "DM Sans" => Some(FontName::DmSans),
            "Nunito Sans" => Some(FontName::NunitoSans),
            "Raleway" => Some(FontName::Raleway),
            "Outfit" => Some(FontName::Outfit),
            "Figtree" => Some(FontName::Figtree),
            "Public Sans" => Some(FontName::PublicSans),
            "JetBrains Mono" => Some(FontName::JetBrainsMono),
            "Geist Mono" => Some(FontName::GeistMono),
            "Lora" => Some(FontName::Lora),
            "Merriweather" => Some(FontName::Merriweather),
            "Playfair Display" => Some(FontName::PlayfairDisplay),
            "Noto Serif" => Some(FontName::NotoSerif),
            _ => None,
        }
    }

    pub fn to_index(self) -> usize {
        match self {
            FontName::Inter => 0,
            FontName::Geist => 1,
            FontName::Roboto => 2,
            FontName::NotoSans => 3,
            FontName::DmSans => 4,
            FontName::NunitoSans => 5,
            FontName::Raleway => 6,
            FontName::Outfit => 7,
            FontName::Figtree => 8,
            FontName::PublicSans => 9,
            FontName::JetBrainsMono => 10,
            FontName::GeistMono => 11,
            FontName::Lora => 12,
            FontName::Merriweather => 13,
            FontName::PlayfairDisplay => 14,
            FontName::NotoSerif => 15,
        }
    }

    pub fn from_index(idx: u32) -> Option<Self> {
        match idx {
            0 => Some(FontName::Inter),
            1 => Some(FontName::Geist),
            2 => Some(FontName::Roboto),
            3 => Some(FontName::NotoSans),
            4 => Some(FontName::DmSans),
            5 => Some(FontName::NunitoSans),
            6 => Some(FontName::Raleway),
            7 => Some(FontName::Outfit),
            8 => Some(FontName::Figtree),
            9 => Some(FontName::PublicSans),
            10 => Some(FontName::JetBrainsMono),
            11 => Some(FontName::GeistMono),
            12 => Some(FontName::Lora),
            13 => Some(FontName::Merriweather),
            14 => Some(FontName::PlayfairDisplay),
            15 => Some(FontName::NotoSerif),
            _ => None,
        }
    }
}

/* ========================================================== */
/*                     ✨ COMPONENT ✨                        */
/* ========================================================== */

#[component]
pub fn FontPicker(font: RwSignal<FontName>) -> impl IntoView {
    view! {
        <Select
            class="w-full"
            default_value=font.get_untracked().label().to_string()
            on_change=Callback::new(move |val: Option<String>| {
                if let Some(f) = val.as_deref().and_then(FontName::from_label) {
                    font.set(f);
                }
            })
        >
            <SelectTrigger>
                <span class="text-sm text-muted-foreground">"Font"</span>
                <span class="mr-auto ml-2 text-sm font-medium">{move || font.get().label()}</span>
                // "Aa" preview rendered in the selected font
                <span
                    class="flex-shrink-0 mr-1 text-sm pointer-events-none select-none text-foreground"
                    style=move || format!("font-family:{}", font.get().css_value())
                >
                    "Aa"
                </span>
            </SelectTrigger>

            <SelectContent class="w-full">
                <SelectGroup>
                    <SelectLabel>"Sans"</SelectLabel>
                    {FontName::SANS
                        .iter()
                        .map(|&f| {
                            view! { <SelectOption value=f.label().to_string()>{f.label()}</SelectOption> }
                        })
                        .collect_view()}
                </SelectGroup>
                <SelectGroup>
                    <SelectLabel>"Mono"</SelectLabel>
                    {FontName::MONO
                        .iter()
                        .map(|&f| {
                            view! { <SelectOption value=f.label().to_string()>{f.label()}</SelectOption> }
                        })
                        .collect_view()}
                </SelectGroup>
                <SelectGroup>
                    <SelectLabel>"Serif"</SelectLabel>
                    {FontName::SERIF
                        .iter()
                        .map(|&f| {
                            view! { <SelectOption value=f.label().to_string()>{f.label()}</SelectOption> }
                        })
                        .collect_view()}
                </SelectGroup>
            </SelectContent>
        </Select>
    }
}
