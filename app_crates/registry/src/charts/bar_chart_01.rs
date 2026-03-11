use leptos::prelude::*;
use serde::Serialize;

/*
 * title: Bar Chart
*/

#[derive(Serialize)]
struct ChartDataPoint {
    label: &'static str,
    value: i32,
}

const CHART_DATA: &[ChartDataPoint] = &[
    ChartDataPoint { label: "Jan", value: 2500 },
    ChartDataPoint { label: "Feb", value: -230 },
    ChartDataPoint { label: "Mar", value: 1800 },
    ChartDataPoint { label: "Apr", value: 3200 },
    ChartDataPoint { label: "May", value: 2100 },
    ChartDataPoint { label: "Jun", value: -560 },
    ChartDataPoint { label: "Jul", value: 800 },
    ChartDataPoint { label: "Aug", value: 4600 },
    ChartDataPoint { label: "Sep", value: -1545 },
    ChartDataPoint { label: "Oct", value: 4800 },
    ChartDataPoint { label: "Nov", value: 3500 },
    ChartDataPoint { label: "Dec", value: 3000 },
];

#[component]
pub fn BarChart01() -> impl IntoView {
    let values: Vec<i32> = CHART_DATA.iter().map(|d| d.value).collect();
    let labels: Vec<&str> = CHART_DATA.iter().map(|d| d.label).collect();

    let data_json = serde_json::to_string(&values).unwrap_or_default();
    let labels_json = serde_json::to_string(&labels).unwrap_or_default();

    view! {
        <section class="flex flex-col gap-9 px-6 mx-auto w-full max-w-4xl rounded-lg border border-border bg-card">
            <p class="mt-4 text-xs font-bold text-card-foreground">Overview</p>

            <div
                id="barChart01"
                class="w-full h-[400px]"
                data-name="BarChart"
                data-chart-values=data_json
                data-chart-labels=labels_json
            ></div>
        </section>
    }
}
