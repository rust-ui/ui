use leptos::prelude::*;

use crate::ui::drawer::{
    Drawer, DrawerBody, DrawerClose, DrawerContent, DrawerDescription, DrawerHandle, DrawerHeader, DrawerTitle,
    DrawerTrigger,
};

#[component]
pub fn DemoDrawerNonDismissable() -> impl IntoView {
    view! {
        <Drawer>
            <DrawerTrigger>"Open Drawer"</DrawerTrigger>

            <DrawerContent style="--initial-transform: 100%; pointer-events: auto;" dismissible="false">
                <DrawerHandle />

                <DrawerBody>
                    <DrawerHeader>
                        <DrawerTitle>"Are you absolutely sure?"</DrawerTitle>
                        <DrawerDescription>
                            "This action cannot be undone. This will permanently delete your account and remove your data from our servers."
                        </DrawerDescription>
                    </DrawerHeader>

                    <DrawerClose>"Confirm"</DrawerClose>
                </DrawerBody>
            </DrawerContent>
        </Drawer>
    }
}
