use leptos::prelude::*;

use crate::ui::callout::{Callout, CalloutVariant};

#[component]
pub fn DemoCalloutWarning() -> impl IntoView {
    view! {
        <Callout title="Breaking Change" variant=CalloutVariant::Warning>
            "This API changed in v2.0. Update your imports from "
            <code>"leptos_ui::old"</code>
            " to "
            <code>"leptos_ui::ui"</code>
            "."
        </Callout>
    }
}
