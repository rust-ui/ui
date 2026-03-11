use leptos::prelude::*;
use registry::ui::select::{Select, SelectContent, SelectGroup, SelectOption, SelectTrigger};

/* ========================================================== */
/*                       ✨ TYPES ✨                          */
/* ========================================================== */

#[derive(Clone, Copy, PartialEq, Default)]
pub enum ColorTheme {
    #[default]
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
    pub const ALL: &'static [ColorTheme] = &[
        ColorTheme::None,
        ColorTheme::Amber,
        ColorTheme::Blue,
        ColorTheme::Cyan,
        ColorTheme::Emerald,
        ColorTheme::Fuchsia,
        ColorTheme::Green,
        ColorTheme::Indigo,
        ColorTheme::Lime,
        ColorTheme::Orange,
        ColorTheme::Pink,
        ColorTheme::Purple,
        ColorTheme::Red,
        ColorTheme::Rose,
        ColorTheme::Sky,
        ColorTheme::Teal,
        ColorTheme::Violet,
        ColorTheme::Yellow,
    ];

    /// CSS variable keys that color themes override.
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

    pub fn label(&self) -> &'static str {
        match self {
            ColorTheme::None => "Default",
            ColorTheme::Amber => "Amber",
            ColorTheme::Blue => "Blue",
            ColorTheme::Cyan => "Cyan",
            ColorTheme::Emerald => "Emerald",
            ColorTheme::Fuchsia => "Fuchsia",
            ColorTheme::Green => "Green",
            ColorTheme::Indigo => "Indigo",
            ColorTheme::Lime => "Lime",
            ColorTheme::Orange => "Orange",
            ColorTheme::Pink => "Pink",
            ColorTheme::Purple => "Purple",
            ColorTheme::Red => "Red",
            ColorTheme::Rose => "Rose",
            ColorTheme::Sky => "Sky",
            ColorTheme::Teal => "Teal",
            ColorTheme::Violet => "Violet",
            ColorTheme::Yellow => "Yellow",
        }
    }

    pub fn swatch(&self) -> &'static str {
        match self {
            ColorTheme::None => "#d4d4d4",
            ColorTheme::Amber => "#f59e0b",
            ColorTheme::Blue => "#3b82f6",
            ColorTheme::Cyan => "#06b6d4",
            ColorTheme::Emerald => "#10b981",
            ColorTheme::Fuchsia => "#d946ef",
            ColorTheme::Green => "#22c55e",
            ColorTheme::Indigo => "#6366f1",
            ColorTheme::Lime => "#84cc16",
            ColorTheme::Orange => "#f97316",
            ColorTheme::Pink => "#ec4899",
            ColorTheme::Purple => "#a855f7",
            ColorTheme::Red => "#ef4444",
            ColorTheme::Rose => "#f43f5e",
            ColorTheme::Sky => "#0ea5e9",
            ColorTheme::Teal => "#14b8a6",
            ColorTheme::Violet => "#8b5cf6",
            ColorTheme::Yellow => "#eab308",
        }
    }

    pub fn light_vars(&self) -> &'static [(&'static str, &'static str)] {
        match self {
            ColorTheme::None => &[],
            ColorTheme::Amber => AMBER_LIGHT,
            ColorTheme::Blue => BLUE_LIGHT,
            ColorTheme::Cyan => CYAN_LIGHT,
            ColorTheme::Emerald => EMERALD_LIGHT,
            ColorTheme::Fuchsia => FUCHSIA_LIGHT,
            ColorTheme::Green => GREEN_LIGHT,
            ColorTheme::Indigo => INDIGO_LIGHT,
            ColorTheme::Lime => LIME_LIGHT,
            ColorTheme::Orange => ORANGE_LIGHT,
            ColorTheme::Pink => PINK_LIGHT,
            ColorTheme::Purple => PURPLE_LIGHT,
            ColorTheme::Red => RED_LIGHT,
            ColorTheme::Rose => ROSE_LIGHT,
            ColorTheme::Sky => SKY_LIGHT,
            ColorTheme::Teal => TEAL_LIGHT,
            ColorTheme::Violet => VIOLET_LIGHT,
            ColorTheme::Yellow => YELLOW_LIGHT,
        }
    }

    pub fn dark_vars(&self) -> &'static [(&'static str, &'static str)] {
        match self {
            ColorTheme::None => &[],
            ColorTheme::Amber => AMBER_DARK,
            ColorTheme::Blue => BLUE_DARK,
            ColorTheme::Cyan => CYAN_DARK,
            ColorTheme::Emerald => EMERALD_DARK,
            ColorTheme::Fuchsia => FUCHSIA_DARK,
            ColorTheme::Green => GREEN_DARK,
            ColorTheme::Indigo => INDIGO_DARK,
            ColorTheme::Lime => LIME_DARK,
            ColorTheme::Orange => ORANGE_DARK,
            ColorTheme::Pink => PINK_DARK,
            ColorTheme::Purple => PURPLE_DARK,
            ColorTheme::Red => RED_DARK,
            ColorTheme::Rose => ROSE_DARK,
            ColorTheme::Sky => SKY_DARK,
            ColorTheme::Teal => TEAL_DARK,
            ColorTheme::Violet => VIOLET_DARK,
            ColorTheme::Yellow => YELLOW_DARK,
        }
    }

    pub fn from_label(label: &str) -> Option<Self> {
        Self::ALL.iter().copied().find(|ct| ct.label() == label)
    }

    pub fn to_index(self) -> usize {
        match self {
            ColorTheme::None => 0,
            ColorTheme::Amber => 1,
            ColorTheme::Blue => 2,
            ColorTheme::Cyan => 3,
            ColorTheme::Emerald => 4,
            ColorTheme::Fuchsia => 5,
            ColorTheme::Green => 6,
            ColorTheme::Indigo => 7,
            ColorTheme::Lime => 8,
            ColorTheme::Orange => 9,
            ColorTheme::Pink => 10,
            ColorTheme::Purple => 11,
            ColorTheme::Red => 12,
            ColorTheme::Rose => 13,
            ColorTheme::Sky => 14,
            ColorTheme::Teal => 15,
            ColorTheme::Violet => 16,
            ColorTheme::Yellow => 17,
        }
    }

    pub fn from_index(idx: u32) -> Option<Self> {
        match idx {
            0 => Some(ColorTheme::None),
            1 => Some(ColorTheme::Amber),
            2 => Some(ColorTheme::Blue),
            3 => Some(ColorTheme::Cyan),
            4 => Some(ColorTheme::Emerald),
            5 => Some(ColorTheme::Fuchsia),
            6 => Some(ColorTheme::Green),
            7 => Some(ColorTheme::Indigo),
            8 => Some(ColorTheme::Lime),
            9 => Some(ColorTheme::Orange),
            10 => Some(ColorTheme::Pink),
            11 => Some(ColorTheme::Purple),
            12 => Some(ColorTheme::Red),
            13 => Some(ColorTheme::Rose),
            14 => Some(ColorTheme::Sky),
            15 => Some(ColorTheme::Teal),
            16 => Some(ColorTheme::Violet),
            17 => Some(ColorTheme::Yellow),
            _ => None,
        }
    }
}

