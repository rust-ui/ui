use icons::{ArrowUpRight, FolderCode};
use leptos::prelude::*;

use crate::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::ui::empty::{Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyMediaVariant, EmptyTitle};

#[component]
pub fn DemoEmpty() -> impl IntoView {
    view! {
        <Empty>
            <EmptyHeader>
                <EmptyMedia variant=EmptyMediaVariant::Icon>
                    <FolderCode />
                </EmptyMedia>
                <EmptyTitle>"No Projects Yet"</EmptyTitle>
                <EmptyDescription>
                    "You haven't created any projects yet. Get started by creating your first project."
                </EmptyDescription>
            </EmptyHeader>
            <EmptyContent>
                <div class="flex gap-2">
                    <Button>"Create Project"</Button>
                    <Button variant=ButtonVariant::Outline>"Import Project"</Button>
                </div>
            </EmptyContent>
            <Button variant=ButtonVariant::Link size=ButtonSize::Sm class="text-muted-foreground">
                <a href="#" class="flex gap-1 items-center">
                    <span>"Learn More"</span>
                    <ArrowUpRight />
                </a>
            </Button>
        </Empty>
    }
}
