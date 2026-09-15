/// Base color (gray scale) — controls background/foreground/border/muted/accent vars.
/// OKLCH values mirror the /create page's theme_picker.rs.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum BaseColor {
    #[default]
    Neutral,
    Stone,
    Zinc,
    Mauve,
    Olive,
    Mist,
    Taupe,
}

impl BaseColor {
    pub const ALL: &'static [BaseColor] = &[
        BaseColor::Neutral,
        BaseColor::Stone,
        BaseColor::Zinc,
        BaseColor::Mauve,
        BaseColor::Olive,
        BaseColor::Mist,
        BaseColor::Taupe,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            BaseColor::Neutral => "Neutral",
            BaseColor::Stone => "Stone",
            BaseColor::Zinc => "Zinc",
            BaseColor::Mauve => "Mauve",
            BaseColor::Olive => "Olive",
            BaseColor::Mist => "Mist",
            BaseColor::Taupe => "Taupe",
        }
    }

    pub fn all_labels() -> Vec<&'static str> {
        Self::ALL.iter().map(|c| c.label()).collect()
    }

    #[allow(dead_code)]
    pub fn from_str(s: &str) -> Option<Self> {
        Self::ALL.iter().copied().find(|c| c.label().eq_ignore_ascii_case(s))
    }

    pub fn from_index(idx: usize) -> Self {
        Self::ALL.get(idx).copied().unwrap_or_default()
    }

    pub fn light_vars(&self) -> &'static [(&'static str, &'static str)] {
        match self {
            BaseColor::Neutral => NEUTRAL_LIGHT,
            BaseColor::Stone => STONE_LIGHT,
            BaseColor::Zinc => ZINC_LIGHT,
            BaseColor::Mauve => MAUVE_LIGHT,
            BaseColor::Olive => OLIVE_LIGHT,
            BaseColor::Mist => MIST_LIGHT,
            BaseColor::Taupe => TAUPE_LIGHT,
        }
    }

    pub fn dark_vars(&self) -> &'static [(&'static str, &'static str)] {
        match self {
            BaseColor::Neutral => NEUTRAL_DARK,
            BaseColor::Stone => STONE_DARK,
            BaseColor::Zinc => ZINC_DARK,
            BaseColor::Mauve => MAUVE_DARK,
            BaseColor::Olive => OLIVE_DARK,
            BaseColor::Mist => MIST_DARK,
            BaseColor::Taupe => TAUPE_DARK,
        }
    }
}

/// Accent color — overrides primary/secondary/chart/sidebar vars on top of the base.
/// OKLCH values mirror the /create page's color_theme_picker.rs.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum AccentColor {
    #[default]
    Default,
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

