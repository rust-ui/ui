use icons::{Cloud, Upload};
use leptos::prelude::*;

use crate::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::ui::empty::{Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyMediaVariant, EmptyTitle};

#[component]
pub fn DemoEmptyOutline() -> impl IntoView {
    view! {
        <Empty>
            <EmptyHeader>
                <EmptyMedia variant=EmptyMediaVariant::Icon>
                    <Cloud />
                </EmptyMedia>
                <EmptyTitle>"Cloud Storage Empty"</EmptyTitle>
                <EmptyDescription>"Upload files to your cloud storage to access them anywhere."</EmptyDescription>
            </EmptyHeader>
            <EmptyContent>
                <Button variant=ButtonVariant::Outline size=ButtonSize::Sm>
                    <Upload />
                    "Upload Files"
                </Button>
            </EmptyContent>
        </Empty>
    }
}