/* ========================================================== */
/*                     ✨ COMPONENT ✨                        */
/* ========================================================== */

#[component]
pub fn ColorThemePicker(color_theme: RwSignal<ColorTheme>) -> impl IntoView {
    view! {
        <Select
            class="w-full"
            default_value=color_theme.get_untracked().label().to_string()
            on_change=Callback::new(move |val: Option<String>| {
                if let Some(ct) = val.as_deref().and_then(ColorTheme::from_label) {
                    color_theme.set(ct);
                }
            })
        >
            <SelectTrigger>
                <span class="text-sm text-muted-foreground">"Theme"</span>
                <span class="mr-auto ml-2 text-sm font-medium">{move || color_theme.get().label()}</span>
                <span
                    class="flex-shrink-0 mr-1 rounded-full border size-4 border-border/50"
                    style=move || format!("background-color:{}", color_theme.get().swatch())
                />
            </SelectTrigger>

            <SelectContent class="w-full">
                <SelectGroup>
                    {ColorTheme::ALL
                        .iter()
                        .map(|&ct| {
                            view! {
                                <SelectOption value=ct.label().to_string()>
                                    <span
                                        class="flex-shrink-0 rounded-full border size-3.5 border-border/50"
                                        style=format!("background-color:{}", ct.swatch())
                                    />
                                    {ct.label()}
                                </SelectOption>
                            }
                        })
                        .collect_view()}
                </SelectGroup>
            </SelectContent>
        </Select>
    }
}

/* ========================================================== */
/*                    ✨ THEME DATA ✨                        */
/* ========================================================== */

// Exact OKLCH values from shadcn/ui v4 registry/themes.ts
// Color themes only override: primary, primary-foreground, secondary, secondary-foreground,
// chart-1..5, sidebar-primary, sidebar-primary-foreground

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