impl AccentColor {
    pub const ALL: &'static [AccentColor] = &[
        AccentColor::Default,
        AccentColor::Amber,
        AccentColor::Blue,
        AccentColor::Cyan,
        AccentColor::Emerald,
        AccentColor::Fuchsia,
        AccentColor::Green,
        AccentColor::Indigo,
        AccentColor::Lime,
        AccentColor::Orange,
        AccentColor::Pink,
        AccentColor::Purple,
        AccentColor::Red,
        AccentColor::Rose,
        AccentColor::Sky,
        AccentColor::Teal,
        AccentColor::Violet,
        AccentColor::Yellow,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            AccentColor::Default => "Default",
            AccentColor::Amber => "Amber",
            AccentColor::Blue => "Blue",
            AccentColor::Cyan => "Cyan",
            AccentColor::Emerald => "Emerald",
            AccentColor::Fuchsia => "Fuchsia",
            AccentColor::Green => "Green",
            AccentColor::Indigo => "Indigo",
            AccentColor::Lime => "Lime",
            AccentColor::Orange => "Orange",
            AccentColor::Pink => "Pink",
            AccentColor::Purple => "Purple",
            AccentColor::Red => "Red",
            AccentColor::Rose => "Rose",
            AccentColor::Sky => "Sky",
            AccentColor::Teal => "Teal",
            AccentColor::Violet => "Violet",
            AccentColor::Yellow => "Yellow",
        }
    }

    pub fn all_labels() -> Vec<&'static str> {
        Self::ALL.iter().map(|c| c.label()).collect()
    }

    #[allow(dead_code)]
    pub fn from_str(s: &str) -> Option<Self> {
        Self::ALL.iter().copied().find(|c| c.label().eq_ignore_ascii_case(s))
    }

    pub fn from_index(idx: usize) -> Self {
        Self::ALL.get(idx).copied().unwrap_or_default()
    }

    pub fn light_vars(&self) -> &'static [(&'static str, &'static str)] {
        match self {
            AccentColor::Default => &[],
            AccentColor::Amber => AMBER_LIGHT,
            AccentColor::Blue => BLUE_LIGHT,
            AccentColor::Cyan => CYAN_LIGHT,
            AccentColor::Emerald => EMERALD_LIGHT,
            AccentColor::Fuchsia => FUCHSIA_LIGHT,
            AccentColor::Green => GREEN_LIGHT,
            AccentColor::Indigo => INDIGO_LIGHT,
            AccentColor::Lime => LIME_LIGHT,
            AccentColor::Orange => ORANGE_LIGHT,
            AccentColor::Pink => PINK_LIGHT,
            AccentColor::Purple => PURPLE_LIGHT,
            AccentColor::Red => RED_LIGHT,
            AccentColor::Rose => ROSE_LIGHT,
            AccentColor::Sky => SKY_LIGHT,
            AccentColor::Teal => TEAL_LIGHT,
            AccentColor::Violet => VIOLET_LIGHT,
            AccentColor::Yellow => YELLOW_LIGHT,
        }
    }

    pub fn dark_vars(&self) -> &'static [(&'static str, &'static str)] {
        match self {
            AccentColor::Default => &[],
            AccentColor::Amber => AMBER_DARK,
            AccentColor::Blue => BLUE_DARK,
            AccentColor::Cyan => CYAN_DARK,
            AccentColor::Emerald => EMERALD_DARK,
            AccentColor::Fuchsia => FUCHSIA_DARK,
            AccentColor::Green => GREEN_DARK,
            AccentColor::Indigo => INDIGO_DARK,
            AccentColor::Lime => LIME_DARK,
            AccentColor::Orange => ORANGE_DARK,
            AccentColor::Pink => PINK_DARK,
            AccentColor::Purple => PURPLE_DARK,
            AccentColor::Red => RED_DARK,
            AccentColor::Rose => ROSE_DARK,
            AccentColor::Sky => SKY_DARK,
            AccentColor::Teal => TEAL_DARK,
            AccentColor::Violet => VIOLET_DARK,
            AccentColor::Yellow => YELLOW_DARK,
        }
    }
}

/* ========================================================== */
/*                    ✨ CSS GENERATION ✨                    */
/* ========================================================== */

/// Build the :root + .dark CSS variable blocks from a base color + accent overlay.
/// The accent vars are written after base vars, overriding primary/secondary/chart/sidebar.
pub fn generate_theme_vars(base: BaseColor, accent: AccentColor) -> String {
    let mut out = String::from(":root {\n  --radius: 0.625rem;\n");

    for (k, v) in base.light_vars() {
        out.push_str(&format!("  {k}: {v};\n"));
    }
    for (k, v) in accent.light_vars() {
        out.push_str(&format!("  {k}: {v};\n"));
    }
    out.push_str("  --destructive: oklch(0.577 0.245 27.325);\n");
    out.push_str("}\n");

    out.push_str("\n.dark {\n");
    for (k, v) in base.dark_vars() {
        out.push_str(&format!("  {k}: {v};\n"));
    }
    for (k, v) in accent.dark_vars() {
        out.push_str(&format!("  {k}: {v};\n"));
    }
    out.push_str("  --destructive: oklch(0.704 0.191 22.216);\n");
    out.push_str("}\n");

    out
}

/* ========================================================== */
/*                    ✨ BASE COLOR DATA ✨                   */
/* ========================================================== */

// Exact OKLCH values from shadcn/ui v4 registry/themes.ts (mirrored from /create page)

