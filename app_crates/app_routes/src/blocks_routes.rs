use strum::{AsRefStr, Display, EnumIter, EnumString, IntoStaticStr};

#[derive(Default, Clone, Copy, Display, AsRefStr, IntoStaticStr, EnumString, EnumIter, Debug, PartialEq, Eq)]
#[strum(serialize_all = "kebab-case")]
pub enum BlockRoutes {
    #[default]
    Login,
    Sidenav,
    Headers,
    Footers,
    Faq,
    Integrations,
}

impl BlockRoutes {
    #[must_use]
    pub const fn base_segment() -> &'static str {
        "blocks"
    }

    #[must_use]
    pub const fn base_path() -> &'static str {
        "/blocks"
    }

    #[must_use]
    pub fn to_route(self) -> String {
        format!("/{}/{}", Self::base_segment(), self.as_ref())
    }

    #[must_use]
    pub fn to_view_route_kebab(self, num: u8) -> String {
        format!("/view/{}", self.into_kebab(num))
    }

    fn into_kebab(self, num: u8) -> String {
        format!("{}-{:02}", self.as_ref(), num)
    }
}
