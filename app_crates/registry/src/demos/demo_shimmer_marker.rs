use leptos::prelude::*;

use crate::ui::marker::{Marker, MarkerContent, MarkerIcon, MarkerVariant};
use crate::ui::spinner::Spinner;

#[component]
pub fn DemoShimmerMarker() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4 w-full max-w-sm">
            <Marker role="status">
                <MarkerIcon>
                    <Spinner />
                </MarkerIcon>
                <MarkerContent class="shimmer">"Thinking..."</MarkerContent>
            </Marker>
            <Marker variant=MarkerVariant::Separator role="status">
                <MarkerContent class="shimmer">"Reading 4 files"</MarkerContent>
            </Marker>
        </div>
    }
}
