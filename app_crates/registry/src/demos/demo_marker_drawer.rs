use icons::Activity;
use leptos::prelude::*;

use crate::ui::drawer::{
    Drawer, DrawerBody, DrawerClose, DrawerContent, DrawerDescription, DrawerHandle, DrawerHeader,
    DrawerTitle, DrawerTrigger,
};
use crate::ui::marker::{Marker, MarkerContent, MarkerVariant};

#[component]
pub fn DemoMarkerDrawer() -> impl IntoView {
    view! {
        <div class="flex w-full flex-col gap-2">
            <Marker variant=MarkerVariant::Separator>
                <MarkerContent>
                    <Drawer>
                        <DrawerTrigger class="inline-flex items-center gap-1.5 text-sm text-muted-foreground underline underline-offset-3 hover:text-foreground">
                            <Activity class="size-3.5" />
                            "View activity log"
                        </DrawerTrigger>

                        <DrawerContent>
                            <DrawerHandle />
                            <DrawerBody>
                                <DrawerHeader>
                                    <DrawerTitle>"File Activity Log"</DrawerTitle>
                                    <DrawerDescription>"Recent changes in this session."</DrawerDescription>
                                </DrawerHeader>
                                <ul class="flex flex-col gap-1 text-sm text-muted-foreground">
                                    <li>"Created " <code>"marker.rs"</code></li>
                                    <li>"Updated " <code>"mod.rs"</code></li>
                                    <li>"Added " <code>"demo_marker.rs"</code></li>
                                </ul>
                                <DrawerClose>"Close"</DrawerClose>
                            </DrawerBody>
                        </DrawerContent>
                    </Drawer>
                </MarkerContent>
            </Marker>
        </div>
    }
}
