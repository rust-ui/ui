use leptos::prelude::*;

use crate::ui::drawer::{
    Drawer, DrawerBody, DrawerClose, DrawerContent, DrawerDescription, DrawerHandle, DrawerHeader, DrawerTitle,
    DrawerTrigger,
};
use crate::ui::input::Input;
use crate::ui::label::Label;
use crate::ui::textarea::Textarea;

#[component]
pub fn DemoDrawerFocus() -> impl IntoView {
    view! {
        <Drawer>
            <DrawerTrigger>"Open Drawer"</DrawerTrigger>

            <DrawerContent>
                <DrawerHandle />

                <DrawerBody>
                    <DrawerHeader>
                        <DrawerTitle>"Focus Drawer"</DrawerTitle>
                        <DrawerDescription>
                            "Test keyboard navigation: Press Tab to cycle through elements, Shift+Tab to go back, and Escape to close."
                        </DrawerDescription>
                    </DrawerHeader>

                    <form class="flex flex-col gap-4">
                        <div class="flex flex-col gap-2">
                            <Label html_for="test-input">"Text Input"</Label>
                            <Input attr:id="test-input" attr:placeholder="Type something..." />
                        </div>

                        <div class="flex flex-col gap-2">
                            <Label html_for="test-email">"Email"</Label>
                            <Input attr:id="test-email" attr:r#type="email" attr:placeholder="email@example.com" />
                        </div>

                        <div class="flex flex-col gap-2">
                            <Label html_for="test-textarea">"Message"</Label>
                            <Textarea attr:id="test-textarea" attr:rows="4" attr:placeholder="Write a message..." />
                        </div>
                    </form>

                    <DrawerClose>"Close"</DrawerClose>
                </DrawerBody>
            </DrawerContent>
        </Drawer>
    }
}
