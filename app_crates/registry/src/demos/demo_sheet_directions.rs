use leptos::prelude::*;

use crate::ui::sheet::{
    Sheet, SheetBody, SheetClose, SheetContent, SheetDescription, SheetDirection, SheetTitle, SheetTrigger,
};

#[component]
pub fn DemoSheetDirections() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4 items-center">
            <DemoSheetTop />
            <div class="flex gap-4">
                <DemoSheetLeft />
                <DemoSheetRight />
            </div>
            <DemoSheetBottom />
        </div>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn DemoSheetTop() -> impl IntoView {
    view! {
        <Sheet>
            <SheetTrigger>"Top"</SheetTrigger>
            <SheetContent direction=SheetDirection::Top>
                <SheetBody>
                    <SheetTitle>"Top Sheet"</SheetTitle>
                    <SheetDescription>"This sheet slides from the top."</SheetDescription>
                    <SheetClose>"Close"</SheetClose>
                </SheetBody>
            </SheetContent>
        </Sheet>
    }
}

#[component]
pub fn DemoSheetLeft() -> impl IntoView {
    view! {
        <Sheet>
            <SheetTrigger>"Left"</SheetTrigger>
            <SheetContent direction=SheetDirection::Left>
                <SheetBody>
                    <SheetTitle>"Left Sheet"</SheetTitle>
                    <SheetDescription>"This sheet slides from the left."</SheetDescription>
                    <SheetClose>"Close"</SheetClose>
                </SheetBody>
            </SheetContent>
        </Sheet>
    }
}

#[component]
pub fn DemoSheetRight() -> impl IntoView {
    view! {
        <Sheet>
            <SheetTrigger>"Right"</SheetTrigger>
            <SheetContent direction=SheetDirection::Right>
                <SheetBody>
                    <SheetTitle>"Right Sheet"</SheetTitle>
                    <SheetDescription>"This sheet slides from the right."</SheetDescription>
                    <SheetClose>"Close"</SheetClose>
                </SheetBody>
            </SheetContent>
        </Sheet>
    }
}

#[component]
pub fn DemoSheetBottom() -> impl IntoView {
    view! {
        <Sheet>
            <SheetTrigger>"Bottom"</SheetTrigger>
            <SheetContent direction=SheetDirection::Bottom>
                <SheetBody>
                    <SheetTitle>"Bottom Sheet"</SheetTitle>
                    <SheetDescription>"This sheet slides from the bottom."</SheetDescription>
                    <SheetClose>"Close"</SheetClose>
                </SheetBody>
            </SheetContent>
        </Sheet>
    }
}
