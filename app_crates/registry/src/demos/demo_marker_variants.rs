use leptos::prelude::*;

use crate::ui::marker::{Marker, MarkerContent, MarkerVariant};

#[component]
pub fn DemoMarkerVariants() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-8 py-12 w-full max-w-sm">
            <Marker>
                <MarkerContent>"A default marker for inline notes."</MarkerContent>
            </Marker>
            <Marker variant=MarkerVariant::Separator>
                <MarkerContent>"A separator marker"</MarkerContent>
            </Marker>
            <Marker variant=MarkerVariant::Border>
                <MarkerContent>"A border marker for row boundaries."</MarkerContent>
            </Marker>
        </div>
    }
}