static NEUTRAL_LIGHT: &[(&str, &str)] = &[
    ("--background", "oklch(1 0 0)"),
    ("--foreground", "oklch(0.145 0 0)"),
    ("--card", "oklch(1 0 0)"),
    ("--card-foreground", "oklch(0.145 0 0)"),
    ("--popover", "oklch(1 0 0)"),
    ("--popover-foreground", "oklch(0.145 0 0)"),
    ("--primary", "oklch(0.205 0 0)"),
    ("--primary-foreground", "oklch(0.985 0 0)"),
    ("--secondary", "oklch(0.97 0 0)"),
    ("--secondary-foreground", "oklch(0.205 0 0)"),
    ("--muted", "oklch(0.97 0 0)"),
    ("--muted-foreground", "oklch(0.556 0 0)"),
    ("--accent", "oklch(0.97 0 0)"),
    ("--accent-foreground", "oklch(0.205 0 0)"),
    ("--border", "oklch(0.922 0 0)"),
    ("--input", "oklch(0.922 0 0)"),
    ("--ring", "oklch(0.708 0 0)"),
];

static NEUTRAL_DARK: &[(&str, &str)] = &[
    ("--background", "oklch(0.145 0 0)"),
    ("--foreground", "oklch(0.985 0 0)"),
    ("--card", "oklch(0.205 0 0)"),
    ("--card-foreground", "oklch(0.985 0 0)"),
    ("--popover", "oklch(0.205 0 0)"),
    ("--popover-foreground", "oklch(0.985 0 0)"),
    ("--primary", "oklch(0.87 0.00 0)"),
    ("--primary-foreground", "oklch(0.205 0 0)"),
    ("--secondary", "oklch(0.269 0 0)"),
    ("--secondary-foreground", "oklch(0.985 0 0)"),
    ("--muted", "oklch(0.269 0 0)"),
    ("--muted-foreground", "oklch(0.708 0 0)"),
    ("--accent", "oklch(0.371 0 0)"),
    ("--accent-foreground", "oklch(0.985 0 0)"),
    ("--border", "oklch(1 0 0 / 10%)"),
    ("--input", "oklch(1 0 0 / 15%)"),
    ("--ring", "oklch(0.556 0 0)"),
];

static STONE_LIGHT: &[(&str, &str)] = &[
    ("--background", "oklch(1 0 0)"),
    ("--foreground", "oklch(0.147 0.004 49.25)"),
    ("--card", "oklch(1 0 0)"),
    ("--card-foreground", "oklch(0.147 0.004 49.25)"),
    ("--popover", "oklch(1 0 0)"),
    ("--popover-foreground", "oklch(0.147 0.004 49.25)"),
    ("--primary", "oklch(0.216 0.006 56.043)"),
    ("--primary-foreground", "oklch(0.985 0.001 106.423)"),
    ("--secondary", "oklch(0.97 0.001 106.424)"),
    ("--secondary-foreground", "oklch(0.216 0.006 56.043)"),
    ("--muted", "oklch(0.97 0.001 106.424)"),
    ("--muted-foreground", "oklch(0.553 0.013 58.071)"),
    ("--accent", "oklch(0.97 0.001 106.424)"),
    ("--accent-foreground", "oklch(0.216 0.006 56.043)"),
    ("--border", "oklch(0.923 0.003 48.717)"),
    ("--input", "oklch(0.923 0.003 48.717)"),
    ("--ring", "oklch(0.709 0.01 56.259)"),
];

static STONE_DARK: &[(&str, &str)] = &[
    ("--background", "oklch(0.147 0.004 49.25)"),
    ("--foreground", "oklch(0.985 0.001 106.423)"),
    ("--card", "oklch(0.216 0.006 56.043)"),
    ("--card-foreground", "oklch(0.985 0.001 106.423)"),
    ("--popover", "oklch(0.216 0.006 56.043)"),
    ("--popover-foreground", "oklch(0.985 0.001 106.423)"),
    ("--primary", "oklch(0.923 0.003 48.717)"),
    ("--primary-foreground", "oklch(0.216 0.006 56.043)"),
    ("--secondary", "oklch(0.268 0.007 34.298)"),
    ("--secondary-foreground", "oklch(0.985 0.001 106.423)"),
    ("--muted", "oklch(0.268 0.007 34.298)"),
    ("--muted-foreground", "oklch(0.709 0.01 56.259)"),
    ("--accent", "oklch(0.268 0.007 34.298)"),
    ("--accent-foreground", "oklch(0.985 0.001 106.423)"),
    ("--border", "oklch(1 0 0 / 10%)"),
    ("--input", "oklch(1 0 0 / 15%)"),
    ("--ring", "oklch(0.553 0.013 58.071)"),
];

