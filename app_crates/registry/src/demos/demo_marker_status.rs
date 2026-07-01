use leptos::prelude::*;

use crate::ui::marker::{Marker, MarkerContent, MarkerIcon, MarkerVariant};
use crate::ui::spinner::Spinner;

#[component]
pub fn DemoMarkerStatus() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-8 py-12 w-full max-w-sm">
            <Marker role="status">
                <MarkerIcon>
                    <Spinner />
                </MarkerIcon>
                <MarkerContent>"Compacting conversation"</MarkerContent>
            </Marker>
            <Marker variant=MarkerVariant::Separator role="status">
                <MarkerIcon>
                    <Spinner />
                </MarkerIcon>
                <MarkerContent>"Running tests"</MarkerContent>
            </Marker>
        </div>
    }
}
