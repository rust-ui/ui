use icons::{Bell, RefreshCcw};
use leptos::prelude::*;

use crate::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::ui::empty::{Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyMediaVariant, EmptyTitle};

#[component]
pub fn DemoEmptyBackground() -> impl IntoView {
    view! {
        <Empty class="bg-muted/30">
            <EmptyHeader>
                <EmptyMedia variant=EmptyMediaVariant::Icon>
                    <Bell />
                </EmptyMedia>
                <EmptyTitle>"No Notifications"</EmptyTitle>
                <EmptyDescription>"You're all caught up. New notifications will appear here."</EmptyDescription>
            </EmptyHeader>
            <EmptyContent>
                <Button variant=ButtonVariant::Outline size=ButtonSize::Sm>
                    <RefreshCcw />
                    "Refresh"
                </Button>
            </EmptyContent>
        </Empty>
    }
}
