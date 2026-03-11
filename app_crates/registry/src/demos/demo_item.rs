use icons::{BadgeCheck, ChevronRight};
use leptos::prelude::*;

use crate::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::ui::item::{Item, ItemActions, ItemContent, ItemDescription, ItemMedia, ItemSize, ItemTitle, ItemVariant};

#[component]
pub fn DemoItem() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-6 w-full max-w-md">
            <Item variant=ItemVariant::Outline>
                <ItemContent>
                    <ItemTitle>"Basic Item"</ItemTitle>
                    <ItemDescription>"A simple item with title and description."</ItemDescription>
                </ItemContent>
                <ItemActions>
                    <Button variant=ButtonVariant::Outline size=ButtonSize::Sm>
                        "Action"
                    </Button>
                </ItemActions>
            </Item>

            <Item variant=ItemVariant::Outline size=ItemSize::Sm href="#">
                <ItemMedia>
                    <BadgeCheck class="size-5" />
                </ItemMedia>
                <ItemContent>
                    <ItemTitle>"Your profile has been verified."</ItemTitle>
                </ItemContent>
                <ItemActions>
                    <ChevronRight class="size-4" />
                </ItemActions>
            </Item>
        </div>
    }
}