static ZINC_LIGHT: &[(&str, &str)] = &[
    ("--background", "oklch(1 0 0)"),
    ("--foreground", "oklch(0.141 0.005 285.823)"),
    ("--card", "oklch(1 0 0)"),
    ("--card-foreground", "oklch(0.141 0.005 285.823)"),
    ("--popover", "oklch(1 0 0)"),
    ("--popover-foreground", "oklch(0.141 0.005 285.823)"),
    ("--primary", "oklch(0.21 0.006 285.885)"),
    ("--primary-foreground", "oklch(0.985 0 0)"),
    ("--secondary", "oklch(0.967 0.001 286.375)"),
    ("--secondary-foreground", "oklch(0.21 0.006 285.885)"),
    ("--muted", "oklch(0.967 0.001 286.375)"),
    ("--muted-foreground", "oklch(0.552 0.016 285.938)"),
    ("--accent", "oklch(0.967 0.001 286.375)"),
    ("--accent-foreground", "oklch(0.21 0.006 285.885)"),
    ("--border", "oklch(0.92 0.004 286.32)"),
    ("--input", "oklch(0.92 0.004 286.32)"),
    ("--ring", "oklch(0.705 0.015 286.067)"),
];

static ZINC_DARK: &[(&str, &str)] = &[
    ("--background", "oklch(0.141 0.005 285.823)"),
    ("--foreground", "oklch(0.985 0 0)"),
    ("--card", "oklch(0.21 0.006 285.885)"),
    ("--card-foreground", "oklch(0.985 0 0)"),
    ("--popover", "oklch(0.21 0.006 285.885)"),
    ("--popover-foreground", "oklch(0.985 0 0)"),
    ("--primary", "oklch(0.92 0.004 286.32)"),
    ("--primary-foreground", "oklch(0.21 0.006 285.885)"),
    ("--secondary", "oklch(0.274 0.006 286.033)"),
    ("--secondary-foreground", "oklch(0.985 0 0)"),
    ("--muted", "oklch(0.274 0.006 286.033)"),
    ("--muted-foreground", "oklch(0.705 0.015 286.067)"),
    ("--accent", "oklch(0.274 0.006 286.033)"),
    ("--accent-foreground", "oklch(0.985 0 0)"),
    ("--border", "oklch(1 0 0 / 10%)"),
    ("--input", "oklch(1 0 0 / 15%)"),
    ("--ring", "oklch(0.552 0.016 285.938)"),
];

static MAUVE_LIGHT: &[(&str, &str)] = &[
    ("--background", "oklch(1 0 0)"),
    ("--foreground", "oklch(0.145 0.008 326)"),
    ("--card", "oklch(1 0 0)"),
    ("--card-foreground", "oklch(0.145 0.008 326)"),
    ("--popover", "oklch(1 0 0)"),
    ("--popover-foreground", "oklch(0.145 0.008 326)"),
    ("--primary", "oklch(0.212 0.019 322.12)"),
    ("--primary-foreground", "oklch(0.985 0 0)"),
    ("--secondary", "oklch(0.96 0.003 325.6)"),
    ("--secondary-foreground", "oklch(0.212 0.019 322.12)"),
    ("--muted", "oklch(0.96 0.003 325.6)"),
    ("--muted-foreground", "oklch(0.542 0.034 322.5)"),
    ("--accent", "oklch(0.96 0.003 325.6)"),
    ("--accent-foreground", "oklch(0.212 0.019 322.12)"),
    ("--border", "oklch(0.922 0.005 325.62)"),
    ("--input", "oklch(0.922 0.005 325.62)"),
    ("--ring", "oklch(0.711 0.019 323.02)"),
];

