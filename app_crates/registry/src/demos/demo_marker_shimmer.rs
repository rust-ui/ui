use leptos::prelude::*;

use crate::ui::marker::{Marker, MarkerContent, MarkerVariant};

#[component]
pub fn DemoMarkerShimmer() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-8 py-12 w-full max-w-sm">
            <Marker role="status">
                <MarkerContent class="shimmer">"Thinking..."</MarkerContent>
            </Marker>
            <Marker variant=MarkerVariant::Separator role="status">
                <MarkerContent class="shimmer">"Reading 4 files"</MarkerContent>
            </Marker>
        </div>
    }
}
