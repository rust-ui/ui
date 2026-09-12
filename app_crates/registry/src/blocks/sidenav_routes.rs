use heck::ToTitleCase;
use strum::{AsRefStr, Display, EnumIter, EnumString, IntoStaticStr};

#[derive(Clone, Copy, Debug, Display, AsRefStr, IntoStaticStr, EnumString, EnumIter, PartialEq, Eq)]
#[strum(serialize_all = "kebab-case")]
pub enum SidenavRoutes {
    Sidenav01,
    Sidenav02,
    Sidenav03,
    Sidenav04,
    Sidenav05,
    Sidenav06,
    Sidenav07,
    Sidenav08,
    Sidenav09,
    Sidenav10,
    Sidenav11,
}

impl SidenavRoutes {
    #[must_use]
    pub fn view_segment() -> &'static str {
        "view"
    }

    #[must_use]
    pub fn from_path(path: &str) -> Self {
        use strum::IntoEnumIterator;
        Self::iter()
            .rev()
            .find(|route| path.contains(route.as_ref()))
            .unwrap_or(Self::Sidenav01)
    }

    #[must_use]
    pub fn to_route(self) -> String {
        format!("{}/{}", Self::view_segment(), self.as_ref())
    }
    #[must_use]
    pub fn to_title(self) -> String {
        self.as_ref().to_title_case()
    }
}

#[derive(Clone, Copy, Debug, Display, AsRefStr, IntoStaticStr, EnumString, EnumIter, PartialEq, Eq)]
#[strum(serialize_all = "kebab-case")]
pub enum DocsRoutes {
    Components,
    Hooks,
}

impl DocsRoutes {
    #[must_use]
    pub fn base_segment() -> &'static str {
        "docs"
    }
    #[must_use]
    pub fn to_title(self) -> String {
        self.as_ref().to_title_case()
    }
}

#[derive(Clone, Copy, Debug, Display, AsRefStr, IntoStaticStr, EnumString, EnumIter, PartialEq, Eq)]
#[strum(serialize_all = "kebab-case")]
pub enum ComponentsRoutes {
    Accordion,
    Alert,
    AlertDialog,
    Button,
}

impl ComponentsRoutes {
    #[must_use]
    pub fn base_segment() -> &'static str {
        "components"
    }
    #[must_use]
    pub fn base_url_with_sidenav(sidenav: SidenavRoutes) -> String {
        format!(
            "/{}/{}/{}",
            sidenav.to_route(),
            DocsRoutes::base_segment(),
            Self::base_segment()
        )
    }
    #[must_use]
    pub fn to_route_with_sidenav(self, sidenav: SidenavRoutes) -> String {
        format!("{}/{}", Self::base_url_with_sidenav(sidenav), self.as_ref())
    }
    #[must_use]
    pub fn to_title(self) -> String {
        self.as_ref().to_title_case()
    }
}

#[derive(Clone, Copy, Debug, Display, AsRefStr, IntoStaticStr, EnumString, EnumIter, PartialEq, Eq)]
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
    pub fn base_segment() -> &'static str {
        "hooks"
    }
    #[must_use]
    pub fn base_url_with_sidenav(sidenav: SidenavRoutes) -> String {
        format!(
            "/{}/{}/{}",
            sidenav.to_route(),
            DocsRoutes::base_segment(),
            Self::base_segment()
        )
    }
    #[must_use]
    pub fn to_route_with_sidenav(self, sidenav: SidenavRoutes) -> String {
        format!("{}/{}", Self::base_url_with_sidenav(sidenav), self.as_ref())
    }
    #[must_use]
    pub fn to_title(self) -> String {
        self.as_ref().to_title_case()
    }
}
