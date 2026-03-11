use leptos::prelude::*;

use crate::ui::callout::Callout;

#[component]
pub fn DemoCallout() -> impl IntoView {
    view! { <Callout title="Note">"You can add components and dependencies to your app using the CLI."</Callout> }
}
