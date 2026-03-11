#[derive(PartialEq)]
pub struct RoutePaths;

impl RoutePaths {
    pub const HOME: &'static str = "/";
    pub const TEST: &'static str = "/test";
    pub const TO_FIX: &'static str = "/to-fix";
    pub const CSS_TRICKS: &'static str = "/css-tricks";
    pub const FLEX_TRANSITION: &'static str = "/flex-transition";
    pub const CHARTS: &'static str = "/charts";

    pub const ICONS: &'static str = "/icons";
    pub const BLOCKS: &'static str = "/blocks";
    pub const DOWNLOAD: &'static str = "/download";
    pub const CREATE: &'static str = "/create";
    pub const DOCS_INTRODUCTION: &'static str = "/docs/components/introduction";
    pub const DOCS_INSTALLATION: &'static str = "/docs/components/installation";
    pub const DOCS_CLI: &'static str = "/docs/components/cli";
}
