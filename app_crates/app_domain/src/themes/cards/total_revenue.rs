use leptos::prelude::*;
use registry::ui::card::{Card, CardContent, CardHeader, CardTitle};

#[component]
pub fn CardTotalRevenue() -> impl IntoView {
    view! {
        <Card>
            <CardHeader class="flex-row justify-between items-center pb-2">
                <CardTitle class="text-base font-normal">Total Revenue</CardTitle>
            </CardHeader>
            <CardContent class="pt-0">
                <div class="text-2xl font-bold">$15,231.89</div>
                <p class="text-xs text-muted-foreground">+20.1% from last month</p>
                <div class="h-[80px]">
                    // * See below
                    <ChartTotalRevenue />
                </div>
            </CardContent>
        </Card>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
fn ChartTotalRevenue() -> impl IntoView {
    let values = r#"[186,305,237,273,209,237,264,486]"#;
    let labels = r#"["","","","","","","",""]"#;

    view! {
        <div
            class="w-full h-full"
            data-name="LineChart"
            data-chart-values=values
            data-chart-labels=labels
            data-chart-sparkline="true"
            data-chart-markers="true"
        ></div>
    }
}
