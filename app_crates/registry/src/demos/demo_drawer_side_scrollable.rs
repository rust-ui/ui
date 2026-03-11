use leptos::prelude::*;

use crate::ui::drawer::{
    Drawer, DrawerBody, DrawerContent, DrawerDescription, DrawerHeader, DrawerPosition, DrawerTitle, DrawerTrigger,
};

#[component]
pub fn DemoDrawerSideScrollable() -> impl IntoView {
    view! {
        <Drawer>
            <DrawerTrigger>"Open Drawer"</DrawerTrigger>

            <DrawerContent
                position=DrawerPosition::Left
                class="top-0 bottom-0 right-auto h-full max-h-full rounded-t-none w-[300px] rounded-r-[10px]"
            >
                <DrawerBody class="overflow-y-auto pr-4 h-full">
                    <DrawerHeader>
                        <DrawerTitle>"Scrollable Side Drawer"</DrawerTitle>
                        <DrawerDescription>
                            "This drawer contains 50 scrollable items to demonstrate side scrolling behavior."
                        </DrawerDescription>
                    </DrawerHeader>

                    <div class="mt-4 space-y-2">
                        {(1..=50)
                            .map(|i| {
                                view! {
                                    <div class="p-3 rounded-md border bg-muted border-border">
                                        <p class="text-sm font-medium">"Item " {i}</p>
                                        <p class="text-xs text-muted-foreground">
                                            "This is item number " {i} " in the list"
                                        </p>
                                    </div>
                                }
                            })
                            .collect_view()}
                    </div>
                </DrawerBody>
            </DrawerContent>
        </Drawer>
    }
}
