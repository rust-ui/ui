use heck::ToTitleCase;
use strum::{AsRefStr, Display, EnumIter, EnumString, IntoStaticStr};

#[derive(Default, Clone, Copy, Display, AsRefStr, IntoStaticStr, EnumString, EnumIter, Debug, PartialEq)]
#[strum(serialize_all = "kebab-case")]
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
    pub fn base_segment() -> &'static str {
        "charts"
    }

    pub fn to_route(self) -> String {
        format!("/{}/{}", ChartRoutes::base_segment(), self.as_ref())
    }

    pub fn to_title(self) -> String {
        self.as_ref().to_title_case()
    }
}
