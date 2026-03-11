use icons::ArrowRight;
use leptos::prelude::*;
use registry::ui::button::{Button, ButtonSize, ButtonVariant};

#[component]
pub fn AnnouncementBadge() -> impl IntoView {
    view! {
        <Button size=ButtonSize::Badge variant=ButtonVariant::Secondary attr:href="https://crates.io/crates/ui-cli">
            <span>Now available: Rust/UI CLI</span>
            <ArrowRight />
        </Button>
    }
}
