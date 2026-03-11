use leptos::prelude::*;

use crate::ui::callout::{Callout, CalloutVariant};

#[component]
pub fn DemoCalloutInfo() -> impl IntoView {
    view! {
        <Callout title="Did you know?" variant=CalloutVariant::Info>
            "Components are server-side rendered by default. Use "
            <code>"#[component]"</code>
            " to define reactive Leptos components."
        </Callout>
    }
}
