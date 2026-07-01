use icons::{FileText, GitBranch, Search};
use leptos::prelude::*;

use crate::ui::marker::{Marker, MarkerContent, MarkerIcon, MarkerVariant};

#[component]
pub fn DemoMarkerBorder() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-3 py-12 w-full max-w-sm">
            <Marker variant=MarkerVariant::Border>
                <MarkerIcon>
                    <GitBranch />
                </MarkerIcon>
                <MarkerContent>"Switched to release-candidate"</MarkerContent>
            </Marker>
            <Marker variant=MarkerVariant::Border>
                <MarkerIcon>
                    <Search />
                </MarkerIcon>
                <MarkerContent>"Reviewed 8 related files"</MarkerContent>
            </Marker>
            <Marker variant=MarkerVariant::Border>
                <MarkerIcon>
                    <FileText />
                </MarkerIcon>
                <MarkerContent>"Opened implementation notes"</MarkerContent>
            </Marker>
        </div>
    }
}
