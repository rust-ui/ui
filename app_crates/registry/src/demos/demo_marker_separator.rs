use leptos::prelude::*;

use crate::ui::marker::{Marker, MarkerContent, MarkerVariant};

#[component]
pub fn DemoMarkerSeparator() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-8 py-12 w-full max-w-sm">
            <Marker variant=MarkerVariant::Separator>
                <MarkerContent>"Today"</MarkerContent>
            </Marker>
            <Marker variant=MarkerVariant::Separator>
                <MarkerContent>"Worked for 42s"</MarkerContent>
            </Marker>
            <Marker variant=MarkerVariant::Separator>
                <MarkerContent>"Conversation compacted"</MarkerContent>
            </Marker>
        </div>
    }
}
