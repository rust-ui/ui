use dioxus::prelude::*;
use registry::ui::select::{Select, SelectContent, SelectGroup, SelectOption, SelectTrigger};

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
    pub const SANS: &'static [Self] = &[
        Self::Inter,
        Self::Geist,
        Self::Roboto,
        Self::NotoSans,
        Self::DmSans,
        Self::NunitoSans,
        Self::Raleway,
        Self::Outfit,
        Self::Figtree,
        Self::PublicSans,
    ];

    pub const MONO: &'static [Self] = &[Self::JetBrainsMono, Self::GeistMono];

    pub const SERIF: &'static [Self] = &[
        Self::Lora,
        Self::Merriweather,
        Self::PlayfairDisplay,
        Self::NotoSerif,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            Self::Inter => "Inter",
            Self::Geist => "Geist",
            Self::Roboto => "Roboto",
            Self::NotoSans => "Noto Sans",
            Self::DmSans => "DM Sans",
            Self::NunitoSans => "Nunito Sans",
            Self::Raleway => "Raleway",
            Self::Outfit => "Outfit",
            Self::Figtree => "Figtree",
            Self::PublicSans => "Public Sans",
            Self::JetBrainsMono => "JetBrains Mono",
            Self::GeistMono => "Geist Mono",
            Self::Lora => "Lora",
            Self::Merriweather => "Merriweather",
            Self::PlayfairDisplay => "Playfair Display",
            Self::NotoSerif => "Noto Serif",
        }
    }

    pub fn css_value(&self) -> &'static str {
        match self {
            Self::Inter => "'Inter', sans-serif",
            Self::Geist => "'Geist', sans-serif",
            Self::Roboto => "'Roboto', sans-serif",
            Self::NotoSans => "'Noto Sans', sans-serif",
            Self::DmSans => "'DM Sans', sans-serif",
            Self::NunitoSans => "'Nunito Sans', sans-serif",
            Self::Raleway => "'Raleway', sans-serif",
            Self::Outfit => "'Outfit', sans-serif",
            Self::Figtree => "'Figtree', sans-serif",
            Self::PublicSans => "'Public Sans', sans-serif",
            Self::JetBrainsMono => "'JetBrains Mono', monospace",
            Self::GeistMono => "'Geist Mono', monospace",
            Self::Lora => "'Lora', serif",
            Self::Merriweather => "'Merriweather', serif",
            Self::PlayfairDisplay => "'Playfair Display', serif",
            Self::NotoSerif => "'Noto Serif', serif",
        }
    }

    pub fn from_label(label: &str) -> Option<Self> {
        match label {
            "Inter" => Some(Self::Inter),
            "Geist" => Some(Self::Geist),
            "Roboto" => Some(Self::Roboto),
            "Noto Sans" => Some(Self::NotoSans),
            "DM Sans" => Some(Self::DmSans),
            "Nunito Sans" => Some(Self::NunitoSans),
            "Raleway" => Some(Self::Raleway),
            "Outfit" => Some(Self::Outfit),
            "Figtree" => Some(Self::Figtree),
            "Public Sans" => Some(Self::PublicSans),
            "JetBrains Mono" => Some(Self::JetBrainsMono),
            "Geist Mono" => Some(Self::GeistMono),
            "Lora" => Some(Self::Lora),
            "Merriweather" => Some(Self::Merriweather),
            "Playfair Display" => Some(Self::PlayfairDisplay),
            "Noto Serif" => Some(Self::NotoSerif),
            _ => None,
        }
    }

    pub fn to_index(self) -> usize {
        match self {
            Self::Inter => 0,
            Self::Geist => 1,
            Self::Roboto => 2,
            Self::NotoSans => 3,
            Self::DmSans => 4,
            Self::NunitoSans => 5,
            Self::Raleway => 6,
            Self::Outfit => 7,
            Self::Figtree => 8,
            Self::PublicSans => 9,
            Self::JetBrainsMono => 10,
            Self::GeistMono => 11,
            Self::Lora => 12,
            Self::Merriweather => 13,
            Self::PlayfairDisplay => 14,
            Self::NotoSerif => 15,
        }
    }

    pub fn from_index(idx: u32) -> Option<Self> {
        match idx {
            0 => Some(Self::Inter),
            1 => Some(Self::Geist),
            2 => Some(Self::Roboto),
            3 => Some(Self::NotoSans),
            4 => Some(Self::DmSans),
            5 => Some(Self::NunitoSans),
            6 => Some(Self::Raleway),
            7 => Some(Self::Outfit),
            8 => Some(Self::Figtree),
            9 => Some(Self::PublicSans),
            10 => Some(Self::JetBrainsMono),
            11 => Some(Self::GeistMono),
            12 => Some(Self::Lora),
            13 => Some(Self::Merriweather),
            14 => Some(Self::PlayfairDisplay),
            15 => Some(Self::NotoSerif),
            _ => None,
        }
    }
}

/* ========================================================== */
/*                     ✨ COMPONENT ✨                        */
/* ========================================================== */

#[component]
pub fn FontPicker(mut font: Signal<FontName>) -> Element {
    rsx! {
        Select {
            class: "w-full",
            default_value: font().label().to_string(),
            on_change: move |val: Option<String>| {
                if let Some(f) = val.as_deref().and_then(FontName::from_label) {
                    font.set(f);
                }
            },
            SelectTrigger {
                span { class: "text-sm text-muted-foreground", "Font" }
                span { class: "mr-auto ml-2 text-sm font-medium", "{font().label()}" }
                span {
                    class: "flex-shrink-0 mr-1 text-sm pointer-events-none select-none text-foreground",
                    style: "font-family:{font().css_value()}",
                    "Aa"
                }
            }
            SelectContent { class: "w-full",
                SelectGroup {
                    div { class: "px-2 py-1.5 text-xs font-semibold text-muted-foreground", "Sans" }
                    {FontName::SANS.iter().map(|&f| rsx! {
                        SelectOption { key: "{f.label()}", value: f.label(), "{f.label()}" }
                    })}
                }
                SelectGroup {
                    div { class: "px-2 py-1.5 text-xs font-semibold text-muted-foreground", "Mono" }
                    {FontName::MONO.iter().map(|&f| rsx! {
                        SelectOption { key: "{f.label()}", value: f.label(), "{f.label()}" }
                    })}
                }
                SelectGroup {
                    div { class: "px-2 py-1.5 text-xs font-semibold text-muted-foreground", "Serif" }
                    {FontName::SERIF.iter().map(|&f| rsx! {
                        SelectOption { key: "{f.label()}", value: f.label(), "{f.label()}" }
                    })}
                }
            }
        }
    }
}
