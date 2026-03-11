use leptos::prelude::*;

use crate::ui::button::{Button, ButtonVariant};
use crate::ui::sheet::{
    Sheet, SheetClose, SheetContent, SheetDescription, SheetDirection, SheetFooter, SheetHeader, SheetTitle,
    SheetTrigger,
};

#[component]
pub fn DemoSheetNoCloseButton() -> impl IntoView {
    view! {
        <Sheet>
            <SheetTrigger>"Open Sheet"</SheetTrigger>

            <SheetContent direction=SheetDirection::Right show_close_button=false>
                <SheetHeader>
                    <SheetTitle>"No Close Button"</SheetTitle>
                    <SheetDescription>"This sheet hides the default close button."</SheetDescription>
                </SheetHeader>

                <div class="p-4 text-sm text-muted-foreground">"Use the Cancel button or press ESC to close."</div>

                <SheetFooter>
                    <SheetClose variant=ButtonVariant::Outline>"Cancel"</SheetClose>
                    <Button>"Confirm"</Button>
                </SheetFooter>
            </SheetContent>
        </Sheet>
    }
}
