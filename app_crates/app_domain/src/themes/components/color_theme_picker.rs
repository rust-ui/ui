use dioxus::prelude::*;
use registry::ui::select::{Select, SelectContent, SelectGroup, SelectOption, SelectTrigger};

#[derive(Clone, Copy, PartialEq, Eq, Default, strum::IntoStaticStr)]
pub enum ColorTheme {
    #[default]
    #[strum(serialize = "Default")]
    None,
    Amber,
    Blue,
    Cyan,
    Emerald,
    Fuchsia,
    Green,
    Indigo,
    Lime,
    Orange,
    Pink,
    Purple,
    Red,
    Rose,
    Sky,
    Teal,
    Violet,
    Yellow,
}

impl ColorTheme {
    pub const ALL: &'static [Self] = &[
        Self::None,
        Self::Amber,
        Self::Blue,
        Self::Cyan,
        Self::Emerald,
        Self::Fuchsia,
        Self::Green,
        Self::Indigo,
        Self::Lime,
        Self::Orange,
        Self::Pink,
        Self::Purple,
        Self::Red,
        Self::Rose,
        Self::Sky,
        Self::Teal,
        Self::Violet,
        Self::Yellow,
    ];

    pub const KEYS: &'static [&'static str] = &[
        "--primary",
        "--primary-foreground",
        "--secondary",
        "--secondary-foreground",
        "--chart-1",
        "--chart-2",
        "--chart-3",
        "--chart-4",
        "--chart-5",
        "--sidebar-primary",
        "--sidebar-primary-foreground",
    ];

    pub fn label(self) -> &'static str {
        self.into()
    }

    pub fn swatch(self) -> &'static str {
        match self {
            Self::None => "#d4d4d4",
            Self::Amber => "#f59e0b",
            Self::Blue => "#3b82f6",
            Self::Cyan => "#06b6d4",
            Self::Emerald => "#10b981",
            Self::Fuchsia => "#d946ef",
            Self::Green => "#22c55e",
            Self::Indigo => "#6366f1",
            Self::Lime => "#84cc16",
            Self::Orange => "#f97316",
            Self::Pink => "#ec4899",
            Self::Purple => "#a855f7",
            Self::Red => "#ef4444",
            Self::Rose => "#f43f5e",
            Self::Sky => "#0ea5e9",
            Self::Teal => "#14b8a6",
            Self::Violet => "#8b5cf6",
            Self::Yellow => "#eab308",
        }
    }

    pub fn to_index(self) -> usize {
        match self {
            Self::None => 0,
            Self::Amber => 1,
            Self::Blue => 2,
            Self::Cyan => 3,
            Self::Emerald => 4,
            Self::Fuchsia => 5,
            Self::Green => 6,
            Self::Indigo => 7,
            Self::Lime => 8,
            Self::Orange => 9,
            Self::Pink => 10,
            Self::Purple => 11,
            Self::Red => 12,
            Self::Rose => 13,
            Self::Sky => 14,
            Self::Teal => 15,
            Self::Violet => 16,
            Self::Yellow => 17,
        }
    }

    pub fn from_index(idx: u32) -> Option<Self> {
        match idx {
            0 => Some(Self::None),
            1 => Some(Self::Amber),
            2 => Some(Self::Blue),
            3 => Some(Self::Cyan),
            4 => Some(Self::Emerald),
            5 => Some(Self::Fuchsia),
            6 => Some(Self::Green),
            7 => Some(Self::Indigo),
            8 => Some(Self::Lime),
            9 => Some(Self::Orange),
            10 => Some(Self::Pink),
            11 => Some(Self::Purple),
            12 => Some(Self::Red),
            13 => Some(Self::Rose),
            14 => Some(Self::Sky),
            15 => Some(Self::Teal),
            16 => Some(Self::Violet),
            17 => Some(Self::Yellow),
            _ => None,
        }
    }

    pub fn from_label(label: &str) -> Option<Self> {
        Self::ALL.iter().copied().find(|ct| ct.label() == label)
    }

    pub fn light_vars(self) -> &'static [(&'static str, &'static str)] {
        match self {
            Self::None => &[],
            Self::Amber => AMBER_LIGHT,
            Self::Blue => BLUE_LIGHT,
            Self::Cyan => CYAN_LIGHT,
            Self::Emerald => EMERALD_LIGHT,
            Self::Fuchsia => FUCHSIA_LIGHT,
            Self::Green => GREEN_LIGHT,
            Self::Indigo => INDIGO_LIGHT,
            Self::Lime => LIME_LIGHT,
            Self::Orange => ORANGE_LIGHT,
            Self::Pink => PINK_LIGHT,
            Self::Purple => PURPLE_LIGHT,
            Self::Red => RED_LIGHT,
            Self::Rose => ROSE_LIGHT,
            Self::Sky => SKY_LIGHT,
            Self::Teal => TEAL_LIGHT,
            Self::Violet => VIOLET_LIGHT,
            Self::Yellow => YELLOW_LIGHT,
        }
    }

    pub fn dark_vars(self) -> &'static [(&'static str, &'static str)] {
        match self {
            Self::None => &[],
            Self::Amber => AMBER_DARK,
            Self::Blue => BLUE_DARK,
            Self::Cyan => CYAN_DARK,
            Self::Emerald => EMERALD_DARK,
            Self::Fuchsia => FUCHSIA_DARK,
            Self::Green => GREEN_DARK,
            Self::Indigo => INDIGO_DARK,
            Self::Lime => LIME_DARK,
            Self::Orange => ORANGE_DARK,
            Self::Pink => PINK_DARK,
            Self::Purple => PURPLE_DARK,
            Self::Red => RED_DARK,
            Self::Rose => ROSE_DARK,
            Self::Sky => SKY_DARK,
            Self::Teal => TEAL_DARK,
            Self::Violet => VIOLET_DARK,
            Self::Yellow => YELLOW_DARK,
        }
    }
}

