use icons::{Check, UserRound, UserRoundMinus};
use leptos::prelude::*;

use crate::ui::marker::{Marker, MarkerContent, MarkerIcon};

#[component]
pub fn DemoMarker() -> impl IntoView {
    view! {
        <div class="flex w-full flex-col gap-3">
            <Marker>
                <MarkerIcon><UserRound /></MarkerIcon>
                <MarkerContent>"Alice joined the conversation"</MarkerContent>
            </Marker>
            <Marker>
                <MarkerIcon><Check /></MarkerIcon>
                <MarkerContent>"Message delivered"</MarkerContent>
            </Marker>
            <Marker>
                <MarkerIcon><UserRoundMinus /></MarkerIcon>
                <MarkerContent>"Bob left the conversation"</MarkerContent>
            </Marker>
        </div>
    }
}
