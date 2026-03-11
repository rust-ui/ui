use leptos::prelude::*;
use registry::ui::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};

#[component]
pub fn CardExerciseMinutes() -> impl IntoView {
    view! {
        <div class="pt-3 sm:col-span-2 xl:pt-3">
            <Card>
                <CardHeader>
                    <CardTitle>Exercise Minutes</CardTitle>
                    <CardDescription>Your exercise minutes are ahead of where you normally are.</CardDescription>
                </CardHeader>
                <CardContent class="pb-4">
                    <div class="h-[200px]">
                        // * See below
                        <ChartExerciseMinutes />
                    </div>
                </CardContent>
            </Card>
        </div>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
fn ChartExerciseMinutes() -> impl IntoView {
    // Two series: this week (ahead) vs average
    let values = r#"[[30,45,35,60,55,65,75],[25,30,40,35,45,50,55]]"#;
    let labels = r#"["Mon","Tue","Wed","Thu","Fri","Sat","Sun"]"#;
    let series_names = r#"["This Week","Average"]"#;

    view! {
        <div
            class="w-full h-full"
            data-name="LineChart"
            data-chart-values=values
            data-chart-labels=labels
            data-chart-series-names=series_names
            data-chart-show-grid="true"
            data-chart-show-yaxis="false"
        ></div>
    }
}
