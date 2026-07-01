use icons::{BookOpenCheck, GitBranch, Search};
use leptos::prelude::*;

use crate::ui::marker::{Marker, MarkerContent, MarkerIcon, MarkerVariant};

#[component]
pub fn DemoMarkerIcon() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-12 py-12 w-full max-w-sm">
            <Marker>
                <MarkerIcon>
                    <GitBranch />
                </MarkerIcon>
                <MarkerContent>"Switched to a new branch"</MarkerContent>
            </Marker>
            <Marker variant=MarkerVariant::Separator>
                <MarkerIcon>
                    <Search />
                </MarkerIcon>
                <MarkerContent>"Explored 4 files"</MarkerContent>
            </Marker>
            // flex-col stacks icon above content
            <Marker class="flex-col">
                <MarkerIcon>
                    <BookOpenCheck />
                </MarkerIcon>
                <MarkerContent>"Syncing completed"</MarkerContent>
            </Marker>
        </div>
    }
}
