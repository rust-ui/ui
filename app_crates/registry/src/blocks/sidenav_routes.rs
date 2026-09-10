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
    pub fn view_segment() -> &'static str {
        "view"
    }

    pub fn from_path(path: &str) -> Self {
        use strum::IntoEnumIterator;
        Self::iter().rev().find(|route| path.contains(route.as_ref())).unwrap_or(Self::Sidenav01)
    }

    pub fn to_route(self) -> String {
        format!("{}/{}", Self::view_segment(), self.as_ref())
    }
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
    pub fn base_segment() -> &'static str {
        "docs"
    }
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
    pub fn base_segment() -> &'static str {
        "components"
    }
    pub fn base_url_with_sidenav(sidenav: SidenavRoutes) -> String {
        format!("/{}/{}/{}", sidenav.to_route(), DocsRoutes::base_segment(), Self::base_segment())
    }
    pub fn to_route_with_sidenav(self, sidenav: SidenavRoutes) -> String {
        format!("{}/{}", Self::base_url_with_sidenav(sidenav), self.as_ref())
    }
    pub fn to_title(self) -> String {
        self.as_ref().to_title_case()
    }
}

#[derive(Clone, Copy, Debug, Display, AsRefStr, IntoStaticStr, EnumString, EnumIter, PartialEq, Eq)]
#[strum(serialize_all = "kebab-case")]
pub enum HooksRoutes {
    UseCopyClipboard,
    UseLockBodyScroll,
    UseRandom,
}

impl HooksRoutes {
    pub fn base_segment() -> &'static str {
        "hooks"
    }
    pub fn base_url_with_sidenav(sidenav: SidenavRoutes) -> String {
        format!("/{}/{}/{}", sidenav.to_route(), DocsRoutes::base_segment(), Self::base_segment())
    }
    pub fn to_route_with_sidenav(self, sidenav: SidenavRoutes) -> String {
        format!("{}/{}", Self::base_url_with_sidenav(sidenav), self.as_ref())
    }
    pub fn to_title(self) -> String {
        self.as_ref().to_title_case()
    }
}
