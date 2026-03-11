use leptos::prelude::*;
use serde::Serialize;

/*
 * title: Radial Chart
*/

#[derive(Serialize)]
struct ChartDataPoint {
    label: &'static str,
    value: i32,
}

const CHART_DATA: &[ChartDataPoint] = &[
    ChartDataPoint { label: "Project A", value: 76 },
    ChartDataPoint { label: "Project B", value: 67 },
    ChartDataPoint { label: "Project C", value: 61 },
    ChartDataPoint { label: "Project D", value: 90 },
];

#[component]
pub fn RadialChart01() -> impl IntoView {
    let values: Vec<i32> = CHART_DATA.iter().map(|d| d.value).collect();
    let labels: Vec<&str> = CHART_DATA.iter().map(|d| d.label).collect();

    let data_json = serde_json::to_string(&values).unwrap_or_default();
    let labels_json = serde_json::to_string(&labels).unwrap_or_default();

    view! {
        <section class="flex flex-col gap-9 px-6 mx-auto w-full max-w-4xl rounded-lg border border-border bg-card">
            <p class="mt-4 text-xs font-bold text-card-foreground">Completion Rates</p>

            <div
                id="radialChart01"
                class="w-full h-[400px]"
                data-name="RadialChart"
                data-chart-values=data_json
                data-chart-labels=labels_json
            ></div>
        </section>
    }
}