/* ========================================================== */
/*                     ✨ COMPONENT ✨                        */
/* ========================================================== */

#[component]
pub fn ColorThemePicker(mut color_theme: Signal<ColorTheme>) -> Element {
    rsx! {
        Select {
            class: "w-full",
            default_value: color_theme().label().to_string(),
            on_change: move |val: Option<String>| {
                if let Some(ct) = val.as_deref().and_then(ColorTheme::from_label) {
                    color_theme.set(ct);
                }
            },
            SelectTrigger {
                span { class: "text-sm text-muted-foreground", "Theme" }
                span { class: "mr-auto ml-2 text-sm font-medium", "{color_theme().label()}" }
                span {
                    class: "flex-shrink-0 mr-1 rounded-full border size-4 border-border/50",
                    style: "background-color:{color_theme().swatch()}"
                }
            }
            SelectContent { class: "w-full",
                SelectGroup {
                    {ColorTheme::ALL.iter().map(|ct| {
                        let label = ct.label();
                        let swatch = ct.swatch();
                        rsx! {
                            SelectOption { key: "{label}", value: label,
                                span {
                                    class: "flex-shrink-0 rounded-full border size-3.5 border-border/50",
                                    style: "background-color:{swatch}"
                                }
                                "{label}"
                            }
                        }
                    })}
                }
            }
        }
    }
}

/* ========================================================== */
/*                    ✨ THEME DATA ✨                        */
/* ========================================================== */

