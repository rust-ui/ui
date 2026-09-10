use dioxus::prelude::*;
use tw_merge::tw_merge;

use crate::hooks::use_random::use_random_id_for;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ChartCurve {
    #[default]
    Smooth,
    Straight,
    Stepline,
}

impl ChartCurve {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Smooth => "Smooth",
            Self::Straight => "Straight",
            Self::Stepline => "Stepline",
        }
    }
}

#[component]
pub fn AreaChart(
    json_values: String,
    json_labels: String,
    #[props(default)] curve: ChartCurve,
    #[props(into, optional)] class: Option<String>,
    #[props(into, optional)] series_names: Option<String>,
    #[props(into, optional)] stack_type: Option<String>,
    #[props(optional)] gradient: Option<bool>,
    #[props(optional)] show_yaxis: Option<bool>,
    #[props(optional)] show_grid: Option<bool>,
    #[props(into, optional)] json_annotations: Option<String>,
) -> Element {
    let chart_id = use_random_id_for("AreaChart");
    let merged_class = tw_merge!("w-full", class.as_deref().unwrap_or(""));

    rsx! {
        div {
            id: chart_id,
            class: merged_class,
            "data-name": "AreaChart",
            "data-chart-curve": curve.as_str(),
            "data-chart-values": json_values,
            "data-chart-labels": json_labels,
            "data-chart-series-names": series_names.unwrap_or_default(),
            "data-chart-stack-type": stack_type.unwrap_or_default(),
            "data-chart-gradient": gradient
                .map(|g| if g { "true" } else { "false" })
                .unwrap_or_default(),
            "data-chart-show-yaxis": show_yaxis
                .map(|y| if y { "true" } else { "false" })
                .unwrap_or_default(),
            "data-chart-show-grid": show_grid
                .map(|g| if g { "true" } else { "false" })
                .unwrap_or_default(),
            "data-chart-annotations": json_annotations.unwrap_or_default(),
        }
    }
}
