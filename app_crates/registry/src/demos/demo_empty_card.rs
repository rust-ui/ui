use icons::{ArrowUpRight, Folder};
use leptos::prelude::*;

use crate::ui::button::{Button, ButtonVariant};
use crate::ui::card::{Card, CardContent};
use crate::ui::empty::{Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyMediaVariant, EmptyTitle};

#[component]
pub fn DemoEmptyCard() -> impl IntoView {
    view! {
        <Card class="w-full max-w-md">
            <CardContent class="p-0">
                <Empty>
                    <EmptyHeader>
                        <EmptyMedia variant=EmptyMediaVariant::Icon>
                            <Folder />
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
                        <Button variant=ButtonVariant::Link class="text-muted-foreground">
                            <a href="#" class="flex gap-1 items-center">
                                <span>"Learn More"</span>
                                <ArrowUpRight />
                            </a>
                        </Button>
                    </EmptyContent>
                </Empty>
            </CardContent>
        </Card>
    }
}
