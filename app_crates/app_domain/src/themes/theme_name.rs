use super::components::color_theme_picker::ColorTheme;
use super::components::font_picker::FontName;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ThemeName {
    #[default]
    Neutral,
    Stone,
    Zinc,
    Mauve,
    Olive,
    Mist,
    Taupe,
}

impl ThemeName {
    pub const ALL: &'static [Self] = &[
        Self::Neutral,
        Self::Stone,
        Self::Zinc,
        Self::Mauve,
        Self::Olive,
        Self::Mist,
        Self::Taupe,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            Self::Neutral => "Neutral",
            Self::Stone => "Stone",
            Self::Zinc => "Zinc",
            Self::Mauve => "Mauve",
            Self::Olive => "Olive",
            Self::Mist => "Mist",
            Self::Taupe => "Taupe",
        }
    }

    pub fn swatch(&self) -> &'static str {
        match self {
            Self::Neutral => "#737373",
            Self::Stone => "#79716b",
            Self::Zinc => "#71717a",
            Self::Mauve => "#7c6e7c",
            Self::Olive => "#6b7045",
            Self::Mist => "#5f7070",
            Self::Taupe => "#736050",
        }
    }

    pub fn to_index(self) -> usize {
        match self {
            Self::Neutral => 0,
            Self::Stone => 1,
            Self::Zinc => 2,
            Self::Mauve => 3,
            Self::Olive => 4,
            Self::Mist => 5,
            Self::Taupe => 6,
        }
    }

    pub fn from_index(idx: u32) -> Option<Self> {
        match idx {
            0 => Some(Self::Neutral),
            1 => Some(Self::Stone),
            2 => Some(Self::Zinc),
            3 => Some(Self::Mauve),
            4 => Some(Self::Olive),
            5 => Some(Self::Mist),
            6 => Some(Self::Taupe),
            _ => None,
        }
    }

    pub fn light_vars(&self) -> &'static [(&'static str, &'static str)] {
        match self {
            Self::Neutral => NEUTRAL_LIGHT,
            Self::Stone => STONE_LIGHT,
            Self::Zinc => ZINC_LIGHT,
            Self::Mauve => MAUVE_LIGHT,
            Self::Olive => OLIVE_LIGHT,
            Self::Mist => MIST_LIGHT,
            Self::Taupe => TAUPE_LIGHT,
        }
    }

    pub fn dark_vars(&self) -> &'static [(&'static str, &'static str)] {
        match self {
            Self::Neutral => NEUTRAL_DARK,
            Self::Stone => STONE_DARK,
            Self::Zinc => ZINC_DARK,
            Self::Mauve => MAUVE_DARK,
            Self::Olive => OLIVE_DARK,
            Self::Mist => MIST_DARK,
            Self::Taupe => TAUPE_DARK,
        }
    }

    pub fn css_string(&self, radius: f32, color_theme: ColorTheme, font: FontName) -> String {
        let mut out = format!(
            ":root {{\n  --radius: {radius}rem;\n  --font-sans: {};\n",
            font.css_value()
        );
        for (k, v) in self.light_vars() {
            out.push_str(&format!("  {k}: {v};\n"));
        }
        for (k, v) in color_theme.light_vars() {
            out.push_str(&format!("  {k}: {v};\n"));
        }
        out.push_str("}\n\n.dark {\n");
        for (k, v) in self.dark_vars() {
            out.push_str(&format!("  {k}: {v};\n"));
        }
        for (k, v) in color_theme.dark_vars() {
            out.push_str(&format!("  {k}: {v};\n"));
        }
        out.push('}');
        out
    }
}

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
