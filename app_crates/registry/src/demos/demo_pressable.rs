use icons::Bell;
use leptos::prelude::*;

use crate::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::ui::card::Card;
use crate::ui::pressable::Pressable;

#[component]
pub fn DemoPressable() -> impl IntoView {
    view! {
        <Pressable>
            <Card class="flex gap-3 items-center py-3 px-4">
                <div class="flex justify-center items-center rounded-full size-9 bg-primary/10">
                    <Bell class="size-4 text-primary" />
                </div>
                <div class="flex-1">
                    <p class="text-sm font-medium">"New message received"</p>
                    <p class="text-xs text-muted-foreground">"2 minutes ago"</p>
                </div>
                <Button variant=ButtonVariant::Outline size=ButtonSize::Sm>
                    "Mark as read"
                </Button>
            </Card>
        </Pressable>
    }
}