static MAUVE_DARK: &[(&str, &str)] = &[
    ("--background", "oklch(0.145 0.008 326)"),
    ("--foreground", "oklch(0.985 0 0)"),
    ("--card", "oklch(0.212 0.019 322.12)"),
    ("--card-foreground", "oklch(0.985 0 0)"),
    ("--popover", "oklch(0.212 0.019 322.12)"),
    ("--popover-foreground", "oklch(0.985 0 0)"),
    ("--primary", "oklch(0.922 0.005 325.62)"),
    ("--primary-foreground", "oklch(0.212 0.019 322.12)"),
    ("--secondary", "oklch(0.263 0.024 320.12)"),
    ("--secondary-foreground", "oklch(0.985 0 0)"),
    ("--muted", "oklch(0.263 0.024 320.12)"),
    ("--muted-foreground", "oklch(0.711 0.019 323.02)"),
    ("--accent", "oklch(0.263 0.024 320.12)"),
    ("--accent-foreground", "oklch(0.985 0 0)"),
    ("--border", "oklch(1 0 0 / 10%)"),
    ("--input", "oklch(1 0 0 / 15%)"),
    ("--ring", "oklch(0.542 0.034 322.5)"),
];

static OLIVE_LIGHT: &[(&str, &str)] = &[
    ("--background", "oklch(1 0 0)"),
    ("--foreground", "oklch(0.153 0.006 107.1)"),
    ("--card", "oklch(1 0 0)"),
    ("--card-foreground", "oklch(0.153 0.006 107.1)"),
    ("--popover", "oklch(1 0 0)"),
    ("--popover-foreground", "oklch(0.153 0.006 107.1)"),
    ("--primary", "oklch(0.228 0.013 107.4)"),
    ("--primary-foreground", "oklch(0.988 0.003 106.5)"),
    ("--secondary", "oklch(0.966 0.005 106.5)"),
    ("--secondary-foreground", "oklch(0.228 0.013 107.4)"),
    ("--muted", "oklch(0.966 0.005 106.5)"),
    ("--muted-foreground", "oklch(0.58 0.031 107.3)"),
    ("--accent", "oklch(0.966 0.005 106.5)"),
    ("--accent-foreground", "oklch(0.228 0.013 107.4)"),
    ("--border", "oklch(0.93 0.007 106.5)"),
    ("--input", "oklch(0.93 0.007 106.5)"),
    ("--ring", "oklch(0.737 0.021 106.9)"),
];

static OLIVE_DARK: &[(&str, &str)] = &[
    ("--background", "oklch(0.153 0.006 107.1)"),
    ("--foreground", "oklch(0.988 0.003 106.5)"),
    ("--card", "oklch(0.228 0.013 107.4)"),
    ("--card-foreground", "oklch(0.988 0.003 106.5)"),
    ("--popover", "oklch(0.228 0.013 107.4)"),
    ("--popover-foreground", "oklch(0.988 0.003 106.5)"),
    ("--primary", "oklch(0.93 0.007 106.5)"),
    ("--primary-foreground", "oklch(0.228 0.013 107.4)"),
    ("--secondary", "oklch(0.286 0.016 107.4)"),
    ("--secondary-foreground", "oklch(0.988 0.003 106.5)"),
    ("--muted", "oklch(0.286 0.016 107.4)"),
    ("--muted-foreground", "oklch(0.737 0.021 106.9)"),
    ("--accent", "oklch(0.286 0.016 107.4)"),
    ("--accent-foreground", "oklch(0.988 0.003 106.5)"),
    ("--border", "oklch(1 0 0 / 10%)"),
    ("--input", "oklch(1 0 0 / 15%)"),
    ("--ring", "oklch(0.58 0.031 107.3)"),
];

static MIST_LIGHT: &[(&str, &str)] = &[
    ("--background", "oklch(1 0 0)"),
    ("--foreground", "oklch(0.148 0.004 228.8)"),
    ("--card", "oklch(1 0 0)"),
    ("--card-foreground", "oklch(0.148 0.004 228.8)"),
    ("--popover", "oklch(1 0 0)"),
    ("--popover-foreground", "oklch(0.148 0.004 228.8)"),
    ("--primary", "oklch(0.218 0.008 223.9)"),
    ("--primary-foreground", "oklch(0.987 0.002 197.1)"),
    ("--secondary", "oklch(0.963 0.002 197.1)"),
    ("--secondary-foreground", "oklch(0.218 0.008 223.9)"),
    ("--muted", "oklch(0.963 0.002 197.1)"),
    ("--muted-foreground", "oklch(0.56 0.021 213.5)"),
    ("--accent", "oklch(0.963 0.002 197.1)"),
    ("--accent-foreground", "oklch(0.218 0.008 223.9)"),
    ("--border", "oklch(0.925 0.005 214.3)"),
    ("--input", "oklch(0.925 0.005 214.3)"),
    ("--ring", "oklch(0.723 0.014 214.4)"),
];

