use strum::{AsRefStr, Display, EnumIter, EnumString, IntoStaticStr};

#[derive(Default, Clone, Copy, Display, AsRefStr, IntoStaticStr, EnumString, EnumIter, Debug, PartialEq, Eq)]
#[strum(serialize_all = "kebab-case")]
pub enum WorkflowRoutes {
    #[default]
    Workflows,
}

impl WorkflowRoutes {
    #[must_use]
    pub const fn base_segment() -> &'static str {
        "workflows"
    }

    #[must_use]
    pub const fn base_path() -> &'static str {
        "/workflows"
    }

    // `self` kept for method-call ergonomics and parity with the other route enums.
    #[allow(clippy::unused_self)]
    #[must_use]
    pub fn to_route(self) -> String {
        format!("/{}", Self::base_segment())
    }
}
