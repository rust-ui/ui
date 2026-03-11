use leptos::prelude::*;
use serde::Serialize;

/*
 * title: Line Chart
*/

#[derive(Serialize)]
struct ChartDataPoint {
    label: &'static str,
    value: i32,
}

const CHART_DATA: &[ChartDataPoint] = &[
    ChartDataPoint { label: "Jan", value: 1500 },
    ChartDataPoint { label: "Feb", value: 1800 },
    ChartDataPoint { label: "Mar", value: 2100 },
    ChartDataPoint { label: "Apr", value: 1900 },
    ChartDataPoint { label: "May", value: 2400 },
    ChartDataPoint { label: "Jun", value: 2800 },
    ChartDataPoint { label: "Jul", value: 2600 },
    ChartDataPoint { label: "Aug", value: 3200 },
    ChartDataPoint { label: "Sep", value: 2900 },
    ChartDataPoint { label: "Oct", value: 3500 },
    ChartDataPoint { label: "Nov", value: 3800 },
    ChartDataPoint { label: "Dec", value: 4000 },
];

#[component]
pub fn LineChart01() -> impl IntoView {
    let values: Vec<i32> = CHART_DATA.iter().map(|d| d.value).collect();
    let labels: Vec<&str> = CHART_DATA.iter().map(|d| d.label).collect();

    let data_json = serde_json::to_string(&values).unwrap_or_default();
    let labels_json = serde_json::to_string(&labels).unwrap_or_default();

    view! {
        <section class="flex flex-col gap-9 px-6 mx-auto w-full max-w-4xl rounded-lg border border-border bg-card">
            <p class="mt-4 text-xs font-bold text-card-foreground">Overview</p>

            <div
                id="lineChart01"
                class="w-full h-[400px]"
                data-name="LineChart"
                data-chart-values=data_json
                data-chart-labels=labels_json
            ></div>
        </section>
    }
}
