use leptos::prelude::*;
use registry::demos::demo_data_table::DemoDataTable;
use registry::ui::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};

#[component]
pub fn CardPaymentsTable() -> impl IntoView {
    view! {
        <Card>
            <CardHeader>
                <CardTitle>Payments</CardTitle>
                <CardDescription>Manage your payments.</CardDescription>
            </CardHeader>
            <CardContent class="pt-0">
                // * @demo
                <DemoDataTable />
            </CardContent>
        </Card>
    }
}