static MIST_DARK: &[(&str, &str)] = &[
    ("--background", "oklch(0.148 0.004 228.8)"),
    ("--foreground", "oklch(0.987 0.002 197.1)"),
    ("--card", "oklch(0.218 0.008 223.9)"),
    ("--card-foreground", "oklch(0.987 0.002 197.1)"),
    ("--popover", "oklch(0.218 0.008 223.9)"),
    ("--popover-foreground", "oklch(0.987 0.002 197.1)"),
    ("--primary", "oklch(0.925 0.005 214.3)"),
    ("--primary-foreground", "oklch(0.218 0.008 223.9)"),
    ("--secondary", "oklch(0.275 0.011 216.9)"),
    ("--secondary-foreground", "oklch(0.987 0.002 197.1)"),
    ("--muted", "oklch(0.275 0.011 216.9)"),
    ("--muted-foreground", "oklch(0.723 0.014 214.4)"),
    ("--accent", "oklch(0.275 0.011 216.9)"),
    ("--accent-foreground", "oklch(0.987 0.002 197.1)"),
    ("--border", "oklch(1 0 0 / 10%)"),
    ("--input", "oklch(1 0 0 / 15%)"),
    ("--ring", "oklch(0.56 0.021 213.5)"),
];

static TAUPE_LIGHT: &[(&str, &str)] = &[
    ("--background", "oklch(1 0 0)"),
    ("--foreground", "oklch(0.147 0.004 49.3)"),
    ("--card", "oklch(1 0 0)"),
    ("--card-foreground", "oklch(0.147 0.004 49.3)"),
    ("--popover", "oklch(1 0 0)"),
    ("--popover-foreground", "oklch(0.147 0.004 49.3)"),
    ("--primary", "oklch(0.214 0.009 43.1)"),
    ("--primary-foreground", "oklch(0.986 0.002 67.8)"),
    ("--secondary", "oklch(0.96 0.002 17.2)"),
    ("--secondary-foreground", "oklch(0.214 0.009 43.1)"),
    ("--muted", "oklch(0.96 0.002 17.2)"),
    ("--muted-foreground", "oklch(0.547 0.021 43.1)"),
    ("--accent", "oklch(0.96 0.002 17.2)"),
    ("--accent-foreground", "oklch(0.214 0.009 43.1)"),
    ("--border", "oklch(0.922 0.005 34.3)"),
    ("--input", "oklch(0.922 0.005 34.3)"),
    ("--ring", "oklch(0.714 0.014 41.2)"),
];

static TAUPE_DARK: &[(&str, &str)] = &[
    ("--background", "oklch(0.147 0.004 49.3)"),
    ("--foreground", "oklch(0.986 0.002 67.8)"),
    ("--card", "oklch(0.214 0.009 43.1)"),
    ("--card-foreground", "oklch(0.986 0.002 67.8)"),
    ("--popover", "oklch(0.214 0.009 43.1)"),
    ("--popover-foreground", "oklch(0.986 0.002 67.8)"),
    ("--primary", "oklch(0.922 0.005 34.3)"),
    ("--primary-foreground", "oklch(0.214 0.009 43.1)"),
    ("--secondary", "oklch(0.268 0.011 36.5)"),
    ("--secondary-foreground", "oklch(0.986 0.002 67.8)"),
    ("--muted", "oklch(0.268 0.011 36.5)"),
    ("--muted-foreground", "oklch(0.714 0.014 41.2)"),
    ("--accent", "oklch(0.268 0.011 36.5)"),
    ("--accent-foreground", "oklch(0.986 0.002 67.8)"),
    ("--border", "oklch(1 0 0 / 10%)"),
    ("--input", "oklch(1 0 0 / 15%)"),
    ("--ring", "oklch(0.547 0.021 43.1)"),
];