static AMBER_LIGHT: &[(&str, &str)] = &[
    ("--primary", "oklch(0.555 0.163 48.998)"),
    ("--primary-foreground", "oklch(0.987 0.022 95.277)"),
    ("--secondary", "oklch(0.967 0.001 286.375)"),
    ("--secondary-foreground", "oklch(0.21 0.006 285.885)"),
    ("--chart-1", "oklch(0.879 0.169 91.605)"),
    ("--chart-2", "oklch(0.769 0.188 70.08)"),
    ("--chart-3", "oklch(0.666 0.179 58.318)"),
    ("--chart-4", "oklch(0.555 0.163 48.998)"),
    ("--chart-5", "oklch(0.473 0.137 46.201)"),
    ("--sidebar-primary", "oklch(0.666 0.179 58.318)"),
    ("--sidebar-primary-foreground", "oklch(0.987 0.022 95.277)"),
];
static AMBER_DARK: &[(&str, &str)] = &[
    ("--primary", "oklch(0.473 0.137 46.201)"),
    ("--primary-foreground", "oklch(0.987 0.022 95.277)"),
    ("--secondary", "oklch(0.274 0.006 286.033)"),
    ("--secondary-foreground", "oklch(0.985 0 0)"),
    ("--chart-1", "oklch(0.879 0.169 91.605)"),
    ("--chart-2", "oklch(0.769 0.188 70.08)"),
    ("--chart-3", "oklch(0.666 0.179 58.318)"),
    ("--chart-4", "oklch(0.555 0.163 48.998)"),
    ("--chart-5", "oklch(0.473 0.137 46.201)"),
    ("--sidebar-primary", "oklch(0.769 0.188 70.08)"),
    ("--sidebar-primary-foreground", "oklch(0.279 0.077 45.635)"),
];
static BLUE_LIGHT: &[(&str, &str)] = &[
    ("--primary", "oklch(0.488 0.243 264.376)"),
    ("--primary-foreground", "oklch(0.97 0.014 254.604)"),
    ("--secondary", "oklch(0.967 0.001 286.375)"),
    ("--secondary-foreground", "oklch(0.21 0.006 285.885)"),
    ("--chart-1", "oklch(0.809 0.105 251.813)"),
    ("--chart-2", "oklch(0.623 0.214 259.815)"),
    ("--chart-3", "oklch(0.546 0.245 262.881)"),
    ("--chart-4", "oklch(0.488 0.243 264.376)"),
    ("--chart-5", "oklch(0.424 0.199 265.638)"),
    ("--sidebar-primary", "oklch(0.546 0.245 262.881)"),
    ("--sidebar-primary-foreground", "oklch(0.97 0.014 254.604)"),
];
static BLUE_DARK: &[(&str, &str)] = &[
    ("--primary", "oklch(0.424 0.199 265.638)"),
    ("--primary-foreground", "oklch(0.97 0.014 254.604)"),
    ("--secondary", "oklch(0.274 0.006 286.033)"),
    ("--secondary-foreground", "oklch(0.985 0 0)"),
    ("--chart-1", "oklch(0.809 0.105 251.813)"),
    ("--chart-2", "oklch(0.623 0.214 259.815)"),
    ("--chart-3", "oklch(0.546 0.245 262.881)"),
    ("--chart-4", "oklch(0.488 0.243 264.376)"),
    ("--chart-5", "oklch(0.424 0.199 265.638)"),
    ("--sidebar-primary", "oklch(0.623 0.214 259.815)"),
    ("--sidebar-primary-foreground", "oklch(0.97 0.014 254.604)"),
];
static CYAN_LIGHT: &[(&str, &str)] = &[
    ("--primary", "oklch(0.52 0.105 223.128)"),
    ("--primary-foreground", "oklch(0.984 0.019 200.873)"),
    ("--secondary", "oklch(0.967 0.001 286.375)"),
    ("--secondary-foreground", "oklch(0.21 0.006 285.885)"),
    ("--chart-1", "oklch(0.865 0.127 207.078)"),
    ("--chart-2", "oklch(0.715 0.143 215.221)"),
    ("--chart-3", "oklch(0.609 0.126 221.723)"),
    ("--chart-4", "oklch(0.52 0.105 223.128)"),
    ("--chart-5", "oklch(0.45 0.085 224.283)"),
    ("--sidebar-primary", "oklch(0.609 0.126 221.723)"),
    ("--sidebar-primary-foreground", "oklch(0.984 0.019 200.873)"),
];
static CYAN_DARK: &[(&str, &str)] = &[
    ("--primary", "oklch(0.45 0.085 224.283)"),
    ("--primary-foreground", "oklch(0.984 0.019 200.873)"),
    ("--secondary", "oklch(0.274 0.006 286.033)"),
    ("--secondary-foreground", "oklch(0.985 0 0)"),
    ("--chart-1", "oklch(0.865 0.127 207.078)"),
    ("--chart-2", "oklch(0.715 0.143 215.221)"),
    ("--chart-3", "oklch(0.609 0.126 221.723)"),
    ("--chart-4", "oklch(0.52 0.105 223.128)"),
    ("--chart-5", "oklch(0.45 0.085 224.283)"),
    ("--sidebar-primary", "oklch(0.715 0.143 215.221)"),
    ("--sidebar-primary-foreground", "oklch(0.302 0.056 229.695)"),
];
static EMERALD_LIGHT: &[(&str, &str)] = &[
    ("--primary", "oklch(0.508 0.118 165.612)"),
    ("--primary-foreground", "oklch(0.979 0.021 166.113)"),
    ("--secondary", "oklch(0.967 0.001 286.375)"),
    ("--secondary-foreground", "oklch(0.21 0.006 285.885)"),
    ("--chart-1", "oklch(0.845 0.143 164.978)"),
    ("--chart-2", "oklch(0.696 0.17 162.48)"),
    ("--chart-3", "oklch(0.596 0.145 163.225)"),
    ("--chart-4", "oklch(0.508 0.118 165.612)"),
    ("--chart-5", "oklch(0.432 0.095 166.913)"),
    ("--sidebar-primary", "oklch(0.596 0.145 163.225)"),
    ("--sidebar-primary-foreground", "oklch(0.979 0.021 166.113)"),
];
static EMERALD_DARK: &[(&str, &str)] = &[
    ("--primary", "oklch(0.432 0.095 166.913)"),
    ("--primary-foreground", "oklch(0.979 0.021 166.113)"),
    ("--secondary", "oklch(0.274 0.006 286.033)"),
    ("--secondary-foreground", "oklch(0.985 0 0)"),
    ("--chart-1", "oklch(0.845 0.143 164.978)"),
    ("--chart-2", "oklch(0.696 0.17 162.48)"),
    ("--chart-3", "oklch(0.596 0.145 163.225)"),
    ("--chart-4", "oklch(0.508 0.118 165.612)"),
    ("--chart-5", "oklch(0.432 0.095 166.913)"),
    ("--sidebar-primary", "oklch(0.696 0.17 162.48)"),
    ("--sidebar-primary-foreground", "oklch(0.262 0.051 172.552)"),
];
static FUCHSIA_LIGHT: &[(&str, &str)] = &[
    ("--primary", "oklch(0.518 0.253 323.949)"),
    ("--primary-foreground", "oklch(0.977 0.017 320.058)"),
    ("--secondary", "oklch(0.967 0.001 286.375)"),
    ("--secondary-foreground", "oklch(0.21 0.006 285.885)"),
    ("--chart-1", "oklch(0.833 0.145 321.434)"),
    ("--chart-2", "oklch(0.667 0.295 322.15)"),
    ("--chart-3", "oklch(0.591 0.293 322.896)"),
    ("--chart-4", "oklch(0.518 0.253 323.949)"),
    ("--chart-5", "oklch(0.452 0.211 324.591)"),
    ("--sidebar-primary", "oklch(0.591 0.293 322.896)"),
    ("--sidebar-primary-foreground", "oklch(0.977 0.017 320.058)"),
];
static FUCHSIA_DARK: &[(&str, &str)] = &[
    ("--primary", "oklch(0.452 0.211 324.591)"),
    ("--primary-foreground", "oklch(0.977 0.017 320.058)"),
    ("--secondary", "oklch(0.274 0.006 286.033)"),
    ("--secondary-foreground", "oklch(0.985 0 0)"),
    ("--chart-1", "oklch(0.833 0.145 321.434)"),
    ("--chart-2", "oklch(0.667 0.295 322.15)"),
    ("--chart-3", "oklch(0.591 0.293 322.896)"),
    ("--chart-4", "oklch(0.518 0.253 323.949)"),
    ("--chart-5", "oklch(0.452 0.211 324.591)"),
    ("--sidebar-primary", "oklch(0.667 0.295 322.15)"),
    ("--sidebar-primary-foreground", "oklch(0.977 0.017 320.058)"),
];
static GREEN_LIGHT: &[(&str, &str)] = &[
    ("--primary", "oklch(0.532 0.157 131.589)"),
    ("--primary-foreground", "oklch(0.986 0.031 120.757)"),
    ("--secondary", "oklch(0.967 0.001 286.375)"),
    ("--secondary-foreground", "oklch(0.21 0.006 285.885)"),
    ("--chart-1", "oklch(0.871 0.15 154.449)"),
    ("--chart-2", "oklch(0.723 0.219 149.579)"),
    ("--chart-3", "oklch(0.627 0.194 149.214)"),
    ("--chart-4", "oklch(0.527 0.154 150.069)"),
    ("--chart-5", "oklch(0.448 0.119 151.328)"),
    ("--sidebar-primary", "oklch(0.648 0.2 131.684)"),
    ("--sidebar-primary-foreground", "oklch(0.986 0.031 120.757)"),
];
static GREEN_DARK: &[(&str, &str)] = &[
    ("--primary", "oklch(0.453 0.124 130.933)"),
    ("--primary-foreground", "oklch(0.986 0.031 120.757)"),
    ("--secondary", "oklch(0.274 0.006 286.033)"),
    ("--secondary-foreground", "oklch(0.985 0 0)"),
    ("--chart-1", "oklch(0.871 0.15 154.449)"),
    ("--chart-2", "oklch(0.723 0.219 149.579)"),
    ("--chart-3", "oklch(0.627 0.194 149.214)"),
    ("--chart-4", "oklch(0.527 0.154 150.069)"),
    ("--chart-5", "oklch(0.448 0.119 151.328)"),
    ("--sidebar-primary", "oklch(0.768 0.233 130.85)"),
    ("--sidebar-primary-foreground", "oklch(0.986 0.031 120.757)"),
];
static INDIGO_LIGHT: &[(&str, &str)] = &[
    ("--primary", "oklch(0.457 0.24 277.023)"),
    ("--primary-foreground", "oklch(0.962 0.018 272.314)"),
    ("--secondary", "oklch(0.967 0.001 286.375)"),
    ("--secondary-foreground", "oklch(0.21 0.006 285.885)"),
    ("--chart-1", "oklch(0.785 0.115 274.713)"),
    ("--chart-2", "oklch(0.585 0.233 277.117)"),
    ("--chart-3", "oklch(0.511 0.262 276.966)"),
    ("--chart-4", "oklch(0.457 0.24 277.023)"),
    ("--chart-5", "oklch(0.398 0.195 277.366)"),
    ("--sidebar-primary", "oklch(0.511 0.262 276.966)"),
    ("--sidebar-primary-foreground", "oklch(0.962 0.018 272.314)"),
];
static INDIGO_DARK: &[(&str, &str)] = &[
    ("--primary", "oklch(0.398 0.195 277.366)"),
    ("--primary-foreground", "oklch(0.962 0.018 272.314)"),
    ("--secondary", "oklch(0.274 0.006 286.033)"),
    ("--secondary-foreground", "oklch(0.985 0 0)"),
    ("--chart-1", "oklch(0.785 0.115 274.713)"),
    ("--chart-2", "oklch(0.585 0.233 277.117)"),
    ("--chart-3", "oklch(0.511 0.262 276.966)"),
    ("--chart-4", "oklch(0.457 0.24 277.023)"),
    ("--chart-5", "oklch(0.398 0.195 277.366)"),
    ("--sidebar-primary", "oklch(0.585 0.233 277.117)"),
    ("--sidebar-primary-foreground", "oklch(0.962 0.018 272.314)"),
];
static LIME_LIGHT: &[(&str, &str)] = &[
    ("--primary", "oklch(0.532 0.157 131.589)"),
    ("--primary-foreground", "oklch(0.986 0.031 120.757)"),
    ("--secondary", "oklch(0.967 0.001 286.375)"),
    ("--secondary-foreground", "oklch(0.21 0.006 285.885)"),
    ("--chart-1", "oklch(0.897 0.196 126.665)"),
    ("--chart-2", "oklch(0.768 0.233 130.85)"),
    ("--chart-3", "oklch(0.648 0.2 131.684)"),
    ("--chart-4", "oklch(0.532 0.157 131.589)"),
    ("--chart-5", "oklch(0.453 0.124 130.933)"),
    ("--sidebar-primary", "oklch(0.648 0.2 131.684)"),
    ("--sidebar-primary-foreground", "oklch(0.986 0.031 120.757)"),
];
static LIME_DARK: &[(&str, &str)] = &[
    ("--primary", "oklch(0.453 0.124 130.933)"),
    ("--primary-foreground", "oklch(0.986 0.031 120.757)"),
    ("--secondary", "oklch(0.274 0.006 286.033)"),
    ("--secondary-foreground", "oklch(0.985 0 0)"),
    ("--chart-1", "oklch(0.897 0.196 126.665)"),
    ("--chart-2", "oklch(0.768 0.233 130.85)"),
    ("--chart-3", "oklch(0.648 0.2 131.684)"),
    ("--chart-4", "oklch(0.532 0.157 131.589)"),
    ("--chart-5", "oklch(0.453 0.124 130.933)"),
    ("--sidebar-primary", "oklch(0.768 0.233 130.85)"),
    ("--sidebar-primary-foreground", "oklch(0.274 0.072 132.109)"),
];
static ORANGE_LIGHT: &[(&str, &str)] = &[
    ("--primary", "oklch(0.553 0.195 38.402)"),
    ("--primary-foreground", "oklch(0.98 0.016 73.684)"),
    ("--secondary", "oklch(0.967 0.001 286.375)"),
    ("--secondary-foreground", "oklch(0.21 0.006 285.885)"),
    ("--chart-1", "oklch(0.837 0.128 66.29)"),
    ("--chart-2", "oklch(0.705 0.213 47.604)"),
    ("--chart-3", "oklch(0.646 0.222 41.116)"),
    ("--chart-4", "oklch(0.553 0.195 38.402)"),
    ("--chart-5", "oklch(0.47 0.157 37.304)"),
    ("--sidebar-primary", "oklch(0.646 0.222 41.116)"),
    ("--sidebar-primary-foreground", "oklch(0.98 0.016 73.684)"),
];
static ORANGE_DARK: &[(&str, &str)] = &[
    ("--primary", "oklch(0.47 0.157 37.304)"),
    ("--primary-foreground", "oklch(0.98 0.016 73.684)"),
    ("--secondary", "oklch(0.274 0.006 286.033)"),
    ("--secondary-foreground", "oklch(0.985 0 0)"),
    ("--chart-1", "oklch(0.837 0.128 66.29)"),
    ("--chart-2", "oklch(0.705 0.213 47.604)"),
    ("--chart-3", "oklch(0.646 0.222 41.116)"),
    ("--chart-4", "oklch(0.553 0.195 38.402)"),
    ("--chart-5", "oklch(0.47 0.157 37.304)"),
    ("--sidebar-primary", "oklch(0.705 0.213 47.604)"),
    ("--sidebar-primary-foreground", "oklch(0.98 0.016 73.684)"),
];
static PINK_LIGHT: &[(&str, &str)] = &[
    ("--primary", "oklch(0.525 0.223 3.958)"),
    ("--primary-foreground", "oklch(0.971 0.014 343.198)"),
    ("--secondary", "oklch(0.967 0.001 286.375)"),
    ("--secondary-foreground", "oklch(0.21 0.006 285.885)"),
    ("--chart-1", "oklch(0.823 0.12 346.018)"),
    ("--chart-2", "oklch(0.656 0.241 354.308)"),
    ("--chart-3", "oklch(0.592 0.249 0.584)"),
    ("--chart-4", "oklch(0.525 0.223 3.958)"),
    ("--chart-5", "oklch(0.459 0.187 3.815)"),
    ("--sidebar-primary", "oklch(0.592 0.249 0.584)"),
    ("--sidebar-primary-foreground", "oklch(0.971 0.014 343.198)"),
];
static PINK_DARK: &[(&str, &str)] = &[
    ("--primary", "oklch(0.459 0.187 3.815)"),
    ("--primary-foreground", "oklch(0.971 0.014 343.198)"),
    ("--secondary", "oklch(0.274 0.006 286.033)"),
    ("--secondary-foreground", "oklch(0.985 0 0)"),
    ("--chart-1", "oklch(0.823 0.12 346.018)"),
    ("--chart-2", "oklch(0.656 0.241 354.308)"),
    ("--chart-3", "oklch(0.592 0.249 0.584)"),
    ("--chart-4", "oklch(0.525 0.223 3.958)"),
    ("--chart-5", "oklch(0.459 0.187 3.815)"),
    ("--sidebar-primary", "oklch(0.656 0.241 354.308)"),
    ("--sidebar-primary-foreground", "oklch(0.971 0.014 343.198)"),
];
static PURPLE_LIGHT: &[(&str, &str)] = &[
    ("--primary", "oklch(0.496 0.265 301.924)"),
    ("--primary-foreground", "oklch(0.977 0.014 308.299)"),
    ("--secondary", "oklch(0.967 0.001 286.375)"),
    ("--secondary-foreground", "oklch(0.21 0.006 285.885)"),
    ("--chart-1", "oklch(0.827 0.119 306.383)"),
    ("--chart-2", "oklch(0.627 0.265 303.9)"),
    ("--chart-3", "oklch(0.558 0.288 302.321)"),
    ("--chart-4", "oklch(0.496 0.265 301.924)"),
    ("--chart-5", "oklch(0.438 0.218 303.724)"),
    ("--sidebar-primary", "oklch(0.558 0.288 302.321)"),
    ("--sidebar-primary-foreground", "oklch(0.977 0.014 308.299)"),
];
static PURPLE_DARK: &[(&str, &str)] = &[
    ("--primary", "oklch(0.438 0.218 303.724)"),
    ("--primary-foreground", "oklch(0.977 0.014 308.299)"),
    ("--secondary", "oklch(0.274 0.006 286.033)"),
    ("--secondary-foreground", "oklch(0.985 0 0)"),
    ("--chart-1", "oklch(0.827 0.119 306.383)"),
    ("--chart-2", "oklch(0.627 0.265 303.9)"),
    ("--chart-3", "oklch(0.558 0.288 302.321)"),
    ("--chart-4", "oklch(0.496 0.265 301.924)"),
    ("--chart-5", "oklch(0.438 0.218 303.724)"),
    ("--sidebar-primary", "oklch(0.627 0.265 303.9)"),
    ("--sidebar-primary-foreground", "oklch(0.977 0.014 308.299)"),
];
static RED_LIGHT: &[(&str, &str)] = &[
    ("--primary", "oklch(0.505 0.213 27.518)"),
    ("--primary-foreground", "oklch(0.971 0.013 17.38)"),
    ("--secondary", "oklch(0.967 0.001 286.375)"),
    ("--secondary-foreground", "oklch(0.21 0.006 285.885)"),
    ("--chart-1", "oklch(0.808 0.114 19.571)"),
    ("--chart-2", "oklch(0.637 0.237 25.331)"),
    ("--chart-3", "oklch(0.577 0.245 27.325)"),
    ("--chart-4", "oklch(0.505 0.213 27.518)"),
    ("--chart-5", "oklch(0.444 0.177 26.899)"),
    ("--sidebar-primary", "oklch(0.577 0.245 27.325)"),
    ("--sidebar-primary-foreground", "oklch(0.971 0.013 17.38)"),
];
static RED_DARK: &[(&str, &str)] = &[
    ("--primary", "oklch(0.444 0.177 26.899)"),
    ("--primary-foreground", "oklch(0.971 0.013 17.38)"),
    ("--secondary", "oklch(0.274 0.006 286.033)"),
    ("--secondary-foreground", "oklch(0.985 0 0)"),
    ("--chart-1", "oklch(0.808 0.114 19.571)"),
    ("--chart-2", "oklch(0.637 0.237 25.331)"),
    ("--chart-3", "oklch(0.577 0.245 27.325)"),
    ("--chart-4", "oklch(0.505 0.213 27.518)"),
    ("--chart-5", "oklch(0.444 0.177 26.899)"),
    ("--sidebar-primary", "oklch(0.637 0.237 25.331)"),
    ("--sidebar-primary-foreground", "oklch(0.971 0.013 17.38)"),
];
static ROSE_LIGHT: &[(&str, &str)] = &[
    ("--primary", "oklch(0.514 0.222 16.935)"),
    ("--primary-foreground", "oklch(0.969 0.015 12.422)"),
    ("--secondary", "oklch(0.967 0.001 286.375)"),
    ("--secondary-foreground", "oklch(0.21 0.006 285.885)"),
    ("--chart-1", "oklch(0.81 0.117 11.638)"),
    ("--chart-2", "oklch(0.645 0.246 16.439)"),
    ("--chart-3", "oklch(0.586 0.253 17.585)"),
    ("--chart-4", "oklch(0.514 0.222 16.935)"),
    ("--chart-5", "oklch(0.455 0.188 13.697)"),
    ("--sidebar-primary", "oklch(0.586 0.253 17.585)"),
    ("--sidebar-primary-foreground", "oklch(0.969 0.015 12.422)"),
];
static ROSE_DARK: &[(&str, &str)] = &[
    ("--primary", "oklch(0.455 0.188 13.697)"),
    ("--primary-foreground", "oklch(0.969 0.015 12.422)"),
    ("--secondary", "oklch(0.274 0.006 286.033)"),
    ("--secondary-foreground", "oklch(0.985 0 0)"),
    ("--chart-1", "oklch(0.81 0.117 11.638)"),
    ("--chart-2", "oklch(0.645 0.246 16.439)"),
    ("--chart-3", "oklch(0.586 0.253 17.585)"),
    ("--chart-4", "oklch(0.514 0.222 16.935)"),
    ("--chart-5", "oklch(0.455 0.188 13.697)"),
    ("--sidebar-primary", "oklch(0.645 0.246 16.439)"),
    ("--sidebar-primary-foreground", "oklch(0.969 0.015 12.422)"),
];
static SKY_LIGHT: &[(&str, &str)] = &[
    ("--primary", "oklch(0.5 0.134 242.749)"),
    ("--primary-foreground", "oklch(0.977 0.013 236.62)"),
    ("--secondary", "oklch(0.967 0.001 286.375)"),
    ("--secondary-foreground", "oklch(0.21 0.006 285.885)"),
    ("--chart-1", "oklch(0.828 0.111 230.318)"),
    ("--chart-2", "oklch(0.685 0.169 237.323)"),
    ("--chart-3", "oklch(0.588 0.158 241.966)"),
    ("--chart-4", "oklch(0.5 0.134 242.749)"),
    ("--chart-5", "oklch(0.443 0.11 240.79)"),
    ("--sidebar-primary", "oklch(0.588 0.158 241.966)"),
    ("--sidebar-primary-foreground", "oklch(0.977 0.013 236.62)"),
];
static SKY_DARK: &[(&str, &str)] = &[
    ("--primary", "oklch(0.443 0.11 240.79)"),
    ("--primary-foreground", "oklch(0.977 0.013 236.62)"),
    ("--secondary", "oklch(0.274 0.006 286.033)"),
    ("--secondary-foreground", "oklch(0.985 0 0)"),
    ("--chart-1", "oklch(0.828 0.111 230.318)"),
    ("--chart-2", "oklch(0.685 0.169 237.323)"),
    ("--chart-3", "oklch(0.588 0.158 241.966)"),
    ("--chart-4", "oklch(0.5 0.134 242.749)"),
    ("--chart-5", "oklch(0.443 0.11 240.79)"),
    ("--sidebar-primary", "oklch(0.685 0.169 237.323)"),
    ("--sidebar-primary-foreground", "oklch(0.293 0.066 243.157)"),
];
static TEAL_LIGHT: &[(&str, &str)] = &[
    ("--primary", "oklch(0.511 0.096 186.391)"),
    ("--primary-foreground", "oklch(0.984 0.014 180.72)"),
    ("--secondary", "oklch(0.967 0.001 286.375)"),
    ("--secondary-foreground", "oklch(0.21 0.006 285.885)"),
    ("--chart-1", "oklch(0.855 0.138 181.071)"),
    ("--chart-2", "oklch(0.704 0.14 182.503)"),
    ("--chart-3", "oklch(0.6 0.118 184.704)"),
    ("--chart-4", "oklch(0.511 0.096 186.391)"),
    ("--chart-5", "oklch(0.437 0.078 188.216)"),
    ("--sidebar-primary", "oklch(0.6 0.118 184.704)"),
    ("--sidebar-primary-foreground", "oklch(0.984 0.014 180.72)"),
];
static TEAL_DARK: &[(&str, &str)] = &[
    ("--primary", "oklch(0.437 0.078 188.216)"),
    ("--primary-foreground", "oklch(0.984 0.014 180.72)"),
    ("--secondary", "oklch(0.274 0.006 286.033)"),
    ("--secondary-foreground", "oklch(0.985 0 0)"),
    ("--chart-1", "oklch(0.855 0.138 181.071)"),
    ("--chart-2", "oklch(0.704 0.14 182.503)"),
    ("--chart-3", "oklch(0.6 0.118 184.704)"),
    ("--chart-4", "oklch(0.511 0.096 186.391)"),
    ("--chart-5", "oklch(0.437 0.078 188.216)"),
    ("--sidebar-primary", "oklch(0.704 0.14 182.503)"),
    ("--sidebar-primary-foreground", "oklch(0.277 0.046 192.524)"),
];
static VIOLET_LIGHT: &[(&str, &str)] = &[
    ("--primary", "oklch(0.491 0.27 292.581)"),
    ("--primary-foreground", "oklch(0.969 0.016 293.756)"),
    ("--secondary", "oklch(0.967 0.001 286.375)"),
    ("--secondary-foreground", "oklch(0.21 0.006 285.885)"),
    ("--chart-1", "oklch(0.811 0.111 293.571)"),
    ("--chart-2", "oklch(0.606 0.25 292.717)"),
    ("--chart-3", "oklch(0.541 0.281 293.009)"),
    ("--chart-4", "oklch(0.491 0.27 292.581)"),
    ("--chart-5", "oklch(0.432 0.232 292.759)"),
    ("--sidebar-primary", "oklch(0.541 0.281 293.009)"),
    ("--sidebar-primary-foreground", "oklch(0.969 0.016 293.756)"),
];
static VIOLET_DARK: &[(&str, &str)] = &[
    ("--primary", "oklch(0.432 0.232 292.759)"),
    ("--primary-foreground", "oklch(0.969 0.016 293.756)"),
    ("--secondary", "oklch(0.274 0.006 286.033)"),
    ("--secondary-foreground", "oklch(0.985 0 0)"),
    ("--chart-1", "oklch(0.811 0.111 293.571)"),
    ("--chart-2", "oklch(0.606 0.25 292.717)"),
    ("--chart-3", "oklch(0.541 0.281 293.009)"),
    ("--chart-4", "oklch(0.491 0.27 292.581)"),
    ("--chart-5", "oklch(0.432 0.232 292.759)"),
    ("--sidebar-primary", "oklch(0.606 0.25 292.717)"),
    ("--sidebar-primary-foreground", "oklch(0.969 0.016 293.756)"),
];
static YELLOW_LIGHT: &[(&str, &str)] = &[
    ("--primary", "oklch(0.852 0.199 91.936)"),
    ("--primary-foreground", "oklch(0.421 0.095 57.708)"),
    ("--secondary", "oklch(0.967 0.001 286.375)"),
    ("--secondary-foreground", "oklch(0.21 0.006 285.885)"),
    ("--chart-1", "oklch(0.905 0.182 98.111)"),
    ("--chart-2", "oklch(0.795 0.184 86.047)"),
    ("--chart-3", "oklch(0.681 0.162 75.834)"),
    ("--chart-4", "oklch(0.554 0.135 66.442)"),
    ("--chart-5", "oklch(0.476 0.114 61.907)"),
    ("--sidebar-primary", "oklch(0.681 0.162 75.834)"),
    ("--sidebar-primary-foreground", "oklch(0.987 0.026 102.212)"),
];
static YELLOW_DARK: &[(&str, &str)] = &[
    ("--primary", "oklch(0.795 0.184 86.047)"),
    ("--primary-foreground", "oklch(0.421 0.095 57.708)"),
    ("--secondary", "oklch(0.274 0.006 286.033)"),
    ("--secondary-foreground", "oklch(0.985 0 0)"),
    ("--chart-1", "oklch(0.905 0.182 98.111)"),
    ("--chart-2", "oklch(0.795 0.184 86.047)"),
    ("--chart-3", "oklch(0.681 0.162 75.834)"),
    ("--chart-4", "oklch(0.554 0.135 66.442)"),
    ("--chart-5", "oklch(0.476 0.114 61.907)"),
    ("--sidebar-primary", "oklch(0.795 0.184 86.047)"),
    ("--sidebar-primary-foreground", "oklch(0.987 0.026 102.212)"),
];
