use heck::ToTitleCase;
use strum::{AsRefStr, Display, EnumIter, EnumString, IntoStaticStr};

#[derive(Default, Clone, Copy, Display, AsRefStr, IntoStaticStr, EnumString, EnumIter, Debug, PartialEq, Eq)]
#[strum(serialize_all = "kebab-case")]
// Route enum variant names are the public API; the `Chart` suffix is intentional.
#[allow(clippy::enum_variant_names)]
pub enum ChartRoutes {
    #[default]
    AreaChart,
    BarChart,
    LineChart,
    PieChart,
    RadarChart,
    RadialChart,
}

impl ChartRoutes {
    pub const ALL: &'static [Self] = &[
        Self::AreaChart,
        Self::BarChart,
        Self::LineChart,
        Self::PieChart,
        Self::RadarChart,
        Self::RadialChart,
    ];

    #[must_use]
    pub const fn base_segment() -> &'static str {
        "charts"
    }

    #[must_use]
    pub fn to_route(self) -> String {
        format!("/{}/{}", Self::base_segment(), self.as_ref())
    }

    #[must_use]
    pub fn to_title(self) -> String {
        self.as_ref().to_title_case()
    }
}
