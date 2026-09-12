use strum::{AsRefStr, Display, EnumIter, EnumString, IntoStaticStr};

#[derive(Clone, Copy, Display, AsRefStr, IntoStaticStr, EnumString, EnumIter, Debug, PartialEq, Eq)]
#[strum(serialize_all = "kebab-case")]
pub enum ComponentsRoutes {
    AlertDialog,
    Button,
    Breadcrumb,
    Sonner,
}

impl ComponentsRoutes {
    #[must_use]
    pub const fn segment() -> &'static str {
        "components"
    }

    #[must_use]
    pub const fn base_url() -> &'static str {
        "/docs/components"
    }

    #[must_use]
    pub fn to_route(self) -> String {
        format!("{}/{}", Self::base_url(), self.as_ref())
    }
}

#[derive(Clone, Copy, Display, AsRefStr, IntoStaticStr, EnumString, EnumIter, Debug, PartialEq, Eq)]
#[strum(serialize_all = "kebab-case")]
// Route enum variant names are the public API; the `Use` prefix is intentional.
#[allow(clippy::enum_variant_names)]
pub enum HooksRoutes {
    UseCopyClipboard,
    UseLockBodyScroll,
    UseRandom,
}

impl HooksRoutes {
    #[must_use]
    pub const fn segment() -> &'static str {
        "hooks"
    }

    #[must_use]
    pub const fn base_url() -> &'static str {
        "/docs/hooks"
    }

    #[must_use]
    pub fn to_route(self) -> String {
        format!("{}/{}", Self::base_url(), self.as_ref())
    }
}
