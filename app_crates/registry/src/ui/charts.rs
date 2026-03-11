use leptos::prelude::*;
use strum::{AsRefStr, Display};
use tw_merge::*;

use crate::hooks::use_random::use_random_id_for;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, AsRefStr, Default)]
pub enum ChartCurve {
    #[default]
    Smooth,
    Straight,
    Stepline,
}

#[component]
pub fn AreaChart(
    #[prop(into)] json_values: Signal<String>,
    #[prop(into)] json_labels: Signal<String>,
    #[prop(default = ChartCurve::default())] curve: ChartCurve,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] series_names: Option<String>,
    #[prop(optional, into)] stack_type: Option<String>,
    #[prop(optional, into)] gradient: Option<bool>,
    #[prop(optional, into)] show_yaxis: Option<bool>,
    #[prop(optional, into)] show_grid: Option<bool>,
    /// JSON array of `{x: "YYYY-MM-DD", title: "..."}` for article publish markers.
    #[prop(optional, into)]
    json_annotations: Option<String>,
) -> impl IntoView {
    let area_chart_id = use_random_id_for("AreaChart");
    let merged_class = Signal::derive(move || tw_merge!("w-full", class.get()));

    view! {
        <div
            id=area_chart_id
            class=move || merged_class.get()
            data-name="AreaChart"
            data-chart-curve=curve.to_string()
            data-chart-values=move || json_values.get()
            data-chart-labels=move || json_labels.get()
            data-chart-series-names=series_names
            data-chart-stack-type=stack_type
            data-chart-gradient=gradient.map(|g| g.to_string())
            data-chart-show-yaxis=show_yaxis.map(|y| y.to_string())
            data-chart-show-grid=show_grid.map(|g| g.to_string())
            data-chart-annotations=json_annotations
        ></div>
    }
}
