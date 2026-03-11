use leptos::prelude::*;
use leptos_ui::void;

use crate::ui::status::{Status, StatusIndactorVariant};

#[component]
pub fn DemoStatusVariants() -> impl IntoView {
    void! {DemoContainer, div, "rounded-md size-16 bg-neutral-500"}

    view! {
        <div class="flex gap-4">
            <Status variant=StatusIndactorVariant::Normal>
                <DemoContainer />
            </Status>
            <Status variant=StatusIndactorVariant::Active>
                <DemoContainer />
            </Status>
            <Status variant=StatusIndactorVariant::Inactive>
                <DemoContainer />
            </Status>
        </div>
    }
}
