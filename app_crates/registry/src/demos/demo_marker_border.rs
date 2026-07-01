use icons::{FileText, GitBranch, NotepadText};
use leptos::prelude::*;

use crate::ui::marker::{Marker, MarkerContent, MarkerIcon, MarkerVariant};

#[component]
pub fn DemoMarkerBorder() -> impl IntoView {
    view! {
        <div class="flex w-full flex-col gap-3">
            <Marker variant=MarkerVariant::Border>
                <MarkerIcon><GitBranch /></MarkerIcon>
                <MarkerContent>"Switched to branch " <strong>"feat/streaming"</strong></MarkerContent>
            </Marker>
            <Marker variant=MarkerVariant::Border>
                <MarkerIcon><FileText /></MarkerIcon>
                <MarkerContent>"Reviewing " <strong>"3 files"</strong> " changed"</MarkerContent>
            </Marker>
            <Marker variant=MarkerVariant::Border>
                <MarkerIcon><NotepadText /></MarkerIcon>
                <MarkerContent>"Notes opened"</MarkerContent>
            </Marker>
        </div>
    }
}
