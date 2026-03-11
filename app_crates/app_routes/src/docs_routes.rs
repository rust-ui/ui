use strum::{AsRefStr, Display, EnumIter, EnumString, IntoStaticStr};

#[derive(Clone, Copy, Display, AsRefStr, IntoStaticStr, EnumString, EnumIter, Debug, PartialEq)]
#[strum(serialize_all = "kebab-case")]
pub enum ComponentsRoutes {
    AlertDialog,
    Button,
    Breadcrumb,
    Sonner,
}

impl ComponentsRoutes {
    pub fn segment() -> &'static str {
        "components"
    }

    pub fn base_url() -> &'static str {
        "/docs/components"
    }

    pub fn to_route(self) -> String {
        format!("{}/{}", ComponentsRoutes::base_url(), self.as_ref())
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[derive(Clone, Copy, Display, AsRefStr, IntoStaticStr, EnumString, EnumIter, Debug, PartialEq)]
#[strum(serialize_all = "kebab-case")]
pub enum HooksRoutes {
    UseCopyClipboard,
    UseLockBodyScroll,
    UseRandom,
}

impl HooksRoutes {
    pub fn segment() -> &'static str {
        "hooks"
    }

    pub fn base_url() -> &'static str {
        "/docs/hooks"
    }

    pub fn to_route(self) -> String {
        format!("{}/{}", HooksRoutes::base_url(), self.as_ref())
    }
}