/* ========================================================== */
/*                   ✨ ACCENT COLOR DATA ✨                  */
/* ========================================================== */

// Accent colors only override: primary, primary-foreground, secondary,
// secondary-foreground, chart-1..5, sidebar-primary, sidebar-primary-foreground

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

/* ========================================================== */
/*                        🧪 TESTS 🧪                         */
/* ========================================================== */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_color_from_str_case_insensitive() {
        assert_eq!(BaseColor::from_str("neutral"), Some(BaseColor::Neutral));
        assert_eq!(BaseColor::from_str("NEUTRAL"), Some(BaseColor::Neutral));
        assert_eq!(BaseColor::from_str("Zinc"), Some(BaseColor::Zinc));
        assert_eq!(BaseColor::from_str("unknown"), None);
    }

    #[test]
    fn accent_color_from_str_case_insensitive() {
        assert_eq!(AccentColor::from_str("default"), Some(AccentColor::Default));
        assert_eq!(AccentColor::from_str("Blue"), Some(AccentColor::Blue));
        assert_eq!(AccentColor::from_str("AMBER"), Some(AccentColor::Amber));
        assert_eq!(AccentColor::from_str("unknown"), None);
    }

    #[test]
    fn base_color_from_index_defaults_on_oob() {
        assert_eq!(BaseColor::from_index(0), BaseColor::Neutral);
        assert_eq!(BaseColor::from_index(6), BaseColor::Taupe);
        assert_eq!(BaseColor::from_index(99), BaseColor::Neutral); // default
    }

    #[test]
    fn accent_color_from_index_defaults_on_oob() {
        assert_eq!(AccentColor::from_index(0), AccentColor::Default);
        assert_eq!(AccentColor::from_index(17), AccentColor::Yellow);
        assert_eq!(AccentColor::from_index(99), AccentColor::Default); // default
    }

    #[test]
    fn all_labels_cover_all_variants() {
        assert_eq!(BaseColor::all_labels().len(), BaseColor::ALL.len());
        assert_eq!(AccentColor::all_labels().len(), AccentColor::ALL.len());
    }

    #[test]
    fn generate_theme_vars_neutral_default_contains_expected_vars() {
        let css = generate_theme_vars(BaseColor::Neutral, AccentColor::Default);
        assert!(css.contains(":root {"));
        assert!(css.contains(".dark {"));
        assert!(css.contains("--background: oklch(1 0 0)"));
        assert!(css.contains("--radius: 0.625rem"));
        assert!(css.contains("--destructive: oklch(0.577 0.245 27.325)"));
        assert!(css.contains("--destructive: oklch(0.704 0.191 22.216)"));
    }

    #[test]
    fn generate_theme_vars_accent_overrides_primary() {
        let css = generate_theme_vars(BaseColor::Neutral, AccentColor::Blue);
        // Blue accent primary
        assert!(css.contains("--primary: oklch(0.488 0.243 264.376)"));
        // Chart vars from accent
        assert!(css.contains("--chart-1:"));
        assert!(css.contains("--sidebar-primary:"));
    }

    #[test]
    fn generate_theme_vars_default_accent_has_no_chart_vars() {
        let css = generate_theme_vars(BaseColor::Neutral, AccentColor::Default);
        assert!(!css.contains("--chart-1:"));
        assert!(!css.contains("--sidebar-primary:"));
    }

    #[test]
    fn generate_theme_vars_zinc_base_uses_zinc_background() {
        let css = generate_theme_vars(BaseColor::Zinc, AccentColor::Default);
        // Zinc has a bluish foreground hue
        assert!(css.contains("--background: oklch(0.141 0.005 285.823)"), "dark bg: {css}");
    }

    #[test]
    fn base_color_all_have_matching_labels() {
        for color in BaseColor::ALL {
            let found = BaseColor::from_str(color.label());
            assert_eq!(found, Some(*color), "roundtrip failed for {:?}", color);
        }
    }

    #[test]
    fn accent_color_all_have_matching_labels() {
        for color in AccentColor::ALL {
            let found = AccentColor::from_str(color.label());
            assert_eq!(found, Some(*color), "roundtrip failed for {:?}", color);
        }
    }
}
