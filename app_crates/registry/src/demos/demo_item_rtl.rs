use icons::{BadgeCheck, ChevronRight};
use leptos::prelude::*;

use crate::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::item::{Item, ItemActions, ItemContent, ItemDescription, ItemMedia, ItemSize, ItemTitle, ItemVariant};

#[component]
pub fn DemoItemRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="w-full max-w-md">
            <div class="flex flex-col gap-6 w-full max-w-md">
                <Item variant=ItemVariant::Outline>
                    <ItemContent>
                        <ItemTitle>"عنصر أساسي"</ItemTitle>
                        <ItemDescription>"عنصر بسيط يحتوي على عنوان ووصف."</ItemDescription>
                    </ItemContent>
                    <ItemActions>
                        <Button variant=ButtonVariant::Outline size=ButtonSize::Sm>
                            "إجراء"
                        </Button>
                    </ItemActions>
                </Item>

                <Item variant=ItemVariant::Outline size=ItemSize::Sm href="#">
                    <ItemMedia>
                        <BadgeCheck class="size-5" />
                    </ItemMedia>
                    <ItemContent>
                        <ItemTitle>"تم التحقق من ملفك الشخصي."</ItemTitle>
                    </ItemContent>
                    <ItemActions>
                        <ChevronRight class="size-4 rtl:rotate-180" />
                    </ItemActions>
                </Item>
            </div>
        </DirectionProvider>
    }
}
