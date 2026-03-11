use leptos::prelude::*;

use crate::ui::button::{Button, ButtonVariant};
use crate::ui::sheet::{
    Sheet, SheetClose, SheetContent, SheetDescription, SheetDirection, SheetFooter, SheetHeader, SheetTitle,
    SheetTrigger,
};

#[component]
pub fn DemoSheet() -> impl IntoView {
    view! {
        <Sheet>
            <SheetTrigger>"Open Sheet"</SheetTrigger>

            <SheetContent direction=SheetDirection::Right>
                <SheetHeader>
                    <SheetTitle>"Edit Profile"</SheetTitle>
                    <SheetDescription>"Make changes to your profile here."</SheetDescription>
                </SheetHeader>

                <div class="p-4 text-sm text-muted-foreground">
                    "Update your display name, avatar, and other profile details."
                </div>

                <SheetFooter>
                    <SheetClose variant=ButtonVariant::Outline>"Cancel"</SheetClose>
                    <Button>"Save changes"</Button>
                </SheetFooter>
            </SheetContent>
        </Sheet>
    }
}
