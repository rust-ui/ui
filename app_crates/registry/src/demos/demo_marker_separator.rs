use icons::{Check, Link};
use leptos::prelude::*;

use crate::ui::marker::{Marker, MarkerContent, MarkerIcon, MarkerVariant};
use crate::ui::spinner::Spinner;

#[component]
pub fn DemoMarkerSeparator() -> impl IntoView {
    view! {
        <div class="flex w-full flex-col gap-4">
            // Spinner + shimmer text (streaming / thinking state)
            <Marker variant=MarkerVariant::Separator>
                <MarkerIcon><Spinner /></MarkerIcon>
                <MarkerContent>
                    <span class="shimmer">"Thinking…"</span>
                </MarkerContent>
            </Marker>

            // Check icon (completed)
            <Marker variant=MarkerVariant::Separator>
                <MarkerIcon><Check /></MarkerIcon>
                <MarkerContent>"Done"</MarkerContent>
            </Marker>

            // Link inside content
            <Marker variant=MarkerVariant::Separator>
                <MarkerContent>
                    <Link class="inline size-3" />
                    " "
                    <a href="#">"View sources"</a>
                </MarkerContent>
            </Marker>

            // Plain text separator
            <Marker variant=MarkerVariant::Separator>
                <MarkerContent>"Today"</MarkerContent>
            </Marker>
        </div>
    }
}
