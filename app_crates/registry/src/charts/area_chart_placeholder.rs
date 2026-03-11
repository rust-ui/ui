use leptos::prelude::*;

use crate::ui::card::Card;

/*
 * title: Area Chart - Placeholder
*/

#[component]
pub fn AreaChartPlaceholder() -> impl IntoView {
    view! {
        <Card class="hidden border-dashed shadow-none md:block h-[400px] bg-muted">
            <div />
        </Card>
    }
}
