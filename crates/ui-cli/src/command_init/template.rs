use super::colors::{AccentColor, BaseColor, generate_theme_vars};

pub struct MyTemplate;

impl MyTemplate {
    const CSS_HEADER: &str = "@import \"tailwindcss\";\n@import \"tw-animate-css\";\n\n";

    const CSS_FOOTER: &str = r#"
@theme inline {
  --color-background: var(--background);
  --color-foreground: var(--foreground);
  --color-card: var(--card);
  --color-card-foreground: var(--card-foreground);
  --color-popover: var(--popover);
  --color-popover-foreground: var(--popover-foreground);
  --color-primary: var(--primary);
  --color-primary-foreground: var(--primary-foreground);
  --color-secondary: var(--secondary);
  --color-secondary-foreground: var(--secondary-foreground);
  --color-muted: var(--muted);
  --color-muted-foreground: var(--muted-foreground);
  --color-accent: var(--accent);
  --color-accent-foreground: var(--accent-foreground);
  --color-destructive: var(--destructive);
  --color-destructive-foreground: var(--destructive-foreground);
  --color-border: var(--border);
  --color-input: var(--input);
  --color-ring: var(--ring);
  --radius-sm: calc(var(--radius) - 4px);
  --radius-md: calc(var(--radius) - 2px);
  --radius-lg: var(--radius);
  --radius-xl: calc(var(--radius) + 4px);
}

@layer base {
  * {
    @apply border-border outline-ring/50;
  }
  body {
    @apply bg-background text-foreground;
  }

  button:not(:disabled),
  [role="button"]:not(:disabled) {
    cursor: pointer;
  }

  dialog {
    margin: auto;
  }
}
"#;

    /// Build a complete tailwind.css from the chosen base + accent colors.
    pub fn build_css(base: BaseColor, accent: AccentColor) -> String {
        format!("{}{}{}", Self::CSS_HEADER, generate_theme_vars(base, accent), Self::CSS_FOOTER)
    }

    pub const PACKAGE_JSON: &str = r#"{
	"type": "module"
}
"#;
}

/* ========================================================== */
/*                        🧪 TESTS 🧪                         */
/* ========================================================== */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_css_contains_tailwind_import() {
        let css = MyTemplate::build_css(BaseColor::default(), AccentColor::default());
        assert!(css.contains("@import \"tailwindcss\""));
        assert!(css.contains("@import \"tw-animate-css\""));
    }

    #[test]
    fn build_css_contains_theme_inline_block() {
        let css = MyTemplate::build_css(BaseColor::default(), AccentColor::default());
        assert!(css.contains("@theme inline {"));
        assert!(css.contains("--color-background: var(--background)"));
    }

    #[test]
    fn build_css_contains_layer_base() {
        let css = MyTemplate::build_css(BaseColor::default(), AccentColor::default());
        assert!(css.contains("@layer base {"));
    }

    #[test]
    fn build_css_contains_color_vars() {
        let css = MyTemplate::build_css(BaseColor::default(), AccentColor::default());
        assert!(css.contains(":root {"));
        assert!(css.contains(".dark {"));
        assert!(css.contains("--radius: 0.625rem"));
    }

    #[test]
    fn build_css_zinc_blue_has_zinc_background() {
        let css = MyTemplate::build_css(BaseColor::Zinc, AccentColor::Blue);
        // Zinc dark background
        assert!(css.contains("--background: oklch(0.141 0.005 285.823)"));
        // Blue accent primary
        assert!(css.contains("--primary: oklch(0.488 0.243 264.376)"));
    }
}
