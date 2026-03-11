use leptos::prelude::*;
use registry::ui::card::{Card, CardContent, CardHeader, CardTitle};

#[component]
pub fn CardSubscriptions() -> impl IntoView {
    view! {
        <Card>
            <CardHeader class="flex-row justify-between items-center pb-2">
                <CardTitle class="text-base font-normal">Subscriptions</CardTitle>
            </CardHeader>
            <CardContent class="pt-0">
                <div class="text-2xl font-bold">+2350</div>
                <p class="text-xs text-muted-foreground">+180.1% from last month</p>
                <div class="mt-4 h-[80px]">
                    // * See below
                    <ChartSubscriptions />
                </div>
            </CardContent>
        </Card>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
fn ChartSubscriptions() -> impl IntoView {
    let values = r#"[560,700,467,649,441,558,649,441]"#;
    let labels = r#"["","","","","","","",""]"#;

    view! {
        <div
            id="chartSubscriptions"
            class="w-full h-full"
            data-name="BarChart"
            data-chart-values=values
            data-chart-labels=labels
            data-chart-show-xaxis="false"
            data-chart-show-yaxis="false"
        ></div>
    }
}
