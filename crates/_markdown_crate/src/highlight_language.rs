use strum::{AsRefStr, IntoStaticStr};

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[derive(Debug, Clone, Copy, PartialEq, Default, IntoStaticStr, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum HighlightLanguage {
    #[default]
    Rust,
    Json,
    Css,
    Toml,
    Bash,
}

impl HighlightLanguage {
    fn from_filename(filename: &str) -> Self {
        let extension = filename.split('.').next_back().unwrap_or("");
        match extension.to_lowercase().as_str() {
            "rs" => Self::Rust,
            "json" => Self::Json,
            "css" => Self::Css,
            "toml" => Self::Toml,
            "sh" | "bash" => Self::Bash,
            _ => Self::default(),
        }
    }

    pub fn detect_from_filename(filename: &str) -> Option<&str> {
        Some(Self::from_filename(filename).into())
    }
}
