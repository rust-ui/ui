use leptos::prelude::*;

use crate::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::ui::item::{Item, ItemActions, ItemContent, ItemDescription, ItemTitle, ItemVariant};

#[component]
pub fn DemoItemVariants() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-6">
            <Item>
                <ItemContent>
                    <ItemTitle>"Default Variant"</ItemTitle>
                    <ItemDescription>"Standard styling with subtle background and borders."</ItemDescription>
                </ItemContent>
                <ItemActions>
                    <Button variant=ButtonVariant::Outline size=ButtonSize::Sm>
                        "Open"
                    </Button>
                </ItemActions>
            </Item>

            <Item variant=ItemVariant::Outline>
                <ItemContent>
                    <ItemTitle>"Outline Variant"</ItemTitle>
                    <ItemDescription>"Outlined style with clear borders and transparent background."</ItemDescription>
                </ItemContent>
                <ItemActions>
                    <Button variant=ButtonVariant::Outline size=ButtonSize::Sm>
                        "Open"
                    </Button>
                </ItemActions>
            </Item>

            <Item variant=ItemVariant::Muted>
                <ItemContent>
                    <ItemTitle>"Muted Variant"</ItemTitle>
                    <ItemDescription>"Subdued appearance with muted colors for secondary content."</ItemDescription>
                </ItemContent>
                <ItemActions>
                    <Button variant=ButtonVariant::Outline size=ButtonSize::Sm>
                        "Open"
                    </Button>
                </ItemActions>
            </Item>
        </div>
    }
}
