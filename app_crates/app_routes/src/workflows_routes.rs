use strum::{AsRefStr, Display, EnumIter, EnumString, IntoStaticStr};

#[derive(Default, Clone, Copy, Display, AsRefStr, IntoStaticStr, EnumString, EnumIter, Debug, PartialEq)]
#[strum(serialize_all = "kebab-case")]
pub enum WorkflowRoutes {
    #[default]
    Workflows,
}

impl WorkflowRoutes {
    pub fn base_segment() -> &'static str {
        "workflows"
    }

    pub fn base_path() -> &'static str {
        "/workflows"
    }

    pub fn to_route(self) -> String {
        format!("/{}", WorkflowRoutes::base_segment())
    }
}
