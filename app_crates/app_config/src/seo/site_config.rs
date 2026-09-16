pub struct SiteConfig;

impl SiteConfig {
    pub const TITLE: &str = "Rust/UI";
    pub const AUTHOR: &str = "Rustify";
    pub const META_DESCRIPTION: &str = "Dioxus Rust UI Components · Build once, run Everywhere.";
    pub const DESCRIPTION: &str = "Rust/UI is a cross-platform component registry for Dioxus and Rust fullstack applications. Beautiful UI components built with Dioxus, Rust, and Tailwind CSS. Build your UI once and deploy to iOS, Android, Desktop, and Web.";
    pub const BASE_URL: &str = "https://rust-ui.com";
    /// Local dev server (dx serve). Matches Dioxus.toml's `[serve] port`.
    pub const BASE_URL_LOCALHOST: &str = "http://localhost:8080";
    pub const TWITTER_IMAGE: &str = "https://rust-ui.com/twitter-card.webp";
}
